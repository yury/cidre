//! `#[swift::call(...)]`: a declared Rust signature plus a Swift entry point,
//! expanded into the register-level call Swift's convention asks for.
//!
//! The attribute names the Swift declaration; the Rust signature says how each
//! value is represented. Those two together are everything the ABI needs: the
//! declaration gives the symbol, and the Rust types give the register classes,
//! since a binding already spells out which Rust type stands for a Swift value
//! held indirectly, a class reference, a `String`, or a primitive.
//!
//! Deliberately in the same string-templating style as `gen_msg_send`, so the
//! two generators read alike.

use proc_macro::{Delimiter, TokenStream, TokenTree};

/// What a value looks like in registers.
#[derive(Clone, Debug, PartialEq)]
enum Class {
    /// Nothing to pass or return.
    Void,
    /// One integer register holding `0` or `1`.
    Bool,
    /// One integer register.
    Word,
    /// One floating-point register, as a `double`.
    Double,
    /// One floating-point register, as a `float`.
    Float,
    /// Two integer registers, which is a `Swift.String`.
    String,
    /// The same, borrowed, so the caller keeps it.
    StringRef,
    /// One integer register holding a retained class reference, `arc::R<T>`.
    ClassRef(String),
    /// The same, nullable, which Swift represents with the null reference.
    OptClassRef(String),
    /// Returned through the indirect-result register, into storage the caller
    /// provides. The payload is the Rust wrapper naming the Swift value.
    Indirect(String),
    /// The same, wrapped in a `Swift.Optional`.
    OptIndirect(String),
    /// A borrowed pointer to a Swift value, which is what an argument of a
    /// resilient value type is.
    ValuePtr,
    /// One integer register holding a container's whole representation, which
    /// the Rust wrapper takes ownership of.
    RawWord(String),
    /// Three integer registers, which is a `CMTime`.
    Words3,
    /// A small optional Swift returns in a register rather than indirectly, so
    /// the tag is decoded through the runtime rather than assumed.
    OptPrimitive(String),
    /// A `String?`, whose two words are the string itself and whose empty case
    /// Swift spells as a null word pair.
    OptString,
    /// A run of floating-point registers, which is how a geometric value comes
    /// back rather than through an indirect result.
    Doubles(usize, String),
}

impl Class {
    /// The type whose declared class this one has to agree with, for the
    /// classes read from a name rather than from a fixed Rust primitive.
    fn declared_class(&self) -> Option<(&str, String)> {
        match self {
            Class::Indirect(ty) | Class::OptIndirect(ty) => Some((ty, "Indirect".to_string())),
            Class::RawWord(ty) => Some((ty, "Word".to_string())),
            Class::Words3 => None,
            Class::Doubles(count, ty) => Some((ty, format!("Doubles({count})"))),
            _ => None,
        }
    }

    fn is_indirect(&self) -> bool {
        matches!(self, Class::Indirect(_) | Class::OptIndirect(_))
    }
}

/// Normalizes a type as the token stream prints it, so the tables below can
/// match on it without caring how it was spaced.
///
/// How it was spaced is not the source's business either: a token stream taken
/// apart and put back together loses the joint-versus-alone spacing the
/// original carried, so the *same* type prints as `cg::Rect` where it came out
/// of an argument list and as `cg :: Rect` where it was rebuilt as a return
/// type. The punctuation that separates a path or opens a generic therefore
/// gets one spelling here, and every table is written in it.
fn normalize(ty: &str) -> String {
    let mut out = String::with_capacity(ty.len());
    let mut last_space = true;
    for ch in ty.chars() {
        if ch.is_whitespace() {
            if !last_space {
                out.push(' ');
                last_space = true;
            }
        } else {
            out.push(ch);
            last_space = false;
        }
    }

    let mut out = out.trim().to_string();
    for token in [":: ", " ::", "< ", " <", "> ", " >"] {
        let tight: String = token.chars().filter(|c| !c.is_whitespace()).collect();
        while out.contains(token) {
            out = out.replace(token, &tight);
        }
    }
    out
}

/// Strips one layer of `Name<...>`, returning the inner type.
fn inner_of<'a>(ty: &'a str, name: &str) -> Option<&'a str> {
    let head = format!("{name}<");
    let rest = ty.strip_prefix(&head)?;
    let inner = rest.strip_suffix('>')?;
    Some(inner.trim())
}

/// The last segment of a path, which is what names the type.
fn last_segment(ty: &str) -> &str {
    ty.rsplit("::").next().unwrap_or(ty).trim()
}

/// Classifies a returned Rust type.
///
/// Unknown named types are taken to be wrappers over a Swift value held
/// indirectly, which is what every framework value type these bindings name is.
/// A type that is really something else fails the check the expansion emits
/// rather than silently calling with the wrong registers.
fn classify_ret(ty: &str) -> Class {
    let ty = normalize(ty);
    if ty.is_empty() || ty == "()" {
        return Class::Void;
    }
    if let Some(inner) = inner_of(&ty, "Option") {
        let inner = normalize(inner);
        return match classify_ret(&inner) {
            Class::ClassRef(t) => Class::OptClassRef(t),
            Class::Indirect(t) => Class::OptIndirect(t),
            Class::String => Class::OptString,
            Class::Bool | Class::Word | Class::Double | Class::Float => Class::OptPrimitive(inner),
            other => panic!("swift::call: cannot return Option<{inner}> ({other:?})"),
        };
    }
    if let Some(inner) = inner_of(&ty, "Result") {
        // `Result<T, arc::R<ns::Error>>` is how a throwing call is spelled; the
        // error half is fixed, so only the success half classifies.
        let (ok, _err) =
            split_top(inner)
                .into_iter()
                .fold((String::new(), String::new()), |mut acc, part| {
                    if acc.0.is_empty() {
                        acc.0 = part;
                    } else {
                        acc.1 = part;
                    }
                    acc
                });
        return classify_ret(&ok);
    }
    if let Some(class) = container_class(&ty) {
        return class;
    }
    if ty == "cm::Time" {
        return Class::Words3;
    }
    if let Some(class) = doubles_class(&ty) {
        return class;
    }

    match last_segment(&ty) {
        "bool" => Class::Bool,
        "f64" => Class::Double,
        "f32" => Class::Float,
        "isize" | "usize" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" => {
            Class::Word
        }
        "String" => Class::String,
        _ => {
            if let Some(inner) = inner_of(&ty, "arc::R").or_else(|| inner_of(&ty, "R")) {
                Class::ClassRef(normalize(inner))
            } else {
                Class::Indirect(ty.clone())
            }
        }
    }
}

