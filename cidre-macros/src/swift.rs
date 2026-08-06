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
            Class::Indirect(ty) | Class::OptIndirect(ty) => {
                Some((ty, "Indirect".to_string()))
            }
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

/// Normalizes a type as the token stream prints it, so the table below can
/// match on it without caring how the source spaced it.
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
    out.trim().to_string()
}

/// Strips one layer of `Name < ... >`, returning the inner type.
fn inner_of<'a>(ty: &'a str, name: &str) -> Option<&'a str> {
    let head = format!("{name} <");
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
            Class::Bool | Class::Word | Class::Double | Class::Float => {
                Class::OptPrimitive(inner)
            }
            other => panic!("swift::call: cannot return Option<{inner}> ({other:?})"),
        };
    }
    if let Some(inner) = inner_of(&ty, "Result") {
        // `Result<T, arc::R<ns::Error>>` is how a throwing call is spelled; the
        // error half is fixed, so only the success half classifies.
        let (ok, _err) = split_top(inner)
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
    // A Swift container is one word whatever it holds, and the Rust wrapper
    // takes that word as its whole representation.
    for container in ["Array", "Set", "Dictionary"] {
        let bare = ty.strip_prefix("swift :: ").unwrap_or(&ty);
        if bare.starts_with(&format!("{container} <")) {
            return Class::RawWord(ty.clone());
        }
    }
    if ty == "cm :: Time" {
        return Class::Words3;
    }
    // Geometry comes back in `d0` up. How many registers that is depends on the
    // Swift type rather than on the Rust one — a `Vector3D` is four doubles
    // wide but only three of them are returned — so the count is stated here
    // and the value is built through its own constructor rather than by
    // reinterpreting the bytes.
    for (name, count) in [("cg :: Rect", 4), ("spatial :: Vector3D", 3)] {
        if ty == name {
            return Class::Doubles(count, ty.clone());
        }
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
            if let Some(inner) = inner_of(&ty, "arc :: R").or_else(|| inner_of(&ty, "R")) {
                Class::ClassRef(normalize(inner))
            } else {
                Class::Indirect(ty.clone())
            }
        }
    }
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
        return if borrowed { Class::StringRef } else { Class::String };
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
            if inner_of(&bare, "arc :: R").is_some() || inner_of(&bare, "R").is_some() {
                Class::Word
            } else {
                Class::ValuePtr
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

fn parse_attr(attr: TokenStream) -> Symbol {
    let text = attr.to_string();
    let text = text.trim();
    let (kind, rest) = match text.split_once('=') {
        Some((k, r)) if k.trim() == "sym" => ("sym", r.trim()),
        _ => ("decl", text),
    };
    let unquoted = unescape(rest);
    match kind {
        "sym" => Symbol::Mangled(unquoted),
        _ => Symbol::Decl(unquoted),
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
    let symbol = parse_attr(attr);
    let (link_name, alias, conventions) = match symbol {
        // Without a declaration there is nothing to read a convention off, so
        // every argument is taken to be borrowed, which is a method's default.
        Symbol::Mangled(s) => (s, None, Vec::new()),
        Symbol::Decl(d) => {
            let mangled = crate::swift_mangle::mangle(&d)
                .unwrap_or_else(|e| panic!("swift::call: cannot mangle `{d}`: {e}"));
            let conventions = crate::swift_mangle::param_conventions(&d)
                .unwrap_or_else(|e| panic!("swift::call: cannot read `{d}`: {e}"));
            (mangled, Some(d), conventions)
        }
    };

    let sig = parse_signature(func);
    let ret_class = classify_ret(&sig.ret_source);
    let throws = is_throwing(&sig.ret_source);

    // Argument setup, in declaration order: integer-class values fill x0 up,
    // floating-point ones d0 up, and `self` always goes in the context
    // register whatever else the call takes.
    let mut int_args: Vec<String> = Vec::new();
    let mut float_args: Vec<String> = Vec::new();
    let mut prelude = String::new();

    for (index, arg) in sig.args.iter().enumerate() {
        match classify_arg(&arg.ty) {
            Class::Bool => int_args.push(format!("{} as usize", arg.name)),
            Class::Word => int_args.push(format!("{} as usize", arg.name)),
            Class::Double => float_args.push(format!("{} as f64", arg.name)),
            Class::Float => float_args.push(format!("{} as f32", arg.name)),
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
            Some((reg, _)) => reg.replace(['d', 's'], "").parse::<usize>().ok().map_or_else(
                || format!("d{index}"),
                |_| format!("{}{index}", &reg[..1]),
            ),
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
                Class::Indirect(ty) => format!(
                    "<{ty} as crate::swift::value::SwiftOut>::out_ptr(&mut __out) as usize"
                ),
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

    let operand_list: String = operands.iter().map(|o| o.render()).collect::<Vec<_>>().join("\n            ");

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