/// Whether a type is a Swift container, which is one word whatever it holds.
///
/// The Rust wrapper takes that word as its whole representation, in both
/// directions: a returned container is the word the call gives back, and one
/// passed as an argument is the word the wrapper is already holding.
fn container_class(ty: &str) -> Option<Class> {
    let bare = ty.strip_prefix("swift::").unwrap_or(ty);
    ["Array", "Set", "Dictionary"]
        .iter()
        .any(|container| bare.starts_with(&format!("{container}<")))
        .then(|| Class::RawWord(ty.to_string()))
}

/// Whether a type travels in a run of floating-point registers, and how many.
///
/// Geometry goes in `d0` up in both directions. How many registers that is
/// depends on the Swift type rather than on the Rust one — a `Vector3D` is four
/// doubles wide but travels in three — so the count is stated here, and the
/// value is taken apart and put back together through its own conversions
/// rather than by reinterpreting the bytes. The expansion pins this count to
/// the type's own, so a wrong entry is a compile error rather than a call that
/// reads a register the callee never wrote.
fn doubles_class(ty: &str) -> Option<Class> {
    let count = match ty {
        "cg::Rect" => 4,
        "spatial::Vector3D" => 3,
        "cg::Point" => 2,
        _ => return None,
    };
    Some(Class::Doubles(count, ty.to_string()))
}

/// Whether a return type is a `Result`, and so needs the error register.
fn is_throwing(ty: &str) -> bool {
    inner_of(&normalize(ty), "Result").is_some()
}

/// Classifies an argument type.
fn classify_arg(ty: &str) -> Class {
    let ty = normalize(ty);
    let borrowed = ty.starts_with('&');
    let bare = ty.strip_prefix('&').unwrap_or(&ty).trim();
    let bare = normalize(bare);
    if last_segment(&bare) == "String" {
        // A borrowed argument stays the caller's; one taken by value is
        // surrendered. Which of the two the callee wants is what the Swift
        // declaration says, and the check below makes the two agree.
        return if borrowed {
            Class::StringRef
        } else {
            Class::String
        };
    }
    if let Some(class) = doubles_class(&bare) {
        return class;
    }
    if let Some(class) = container_class(&bare) {
        return class;
    }
    match last_segment(&bare) {
        "bool" => Class::Bool,
        "f64" => Class::Double,
        "f32" => Class::Float,
        "isize" | "usize" | "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "i64" | "u64" => {
            Class::Word
        }
        "String" => Class::String,
        _ => {
            // A class-typed value *is* the reference, so it goes in one integer
            // register as itself rather than as the address of storage holding
            // it. `arc::R<T>` is what says the Rust type is that reference;
            // anything else named here is a value type passed indirectly.
            match inner_of(&bare, "arc::R").or_else(|| inner_of(&bare, "R")) {
                Some(inner) => Class::ClassRef(normalize(inner)),
                None => Class::ValuePtr,
            }
        }
    }
}

/// Splits a comma-separated list at nesting depth zero.
fn split_top(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for ch in text.chars() {
        match ch {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                parts.push(current.trim().to_string());
                current = String::new();
                continue;
            }
            _ => {}
        }
        current.push(ch);
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

struct Arg {
    name: String,
    ty: String,
}

/// The pieces of the declared Rust signature the expansion needs.
struct Signature {
    meta: String,
    vis_and_qualifiers: String,
    name: String,
    generics: String,
    args_source: String,
    ret_source: String,
    takes_self: bool,
    args: Vec<Arg>,
}

fn parse_signature(func: TokenStream) -> Signature {
    let mut iter = func.into_iter();
    let mut meta: Vec<TokenTree> = Vec::new();
    let mut qualifiers: Vec<TokenTree> = Vec::new();

    // Everything up to `fn` is either an outer attribute or a qualifier.
    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                meta.push(tt);
                if let Some(next @ TokenTree::Group(_)) = iter.next() {
                    meta.push(next);
                }
            }
            TokenTree::Ident(i) if i.to_string() == "fn" => break,
            _ => qualifiers.push(tt),
        }
    }

    let Some(TokenTree::Ident(name)) = iter.next() else {
        panic!("swift::call: expected a function name");
    };
    let name = name.to_string();

    let mut generics = Vec::new();
    let args = loop {
        match iter.next() {
            Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Parenthesis => break g,
            Some(tt) => generics.push(tt),
            None => panic!("swift::call: expected an argument list"),
        }
    };

    let mut ret = TokenStream::from_iter(iter).to_string();
    let trimmed = ret.trim_end();
    assert!(
        trimmed.ends_with(';'),
        "swift::call: the function must be a declaration ending in `;`"
    );
    ret = trimmed[..trimmed.len() - 1].to_string();
    let ret_source = ret
        .trim()
        .strip_prefix("->")
        .map(|r| r.trim().to_string())
        .unwrap_or_default();

    let args_source = args.to_string();
    let mut takes_self = false;
    let mut parsed = Vec::new();
    for part in split_top(&normalize(args.stream().to_string().as_str())) {
        if part.ends_with("self") {
            takes_self = true;
            continue;
        }
        let Some((name, ty)) = part.split_once(':') else {
            panic!("swift::call: argument `{part}` needs a type");
        };
        parsed.push(Arg {
            name: name.trim().to_string(),
            ty: ty.trim().to_string(),
        });
    }

    Signature {
        meta: TokenStream::from_iter(meta).to_string(),
        vis_and_qualifiers: TokenStream::from_iter(qualifiers).to_string(),
        name,
        generics: TokenStream::from_iter(generics).to_string(),
        args_source,
        ret_source,
        takes_self,
        args: parsed,
    }
}

/// One `asm!` operand.
struct Operand {
    reg: String,
    input: Option<String>,
    output: Option<String>,
}

impl Operand {
    fn render(&self) -> String {
        match (&self.input, &self.output) {
            (Some(i), Some(o)) => format!("inlateout(\"{}\") {i} => {o},", self.reg),
            (Some(i), None) => format!("in(\"{}\") {i},", self.reg),
            (None, Some(o)) => format!("lateout(\"{}\") {o},", self.reg),
            (None, None) => String::new(),
        }
    }
}

/// The Swift entry point named by the attribute.
enum Symbol {
    /// A mangled symbol given as-is.
    Mangled(String),
    /// A Swift declaration to mangle.
    Decl(String),
}

/// What the attribute said, beyond naming the entry point.
struct Attr {
    symbol: Symbol,
    /// Whether the call suspends.
    ///
    /// A declaration says so itself, so this is only read from the attribute
    /// for `sym = "$s..."`, where there is nothing left to read it off: `Ya`
    /// cannot be looked for in the mangled string, since an identifier like
    /// `faceYawAngle` contains it too.
    is_async: bool,
    /// Parameters the callee takes at `+1`, by name.
    ///
    /// Also only for `sym = "$s..."`. A declaration carries each parameter's
    /// convention and is authoritative; a bare symbol carries nothing, and
    /// getting this wrong leaks or over-releases with no compile error either
    /// way, so it is stated rather than assumed.
    owned: Vec<String>,
}

/// Splits the attribute on its top-level commas and reads each piece.
///
/// Over the token tree rather than the printed text: a declaration is a single
/// string literal whose own commas must not split it, and only the tree knows
/// where a literal ends.
fn parse_attr(attr: TokenStream) -> Attr {
    let mut segments: Vec<Vec<TokenTree>> = vec![Vec::new()];
    for tt in attr {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == ',' => segments.push(Vec::new()),
            _ => segments
                .last_mut()
                .expect("there is always a segment open")
                .push(tt),
        }
    }

    let mut symbol = None;
    let mut is_async = false;
    let mut owned = Vec::new();

    for segment in segments {
        let Some(head) = segment.first() else {
            continue;
        };
        match head.to_string().as_str() {
            "async" => is_async = true,
            "owned" => {
                let Some(TokenTree::Group(group)) = segment.get(1) else {
                    panic!(
                        "swift::call: `owned` names the parameters the callee \
                         takes at `+1`, as in `owned(asset)`"
                    );
                };
                owned.extend(group.stream().into_iter().filter_map(|tt| match tt {
                    TokenTree::Ident(ident) => Some(ident.to_string()),
                    _ => None,
                }));
            }
            "sym" => {
                let literal = segment
                    .last()
                    .expect("`sym` is followed by the symbol")
                    .to_string();
                symbol = Some(Symbol::Mangled(unescape(&literal)));
            }
            // Anything else is the declaration itself, written as one literal.
            other => symbol = Some(Symbol::Decl(unescape(other))),
        }
    }

    Attr {
        symbol: symbol.expect("swift::call: expected a declaration or `sym = \"$s...\"`"),
        is_async,
        owned,
    }
}

/// Expands `kind, "Module.Type"` into the address of that type's metadata
/// accessor, declaring the symbol it links against along the way.
///
/// A nominal type's symbol is its context and nothing else, so unlike a member
/// it needs none of the substitution bookkeeping a function signature does and
/// can always be derived from the name.
pub fn gen_metadata_accessor(args: TokenStream) -> TokenStream {
    let text = args.to_string();
    let (kind, path) = text
        .split_once(',')
        .unwrap_or_else(|| panic!("swift::metadata_accessor: expected `kind, \"Module.Type\"`"));
    let kind = crate::swift_mangle::kind_letter(kind.trim())
        .unwrap_or_else(|e| panic!("swift::metadata_accessor: {e}"));
    let path = unescape(path.trim());
    let path = path.as_str();

    let symbol = crate::swift_mangle::mangle_metadata_accessor(path, kind)
        .unwrap_or_else(|e| panic!("swift::metadata_accessor: cannot mangle `{path}`: {e}"));

    format!(
        "{{
            unsafe extern \"C\" {{
                #[link_name = \"{symbol}\"]
                fn __swift_metadata_accessor();
            }}
            __swift_metadata_accessor as *const ()
        }}"
    )
    .parse()
    .expect("valid accessor expression")
}

/// Resolves what a string literal's token spells into the string it denotes.
///
/// A proc macro is handed the literal's *source* text, so a declaration written
/// across two lines arrives with its continuation backslash and indentation
/// still in it — which a mangler would happily count into an identifier's
/// length prefix and produce a symbol that links to nothing.
fn unescape(literal: &str) -> String {
    let body = literal
        .strip_prefix('"')
        .and_then(|l| l.strip_suffix('"'))
        .unwrap_or(literal);

    let chars: Vec<char> = body.chars().collect();
    let mut out = String::with_capacity(body.len());
    let mut index = 0;
    while index < chars.len() {
        if chars[index] != '\\' {
            out.push(chars[index]);
            index += 1;
            continue;
        }
        index += 1;
        match chars.get(index) {
            // A line continuation eats the newline and the indentation after it.
            Some('\n') => {
                index += 1;
                while matches!(chars.get(index), Some(c) if c.is_whitespace()) {
                    index += 1;
                }
            }
            Some('n') => {
                out.push('\n');
                index += 1;
            }
            Some('t') => {
                out.push('\t');
                index += 1;
            }
            Some('r') => {
                out.push('\r');
                index += 1;
            }
            Some(other) => {
                out.push(*other);
                index += 1;
            }
            None => break,
        }
    }
    out
}

/// Expands a Swift declaration into the address of the entry point it names.
///
/// The pointer-valued sibling of `#[swift::call]`, for the places that hand a
/// Swift entry point to something else rather than calling it themselves.
pub fn gen_symbol(args: TokenStream) -> TokenStream {
    let text = args.to_string();
    let decl = text.trim();
    let decl = unescape(decl);
    let decl = decl.as_str();

    let symbol = crate::swift_mangle::mangle(decl)
        .unwrap_or_else(|e| panic!("swift::symbol: cannot mangle `{decl}`: {e}"));

    format!(
        "{{
            unsafe extern \"C\" {{
                #[link_name = \"{symbol}\"]
                fn __swift_symbol();
            }}
            __swift_symbol as *const ()
        }}"
    )
    .parse()
    .expect("valid symbol expression")
}

pub fn gen_swift_call(attr: TokenStream, func: TokenStream) -> TokenStream {
    let attr = parse_attr(attr);
    let (link_name, alias, conventions, is_async) = match attr.symbol {
        // Without a declaration there is nothing to read a convention off, so
        // every argument is taken to be borrowed unless `owned` says otherwise,
        // borrowing being a method's default.
        Symbol::Mangled(s) => (s, None, Vec::new(), attr.is_async),
        Symbol::Decl(d) => {
            let mangled = crate::swift_mangle::mangle(&d)
                .unwrap_or_else(|e| panic!("swift::call: cannot mangle `{d}`: {e}"));
            let conventions = crate::swift_mangle::param_conventions(&d)
                .unwrap_or_else(|e| panic!("swift::call: cannot read `{d}`: {e}"));
            let is_async = crate::swift_mangle::is_async(&d)
                .unwrap_or_else(|e| panic!("swift::call: cannot read `{d}`: {e}"));
            (mangled, Some(d), conventions, is_async)
        }
    };

    assert!(
        attr.owned.is_empty() || alias.is_none(),
        "swift::call: the declaration already says what the callee takes at \
         `+1`, so `owned` would only be a second answer to the same question"
    );

    let sig = parse_signature(func);

    // A suspending function cannot be called the way a direct one is: the
    // caller has to allocate its context and hand it a resume pointer, which is
    // what the shared trampolines do, so the expansion is a different shape
    // entirely.
    if is_async {
        let async_fn = format!("{link_name}Tu");
        return gen_async_call(
            sig,
            &link_name,
            &async_fn,
            &alias,
            &conventions,
            &attr.owned,
        );
    }
    let ret_class = classify_ret(&sig.ret_source);
    let throws = is_throwing(&sig.ret_source);

    // Argument setup, in declaration order: integer-class values fill x0 up,
    // floating-point ones d0 up, and `self` always goes in the context
    // register whatever else the call takes.
    let mut int_args: Vec<String> = Vec::new();
    let mut float_args: Vec<String> = Vec::new();
    // What each floating-point operand is declared as, which the thunk below
    // needs in order to name the parameter that lands in the register.
    let mut float_tys: Vec<&str> = Vec::new();
    let mut prelude = String::new();

    for (index, arg) in sig.args.iter().enumerate() {
        match classify_arg(&arg.ty) {
            Class::Bool => int_args.push(format!("{} as usize", arg.name)),
            Class::Word => int_args.push(format!("{} as usize", arg.name)),
            Class::Double => {
                float_args.push(format!("{} as f64", arg.name));
                float_tys.push("f64");
            }
            Class::Float => {
                float_args.push(format!("{} as f32", arg.name));
                float_tys.push("f32");
            }
            class @ (Class::String | Class::StringRef) => {
                let by_value = class == Class::String;
                check_ownership(&alias, &conventions, index, &arg.name, by_value);
                let raw = format!("__raw_{}", arg.name);
                let take = if by_value {
                    format!("crate::swift::String::into_raw({})", arg.name)
                } else {
                    format!("crate::swift::String::as_raw({})", arg.name)
                };
                prelude.push_str(&format!("let {raw} = {take};\n"));
                int_args.push(format!("{raw}.word0"));
                int_args.push(format!("{raw}.word1"));
            }
            Class::ValuePtr => int_args.push(format!(
                "crate::swift::SwiftSelf::swift_self_ptr({}) as usize",
                arg.name
            )),
            other => panic!("swift::call: argument `{}` is {other:?}", arg.name),
        }
    }

    let self_operand = if sig.takes_self {
        Some("crate::swift::SwiftSelf::swift_self_ptr(self) as usize".to_string())
    } else {
        // A static member takes the type's metadata where an instance member
        // takes the instance.
        Some("<Self as crate::swift::SwiftMetadata>::metadata() as usize".to_string())
    };

    // Results.
    let mut outs: Vec<(String, String)> = Vec::new(); // (register, binding)
    let mut bindings = String::new();
    let mut tail;

    // The register class was read off the Rust type's *name*; the type itself
    // states the truth. Pinning one to the other turns a misread name from a
    // call through the wrong registers into a compile error.
    let mut checks = String::new();
    if let Some((ty, class)) = ret_class.declared_class() {
        // An inline `const` block rather than a `const` item, so `Self` still
        // names the type the call is written on.
        checks.push_str(&format!(
            "const {{
            assert!(
                <{ty} as crate::swift::SwiftAbi>::CLASS.tag()
                    == crate::swift::AbiClass::{class}.tag(),
                \"swift::call: this type is not returned the way the call assumes\"
            )
        }};\n"
        ));
    }

    match &ret_class {
        Class::Void => {
            tail = "()".to_string();
        }
        Class::Bool => {
            bindings.push_str("let __r0: usize;\n");
            outs.push(("x0".into(), "__r0".into()));
            tail = "__r0 & 1 != 0".to_string();
        }
        Class::Word => {
            bindings.push_str("let __r0: usize;\n");
            outs.push(("x0".into(), "__r0".into()));
            tail = format!("__r0 as {}", ret_ok_type(&sig.ret_source));
        }
        Class::Double => {
            bindings.push_str("let __d0: f64;\n");
            outs.push(("d0".into(), "__d0".into()));
            tail = "__d0".to_string();
        }
        Class::Float => {
            bindings.push_str("let __s0: f32;\n");
            outs.push(("s0".into(), "__s0".into()));
            tail = "__s0".to_string();
        }
        Class::String => {
            bindings.push_str("let (__r0, __r1): (usize, usize);\n");
            outs.push(("x0".into(), "__r0".into()));
            outs.push(("x1".into(), "__r1".into()));
            tail =
                "crate::swift::String::from_raw(crate::swift::RawString { word0: __r0, word1: __r1 })"
                    .to_string();
        }
        Class::ClassRef(_) | Class::OptClassRef(_) => {
            bindings.push_str("let __r0: usize;\n");
            outs.push(("x0".into(), "__r0".into()));
            let ok = ret_ok_type(&sig.ret_source);
            tail = if matches!(ret_class, Class::OptClassRef(_)) {
                format!(
                    "if __r0 == 0 {{ None }} else {{ Some(crate::arc::R::from_raw(__r0 as *mut _)) }} as {ok}"
                )
            } else {
                "crate::arc::R::from_raw(__r0 as *mut _)".to_string()
            };
        }
        Class::RawWord(ty) => {
            bindings.push_str("let __r0: usize;\n");
            outs.push(("x0".into(), "__r0".into()));
            tail = format!("<{ty}>::from_raw(__r0 as *mut ())");
        }
        Class::Words3 => {
            bindings.push_str("let (__r0, __r1, __r2): (u64, u64, u64);\n");
            outs.push(("x0".into(), "__r0".into()));
            outs.push(("x1".into(), "__r1".into()));
            outs.push(("x2".into(), "__r2".into()));
            tail = format!(
                "core::mem::transmute::<(u64, u64, u64), {}>((__r0, __r1, __r2))",
                ret_ok_type(&sig.ret_source)
            );
        }
        Class::OptPrimitive(ty) => {
            // The word holds both the payload and the tag, and only the type's
            // own witnesses know where the tag sits, so it is read back through
            // them rather than by assuming a niche.
            bindings.push_str("let __r0: usize;\n");
            outs.push(("x0".into(), "__r0".into()));
            tail = format!(
                "{{
            let mut __opt = crate::swift::value::Storage::<crate::swift::value::Optional<{ty}>>::new();
            crate::swift::value::Storage::as_mut_ptr(&mut __opt).cast::<usize>().write(__r0);
            __opt.take()
        }}"
            );
        }
        Class::OptString => {
            bindings.push_str("let (__r0, __r1): (usize, usize);\n");
            outs.push(("x0".into(), "__r0".into()));
            outs.push(("x1".into(), "__r1".into()));
            tail = "(__r0 != 0 || __r1 != 0).then(|| \
                crate::swift::String::from_raw(crate::swift::RawString { word0: __r0, word1: __r1 }))"
                .to_string();
        }
        Class::Doubles(count, ty) => {
            let names: Vec<String> = (0..*count).map(|i| format!("__d{i}")).collect();
            bindings.push_str(&format!(
                "let ({}): ({});\n",
                names.join(", "),
                vec!["f64"; *count].join(", ")
            ));
            for (index, name) in names.iter().enumerate() {
                outs.push((format!("d{index}"), name.clone()));
            }
            tail = format!(
                "<{ty} as crate::swift::FromSwiftDoubles>::from_doubles(&[{}])",
                names.join(", ")
            );
        }
        Class::Indirect(ty) => {
            prelude.push_str(&format!(
                "let mut __out = <{ty} as crate::swift::value::SwiftOut>::out_buf();\n"
            ));
            tail = format!("<{ty} as crate::swift::value::SwiftOut>::out_take(__out)");
        }
        Class::OptIndirect(ty) => {
            // A wrapper may hold the payload or the optional itself, so which
            // of the two the storage becomes is left to the wrapper.
            prelude.push_str(&format!(
                "let mut __out = crate::swift::value::Storage::<crate::swift::value::Optional<<{ty} as crate::swift::value::SwiftOptionalValue>::Marker>>::new();\n"
            ));
            tail = format!(
                "<{ty} as crate::swift::value::SwiftOptionalValue>::from_optional_storage(__out)"
            );
        }
        other => panic!("swift::call: cannot return {other:?}"),
    }

    if throws {
        bindings.push_str("let __error: *mut ();\n");
        tail = format!(
            "if __error.is_null() {{ Ok({tail}) }} else {{ Err(crate::arc::R::from_raw(crate::swift::abi::error_as_ns_error(__error).cast())) }}"
        );
    }

    // A naked `extern "C"` thunk beats an `asm!` block here: `clobber_abi`
    // cannot say a Swift callee keeps the low half of `v8`-`v15`, so it marks
    // all sixteen and every caller spills `d8`-`d15`. Behind a C call the
    // register mask applies instead.
    //
    // Falls back to assembly for what a thunk cannot carry: the error register,
    // a three-word return, and calls too wide to spare an argument register.
    let indirect_slot = usize::from(ret_class.is_indirect());
    let self_slot = 1;
    let use_thunk = !throws
        && !matches!(ret_class, Class::Words3)
        && int_args.len() + indirect_slot + self_slot <= 8
        && float_args.len() <= 8;

    if use_thunk {
        return gen_thunk_call(
            &sig,
            &link_name,
            &alias,
            &ret_class,
            &int_args,
            &float_args,
            &float_tys,
            indirect_slot,
            &self_operand.expect("a call always names a self operand"),
            &checks,
            &prelude,
            &tail,
        );
    }

    // Register assignment. Arguments claim x0 up and d0 up; a returned value
    // claims the same registers back, so the two meet as `inlateout` wherever
    // they overlap.
    let mut operands: Vec<Operand> = Vec::new();
    let int_ret: Vec<&(String, String)> = outs.iter().filter(|(r, _)| r.starts_with('x')).collect();
    let float_ret: Vec<&(String, String)> = outs
        .iter()
        .filter(|(r, _)| r.starts_with('d') || r.starts_with('s'))
        .collect();

    let int_used = int_args.len().max(int_ret.len());
    for index in 0..int_used {
        let reg = format!("x{index}");
        let input = int_args.get(index).cloned();
        let output = int_ret.get(index).map(|(_, binding)| binding.clone());
        operands.push(Operand { reg, input, output });
    }

    let float_used = float_args.len().max(float_ret.len());
    for index in 0..float_used {
        let input = float_args.get(index).cloned();
        let output = float_ret.get(index).map(|(reg, binding)| {
            // A `float` result comes back in the low half of the same register.
            (reg.clone(), binding.clone())
        });
        let reg = match &output {
            Some((reg, _)) => reg
                .replace(['d', 's'], "")
                .parse::<usize>()
                .ok()
                .map_or_else(|| format!("d{index}"), |_| format!("{}{index}", &reg[..1])),
            None => format!("d{index}"),
        };
        operands.push(Operand {
            reg,
            input,
            output: output.map(|(_, binding)| binding),
        });
    }

    if ret_class.is_indirect() {
        operands.push(Operand {
            reg: "x8".into(),
            // Where the buffer's address comes from depends on which kind of
            // buffer the return type asked for.
            input: Some(match &ret_class {
                Class::Indirect(ty) => {
                    format!("<{ty} as crate::swift::value::SwiftOut>::out_ptr(&mut __out) as usize")
                }
                _ => "crate::swift::value::Storage::as_mut_ptr(&mut __out) as usize".to_string(),
            }),
            output: None,
        });
    }

    if let Some(self_expr) = self_operand {
        operands.push(Operand {
            reg: "x20".into(),
            input: Some(self_expr),
            output: None,
        });
    }

    if throws {
        operands.push(Operand {
            reg: "x21".into(),
            input: Some("0usize".into()),
            output: Some("__error".into()),
        });
    }

    let operand_list: String = operands
        .iter()
        .map(|o| o.render())
        .collect::<Vec<_>>()
        .join("\n            ");

    let doc_alias = alias
        .map(|a| format!("#[doc(alias = \"{a}\")]"))
        .unwrap_or_default();

    let Signature {
        meta,
        vis_and_qualifiers,
        name,
        generics,
        args_source,
        ret_source,
        ..
    } = sig;

    let ret_clause = if ret_source.is_empty() {
        String::new()
    } else {
        format!("-> {ret_source}")
    };

    let out = format!(
        "
{meta}
{doc_alias}
#[inline]
{vis_and_qualifiers} fn {name}{generics}{args_source} {ret_clause} {{
    #[allow(non_snake_case)]
    unsafe extern \"C\" {{
        #[link_name = \"{link_name}\"]
        fn __swift_callee();
    }}
    unsafe {{
        {checks}
        let __fn = __swift_callee as *const ();
        {prelude}
        {bindings}
        core::arch::asm!(
            \"blr {{__fn}}\",
            __fn = in(reg) __fn,
            {operand_list}
            clobber_abi(\"C\"),
        );
        {tail}
    }}
}}
"
    );

    out.parse()
        .unwrap_or_else(|e| panic!("swift::call generated invalid code: {e}\n{out}"))
}

/// The success half of a `Result<T, arc::R<ns::Error>>`, which is what every
/// suspending binding returns.
fn async_ok_type(ret: &str) -> String {
    let ret = normalize(ret);
    let inner = inner_of(&ret, "Result").unwrap_or_else(|| {
        panic!(
            "swift::call: a suspending call must return \
             `Result<T, arc::R<ns::Error>>`, not `{ret}`"
        )
    });
    split_top(inner)
        .into_iter()
        .next()
        .expect("a Result names a success type")
}

/// Rebuilds an argument list with one more parameter on the end.
fn with_trailing_arg(args_source: &str, extra: &str) -> String {
    let trimmed = args_source.trim();
    let inner = trimmed
        .strip_prefix('(')
        .and_then(|a| a.strip_suffix(')'))
        .expect("an argument list is parenthesized")
        .trim();
    // A declaration may have a trailing comma, and appending after one would
    // leave an empty parameter between the two.
    let inner = inner.strip_suffix(',').unwrap_or(inner).trim_end();
    if inner.is_empty() {
        format!("({extra})")
    } else {
        format!("({inner}, {extra})")
    }
}

/// Expands a suspending Swift call into the pair of entry points every async
/// binding is written as: one taking a completion handler, and one returning a
/// future.
///
/// Unlike a direct call there are no registers to name here. A suspending call
/// runs on a Swift task through the shared trampolines, which read the
/// arguments out of a fixed-layout struct, so what this generates is the
/// *contents* of that struct: which register each value goes in, and what has
/// to stay alive until the call resumes.
fn gen_async_call(
    sig: Signature,
    link_name: &str,
    async_fn: &str,
    alias: &Option<String>,
    conventions: &[bool],
    owned_params: &[String],
) -> TokenStream {
    assert!(
        sig.generics.trim().is_empty(),
        "swift::call: a suspending call cannot be generic"
    );

    for name in owned_params {
        assert!(
            sig.args.iter().any(|arg| arg.name == *name),
            "swift::call: `owned` names `{name}`, which is not a parameter"
        );
    }

    let ret_class = classify_ret(&sig.ret_source);
    // Only for the check it makes: every suspending binding throws, and a
    // return type that is not a `Result` would silently drop the error.
    async_ok_type(&sig.ret_source);

    // Everything the call borrows has to outlive it, and the caller's frame
    // does not, so those values are moved into a tuple the task owns. The
    // closure that fills the argument registers is handed that tuple by
    // reference, which is what makes a pointer into it valid for the whole call.
    let mut owned: Vec<String> = Vec::new();
    let mut owned_pat: Vec<String> = Vec::new();
    let mut setters: Vec<String> = Vec::new();
    // What the argument closure has to work out before it can fill a register.
    let mut closure_prelude = String::new();

    // `self` is the instance for a method and the type's metadata for a static
    // one, exactly as a direct call passes it.
    if sig.takes_self {
        owned.push("crate::arc::Retain::retained(self)".to_string());
        owned_pat.push("__self".to_string());
        setters.push(".swift_self(__self.as_ptr().cast())".to_string());
    } else {
        setters.push(
            ".swift_self(<Self as crate::swift::SwiftMetadata>::metadata().cast_mut().cast())"
                .to_string(),
        );
    }

    // A result returned indirectly is written into storage the caller provides,
    // and for a suspending call that storage is the first argument rather than
    // the indirect-result register a direct call would use.
    let mut int_index = 0usize;
    let mut float_index = 0usize;
    let out_slot = matches!(ret_class, Class::Indirect(_)).then(|| {
        let slot = owned.len();
        let Class::Indirect(ty) = &ret_class else {
            unreachable!()
        };
        owned.push(format!(
            "<{ty} as crate::swift::value::SwiftOut>::out_buf()"
        ));
        owned_pat.push("__out".to_string());
        setters.push(format!(
            ".arg(0, <{ty} as crate::swift::value::SwiftOut>::out_ptr(__out))"
        ));
        int_index += 1;
        slot
    });

    for (index, arg) in sig.args.iter().enumerate() {
        let name = &arg.name;
        let consumed = conventions
            .get(index)
            .copied()
            .unwrap_or_else(|| owned_params.iter().any(|owned| *owned == arg.name));
        match classify_arg(&arg.ty) {
            // Scalars are copied into the task's own storage, so the caller's
            // copy going away is nothing to the call.
            Class::Bool | Class::Word => {
                setters.push(format!(".arg({int_index}, {name} as usize as *mut ())"));
                int_index += 1;
            }
            Class::Double => {
                setters.push(format!(".float({float_index}, {name} as f64)"));
                float_index += 1;
            }
            Class::Float => panic!(
                "swift::call: `{name}` is an `f32`, which a suspending call has \
                 no way to place yet"
            ),
            // The reference itself, in one integer register. It has to be the
            // task's rather than the caller's, so a class argument is written
            // `arc::R<T>` and handed over by value; whether Swift then wants it
            // at +1 is the declaration's business, not the Rust type's.
            Class::ClassRef(_) => {
                assert!(
                    !arg.ty.trim().starts_with('&'),
                    "swift::call: `{name}` is borrowed, but a suspending call \
                     outlives the caller's frame, so a class argument has to be \
                     taken by value as `arc::R<_>`"
                );
                let slot = owned.len();
                owned.push(name.clone());
                owned_pat.push(format!("__a{slot}"));
                setters.push(if consumed {
                    // Consumed, so the callee gets a reference of its own and
                    // the one the task holds stays intact.
                    format!(
                        ".arg({int_index}, crate::arc::Retain::retained(&**__a{slot}).into_raw().cast())"
                    )
                } else {
                    format!(".arg({int_index}, __a{slot}.as_ptr().cast())")
                });
                int_index += 1;
            }
            // One word, which the container is already holding. The wrapper
            // owns that word, so like a value passed indirectly it moves into
            // the task rather than being read out of the caller's frame.
            Class::RawWord(ty) => {
                assert!(
                    !arg.ty.trim().starts_with('&'),
                    "swift::call: `{name}` is borrowed, but a suspending call \
                     outlives the caller's frame, so it has to be taken by value"
                );
                let slot = owned.len();
                owned.push(name.clone());
                owned_pat.push(format!("__a{slot}"));
                closure_prelude.push_str(&format!(
                    "const {{
                        assert!(
                            <{ty} as crate::swift::SwiftAbi>::CLASS.tag()
                                == crate::swift::AbiClass::Word.tag(),
                            \"swift::call: this type is not passed the way the call assumes\"
                        )
                    }};\n                    "
                ));
                setters.push(format!(".arg({int_index}, __a{slot}.as_raw())"));
                int_index += 1;
            }
            // Several registers from the one value, taken apart through the
            // type's own conversion. The count the table states is pinned to
            // the type's here, so an entry that is wrong about how wide a value
            // travels fails to build rather than leaving a register unwritten.
            Class::Doubles(count, ty) => {
                closure_prelude.push_str(&format!(
                    "const {{
                        assert!(
                            <{ty} as crate::swift::ToSwiftDoubles>::COUNT == {count},
                            \"swift::call: this type does not travel in the registers the call assumes\"
                        )
                    }};
                    let mut __fd{index} = [0f64; {count}];
                    crate::swift::ToSwiftDoubles::write_doubles(&{name}, &mut __fd{index});\n                    "
                ));
                for offset in 0..count {
                    setters.push(format!(
                        ".float({}, __fd{index}[{offset}])",
                        float_index + offset
                    ));
                }
                float_index += count;
            }
            // A value held indirectly is passed as a pointer to itself, so the
            // value has to be the task's rather than the caller's.
            Class::ValuePtr => {
                assert!(
                    !arg.ty.trim().starts_with('&'),
                    "swift::call: `{name}` is borrowed, but a suspending call \
                     outlives the caller's frame, so it has to be taken by value"
                );
                assert!(
                    !consumed,
                    "swift::call: `{name}` is consumed by the callee, which a \
                     suspending call cannot hand over yet"
                );
                let slot = owned.len();
                owned.push(name.clone());
                owned_pat.push(format!("__a{slot}"));
                setters.push(format!(
                    ".arg({int_index}, crate::swift::SwiftSelf::swift_self_ptr(__a{slot}).cast_mut())"
                ));
                int_index += 1;
            }
            other => panic!("swift::call: argument `{name}` is {other:?}"),
        }
    }

    // What the resume trampoline hands back, turned into the success value.
    let output = match &ret_class {
        Class::Void => "|_, _| ()".to_string(),
        Class::ClassRef(_) => "|_, __result| crate::arc::R::from_raw(__result.cast())".to_string(),
        Class::Indirect(ty) => {
            let slot = out_slot.expect("an indirect return owns its buffer");
            let take: Vec<String> = (0..owned.len())
                .map(|index| {
                    if index == slot {
                        "__out".to_string()
                    } else {
                        "_".to_string()
                    }
                })
                .collect();
            format!(
                "|__owned, _| {{ let ({},) = __owned; \
                 <{ty} as crate::swift::value::SwiftOut>::out_take(__out) }}",
                take.join(", ")
            )
        }
        other => panic!("swift::call: a suspending call cannot return {other:?}"),
    };

    // The register class was read off the Rust type's name; the type itself
    // states the truth, so the two are pinned together as a direct call's are.
    let checks = match ret_class.declared_class() {
        Some((ty, class)) => format!(
            "const {{
            assert!(
                <{ty} as crate::swift::SwiftAbi>::CLASS.tag()
                    == crate::swift::AbiClass::{class}.tag(),
                \"swift::call: this type is not returned the way the call assumes\"
            )
        }};"
        ),
        None => String::new(),
    };

    let (owned_tuple, owned_pattern) = if owned.is_empty() {
        ("()".to_string(), "()".to_string())
    } else {
        (
            format!("({},)", owned.join(", ")),
            format!("({},)", owned_pat.join(", ")),
        )
    };
    let setters = setters.join("\n                    ");

    let doc_alias = alias
        .as_ref()
        .map(|a| format!("#[doc(alias = \"{a}\")]"))
        .unwrap_or_default();

    let Signature {
        meta,
        vis_and_qualifiers,
        name,
        args_source,
        ret_source,
        ..
    } = &sig;

    let handler_args = with_trailing_arg(args_source, "__callback: __F");

    let out = format!(
        "
{meta}
{doc_alias}
#[inline]
{vis_and_qualifiers} fn {name}_handler<__F>{handler_args}
where
    __F: FnOnce({ret_source}) + Send + 'static,
{{
    #[allow(non_snake_case)]
    unsafe extern \"C\" {{
        #[link_name = \"{link_name}\"]
        fn __swift_callee();

        #[link_name = \"{async_fn}\"]
        static __SWIFT_ASYNC_FN: u8;
    }}
    unsafe {{
        {checks}
        crate::swift::concurrency::call_async_result(
            __swift_callee as *const (),
            &raw const __SWIFT_ASYNC_FN,
            {owned_tuple},
            |{owned_pattern}| {{
                    {closure_prelude}crate::swift::concurrency::AsyncCallArgs::new()
                    {setters}
            }},
            {output},
            __callback,
        );
    }}
}}

{meta}
{doc_alias}
#[cfg(feature = \"async\")]
#[inline]
{vis_and_qualifiers} fn {name}{args_source} -> impl core::future::Future<Output = {ret_source}> {{
    #[allow(non_snake_case)]
    unsafe extern \"C\" {{
        #[link_name = \"{link_name}\"]
        fn __swift_callee();

        #[link_name = \"{async_fn}\"]
        static __SWIFT_ASYNC_FN: u8;
    }}
    unsafe {{
        {checks}
        crate::swift::concurrency::call_async_future(
            __swift_callee as *const (),
            &raw const __SWIFT_ASYNC_FN,
            {owned_tuple},
            |{owned_pattern}| {{
                    {closure_prelude}crate::swift::concurrency::AsyncCallArgs::new()
                    {setters}
            }},
            {output},
        )
    }}
}}
"
    );

    out.parse()
        .unwrap_or_else(|e| panic!("swift::call generated invalid code: {e}\n{out}"))
}

/// Expands the call as a plain C call to a naked thunk that tail-calls Swift.
///
/// The thunk places the two operands C cannot name — the context register and
/// the indirect-result register — and restores the first afterwards.
#[allow(clippy::too_many_arguments)]
fn gen_thunk_call(
    sig: &Signature,
    link_name: &str,
    alias: &Option<String>,
    ret_class: &Class,
    int_args: &[String],
    float_args: &[String],
    float_tys: &[&str],
    indirect_slot: usize,
    self_operand: &str,
    checks: &str,
    prelude: &str,
    tail: &str,
) -> TokenStream {
    // Integer parameters fill x0 up and floating-point ones d0 up, each in
    // declaration order, so the thunk groups them that way.
    let mut params: Vec<String> = (0..int_args.len())
        .map(|index| format!("__a{index}: usize"))
        .collect();
    let mut call_args: Vec<String> = int_args.to_vec();

    if indirect_slot == 1 {
        params.push("__out_ptr: *mut ()".to_string());
        call_args.push(match ret_class {
            Class::Indirect(ty) => {
                format!("<{ty} as crate::swift::value::SwiftOut>::out_ptr(&mut __out)")
            }
            _ => "crate::swift::value::Storage::as_mut_ptr(&mut __out)".to_string(),
        });
    }

    params.push("__self: usize".to_string());
    call_args.push(self_operand.to_string());

    for (index, ty) in float_tys.iter().enumerate() {
        params.push(format!("__f{index}: {ty}"));
    }
    call_args.extend(float_args.iter().cloned());

    // `x20` is callee-saved under C, so the thunk must restore it: the callee
    // gives back what the thunk left, not what the caller had. That rules out a
    // tail call and costs the stack slot. The pair also saves the link register.
    let mut shuffle = String::from("\"stp x20, x30, [sp, #-16]!\",\n            ");
    if indirect_slot == 1 {
        shuffle.push_str(&format!("\"mov x8, x{}\",\n            ", int_args.len()));
    }
    shuffle.push_str(&format!(
        "\"mov x20, x{}\",\n            ",
        int_args.len() + indirect_slot
    ));

    // Named as a Rust return type so the compiler places the registers; the
    // bindings match what the shared tail expression reads.
    let (thunk_ret, thunk_item, bind) = match ret_class {
        Class::Void | Class::Indirect(_) | Class::OptIndirect(_) => {
            (String::new(), String::new(), "__CALL__;".to_string())
        }
        Class::Bool
        | Class::Word
        | Class::ClassRef(_)
        | Class::OptClassRef(_)
        | Class::RawWord(_)
        | Class::OptPrimitive(_) => (
            "-> usize".to_string(),
            String::new(),
            "let __r0: usize = __CALL__;".to_string(),
        ),
        Class::Double => (
            "-> f64".to_string(),
            String::new(),
            "let __d0: f64 = __CALL__;".to_string(),
        ),
        Class::Float => (
            "-> f32".to_string(),
            String::new(),
            "let __s0: f32 = __CALL__;".to_string(),
        ),
        // Two words, which is the same pair `RawString` already is.
        Class::String | Class::OptString => (
            "-> crate::swift::RawString".to_string(),
            String::new(),
            "let __rs = __CALL__;\nlet (__r0, __r1) = (__rs.word0, __rs.word1);".to_string(),
        ),
        // A homogeneous float aggregate, which is what puts it in `d0` up.
        Class::Doubles(count, _) => {
            let fields = vec!["f64"; *count].join(", ");
            let names: Vec<String> = (0..*count).map(|i| format!("__d{i}")).collect();
            let values: Vec<String> = (0..*count).map(|i| format!("__ds.{i}")).collect();
            (
                "-> __SwiftDoubles".to_string(),
                format!("#[repr(C)] struct __SwiftDoubles({fields});\n"),
                format!(
                    "let __ds = __CALL__;\nlet ({}) = ({});",
                    names.join(", "),
                    values.join(", ")
                ),
            )
        }
        other => panic!("swift::call: cannot return {other:?} through a thunk"),
    };

    let bind = bind.replace(
        "__CALL__",
        &format!("__swift_thunk({})", call_args.join(", ")),
    );

    let doc_alias = alias
        .as_ref()
        .map(|a| format!("#[doc(alias = \"{a}\")]"))
        .unwrap_or_default();

    let Signature {
        meta,
        vis_and_qualifiers,
        name,
        generics,
        args_source,
        ret_source,
        ..
    } = sig;

    let ret_clause = if ret_source.is_empty() {
        String::new()
    } else {
        format!("-> {ret_source}")
    };
    let params = params.join(", ");

    let out = format!(
        "
{meta}
{doc_alias}
#[inline]
{vis_and_qualifiers} fn {name}{generics}{args_source} {ret_clause} {{
    #[allow(non_snake_case)]
    unsafe extern \"C\" {{
        #[link_name = \"{link_name}\"]
        fn __swift_callee();
    }}
    {thunk_item}
    #[unsafe(naked)]
    #[allow(non_snake_case, improper_ctypes_definitions)]
    unsafe extern \"C\" fn __swift_thunk({params}) {thunk_ret} {{
        core::arch::naked_asm!(
            {shuffle}\"bl {{__callee}}\",
            \"ldp x20, x30, [sp], #16\",
            \"ret\",
            __callee = sym __swift_callee,
        )
    }}
    unsafe {{
        {checks}
        {prelude}
        {bind}
        {tail}
    }}
}}
"
    );

    out.parse()
        .unwrap_or_else(|e| panic!("swift::call generated invalid thunk: {e}\n{out}"))
}

/// Requires the Rust signature and the Swift declaration to agree on who owns
/// an argument.
///
/// Taking a value by reference in Rust and surrendering it to Swift, or the
/// reverse, is a leak or a double release that nothing else would catch, so the
/// two spellings of the same fact are made to match.
fn check_ownership(
    alias: &Option<String>,
    conventions: &[bool],
    index: usize,
    name: &str,
    by_value: bool,
) {
    let Some(decl) = alias else {
        // Only a declaration says what the callee wants; a bare symbol does
        // not, so there is nothing to check against.
        return;
    };
    let Some(&consumed) = conventions.get(index) else {
        return;
    };
    if consumed != by_value {
        let (wants, has) = if consumed {
            ("takes it at `+1`", "`&`")
        } else {
            ("borrows it", "by value")
        };
        panic!(
            "swift::call: `{decl}` {wants}, but `{name}` is declared {has}. \
             Take a borrowed argument by reference and a consumed one by value."
        );
    }
}

/// The success half of a return type, which is the whole type unless it throws.
fn ret_ok_type(ret: &str) -> String {
    let ret = normalize(ret);
    match inner_of(&ret, "Result") {
        Some(inner) => split_top(inner)
            .first()
            .cloned()
            .unwrap_or_else(|| ret.clone()),
        None => ret,
    }
}

#[cfg(test)]
mod tests {
    use super::unescape;

    /// A declaration written across source lines has to arrive as one line, or
    /// the continuation ends up inside an identifier's length prefix.
    #[test]
    fn a_line_continuation_leaves_no_trace() {
        assert_eq!(
            "Speech.SpeechTranscriber(class).Result(struct).text: Foundation.AttributedString",
            unescape(
                r#""Speech.SpeechTranscriber(class).Result(struct).text: \
             Foundation.AttributedString""#
            )
        );
        assert_eq!("plain", unescape(r#""plain""#));
        assert_eq!("a\"b", unescape(r#""a\"b""#));
    }
}
