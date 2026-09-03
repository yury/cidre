/// This is all dirty hacks. We need to reimplement it with syn and quote
///
use std::{borrow::Cow, str::FromStr};

use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

mod swift;
mod swift_mangle;

enum Attr {
    Optional,
    MsgSend(String),
    Overrides(String),
    Init(String),
    ApiAvailable(Versions),
    DocAvailable,
}

impl Attr {
    fn from_stream(stream: TokenStream) -> Option<Attr> {
        let mut iter = stream.into_iter();
        let Some(TokenTree::Ident(ident)) = iter.next() else {
            return None;
        };

        let str = ident.to_string();
        if str == "doc" {
            let Some(TokenTree::Punct(p)) = iter.next() else {
                return None;
            };
            assert_eq!(p, '=');
            let Some(TokenTree::Literal(s)) = iter.next() else {
                return None;
            };
            if s.to_string() == "\" # Availability\"" {
                return Some(Attr::DocAvailable);
            }
        } else if str != "objc" && str != "api" {
            return None;
        }

        let Some(TokenTree::Punct(p)) = iter.next() else {
            return None;
        };

        assert_eq!(p, ':');

        let Some(TokenTree::Punct(p)) = iter.next() else {
            return None;
        };

        assert_eq!(p, ':');

        if let Some(tt) = iter.next() {
            match tt {
                TokenTree::Group(v) => panic!("didn't expect group {v}"),
                TokenTree::Ident(v) => {
                    let v = v.to_string();
                    return match v.as_str() {
                        "optional" => Some(Attr::Optional),
                        "msg_send" => {
                            let Some(TokenTree::Group(a)) = iter.next() else {
                                return None;
                            };
                            let sel = a.stream().to_string().replace([' ', '\n'], "");
                            Some(Attr::MsgSend(sel))
                        }
                        "overrides" | "init" => {
                            let Some(TokenTree::Group(a)) = iter.next() else {
                                return None;
                            };
                            let sel = a.stream().to_string().replace([' ', '\n'], "");
                            if v == "init" {
                                Some(Attr::Init(sel))
                            } else {
                                Some(Attr::Overrides(sel))
                            }
                        }
                        "available" => {
                            let Some(TokenTree::Group(a)) = iter.next() else {
                                return None;
                            };
                            Some(Attr::ApiAvailable(Versions::from_stream(a.stream())))
                        }
                        _ => None,
                    };
                }
                TokenTree::Punct(v) => panic!("didn't expect punct {v}"),
                TokenTree::Literal(v) => panic!("didn't expect literal {v}"),
            }
        }

        panic!("Unexpected attribute")
    }
}

/// Calls a Swift entry point named by its declaration, with the registers its
/// convention asks for derived from the Rust signature.
#[proc_macro_attribute]
pub fn swift_call(decl: TokenStream, func: TokenStream) -> TokenStream {
    swift::gen_swift_call(decl, func)
}

/// The address of the Swift entry point a declaration names.
#[proc_macro]
pub fn swift_symbol(args: TokenStream) -> TokenStream {
    swift::gen_symbol(args)
}

/// The address of a Swift type's metadata accessor, from the type's name.
#[proc_macro]
pub fn swift_metadata_accessor(args: TokenStream) -> TokenStream {
    swift::gen_metadata_accessor(args)
}

/// Should generate static fn sel_xxx function that gets selector.
/// So user can check selector with is_reponds_to_sel
#[proc_macro_attribute]
pub fn optional(_sel: TokenStream, func: TokenStream) -> TokenStream {
    let mut iter = func.clone().into_iter();

    let extern_name = loop {
        let Some(TokenTree::Punct(p)) = iter.next() else {
            panic!("expect #[objc::msg_send(...)]")
        };
        if p != '#' {
            panic!("expect #[objc::msg_send(...)]")
        }
        let Some(TokenTree::Group(g)) = iter.next() else {
            panic!("expect #[objc::msg_send(...)]")
        };
        if let Some(Attr::MsgSend(extern_name)) = Attr::from_stream(g.stream()) {
            break extern_name;
        }
    };

    let mut fn_name = None;

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(i) if i.to_string().eq("fn") => {
                let Some(TokenTree::Ident(name)) = iter.next() else {
                    panic!("expect function name");
                };
                fn_name = Some(name.to_string());
            }
            _ => {} // panic?
        }
    }

    let Some(fn_name) = fn_name else {
        panic!("function name not found");
    };

    let getter: TokenStream = format!(
        "
    /// `@selector({extern_name})` but dynamic
    /// use this function to check if object responds to selector
    fn sel_{fn_name}() -> &'static objc::Sel {{
        unsafe {{ objc::sel_reg_name(c\"{extern_name}\".as_ptr()) }}
    }}
        "
    )
    .parse()
    .unwrap();

    let mut func = func;
    func.extend(getter);
    func
}

#[proc_macro_attribute]
pub fn protocol(args: TokenStream, ts: TokenStream) -> TokenStream {
    let mut original_trait = ts.clone();
    let error_msg = "objc::protocol expects protocol name as first argument";
    let mut args = args.into_iter();
    let Some(TokenTree::Ident(ident)) = args.next() else {
        panic!("{}", error_msg);
    };
    assert!(args.next().is_none(), "{}", error_msg);
    let protocol_name = ident.to_string();
    let mut trait_name = String::new();

    let mut pre_tokens = Vec::<TokenTree>::with_capacity(10);
    let mut ts = ts.into_iter();
    let mut group_stream = None;
    while let Some(tt) = ts.next() {
        match tt {
            TokenTree::Group(ref g) if g.delimiter() == Delimiter::Brace => {
                while let Some(tt) = pre_tokens.pop() {
                    let val = tt.to_string();
                    if val == "trait" {
                        pre_tokens.push(tt);
                        break;
                    }
                    trait_name = val;
                }
                group_stream = Some(g.stream());
                break;
            }
            _ => pre_tokens.push(tt),
        }
    }
    if trait_name != protocol_name {
        let alias = format!("#[doc(alias = \"{protocol_name}\")]");
        let stream = TokenStream::from_str(&alias).unwrap();
        let mut alias_tokens = Vec::from_iter(stream.into_iter());
        alias_tokens.append(&mut pre_tokens);
        pre_tokens = alias_tokens;
    }
    let mut is_optional = false;
    let mut skip = false;
    let mut sel = String::new();
    let mut fn_name; // = "".to_string();
    let mut generics = Vec::new();
    //let mut fn_args = Vec::new();
    let mut fn_args_str; // = "".to_string();
    let mut result = Vec::new();
    let mut fn_body = Cow::Borrowed("");

    let mut impl_trait_functions = vec![];
    let mut has_optionals = false;
    let mut fn_names = vec![];

    let mut iter = group_stream.expect("should be group").into_iter();
    while let Some(token) = iter.next() {
        match token {
            TokenTree::Group(g) => println!("group {g}"),
            TokenTree::Ident(i) => {
                let str = i.to_string();
                if str == "fn" {
                    let Some(TokenTree::Ident(name)) = iter.next() else {
                        panic!("expect fn name");
                    };
                    fn_name = name.to_string();
                    let args = loop {
                        let Some(tt) = iter.next() else {
                            panic!("need more tokens");
                        };
                        match tt {
                            TokenTree::Group(args) => break args,
                            _ => generics.push(tt),
                        }
                    };
                    fn_args_str = args.to_string();
                    for tt in iter.by_ref() {
                        match tt {
                            TokenTree::Punct(ref p) if p.as_char() == ';' => {
                                result.push(tt);
                                break;
                            }
                            TokenTree::Group(ref g) if g.delimiter() == Delimiter::Brace => {
                                fn_body = Cow::Owned(g.to_string());
                                break;
                            }
                            _ => result.push(tt),
                        }
                    }

                    let mut ext = "";

                    let register_sel = if sel.is_empty() {
                        Cow::Borrowed("None")
                    } else {
                        ext = "extern \"C\" ";
                        fn_args_str = fn_args_str.replacen("(& self", "(&self", 1);
                        fn_args_str =
                            fn_args_str.replacen("(&self", "(&self, _cmd: Option<&objc::Sel>", 1);

                        fn_args_str = fn_args_str.replacen("(& mut self", "(&mut self", 1);
                        fn_args_str = fn_args_str.replacen(
                            "(&mut self",
                            "(&mut self, _cmd: Option<&objc::Sel>",
                            1,
                        );
                        Cow::Owned(format!(
                            "unsafe {{ objc::sel_reg_name(c\"{sel}\".as_ptr()) }}"
                        ))
                    };

                    if is_optional && !sel.is_empty() && fn_body.is_empty() {
                        result.pop(); // remove ';'
                        fn_body = Cow::Borrowed("{ unimplemented!() }");
                    }

                    if !is_optional && sel.is_empty() {
                        skip = true;
                    }

                    let ret = if result.is_empty() {
                        Cow::Borrowed("")
                    } else {
                        Cow::Owned(TokenStream::from_iter(result.clone().into_iter()).to_string())
                    };

                    let gen = if generics.is_empty() {
                        Cow::Borrowed("")
                    } else {
                        Cow::Owned(TokenStream::from_iter(generics.clone().into_iter()).to_string())
                    };

                    let gen_rar_version = ret.contains("arc :: R <") && !returns_retained(&sel);

                    let impl_fn = if skip {
                        format!(
                            "
    {ext}fn {fn_name}{gen}{fn_args_str}{ret} {fn_body}

                                    "
                        )
                    } else {
                        fn_names.push((fn_name.clone(), gen_rar_version));
                        if gen_rar_version {
                            let ret = ret.replacen("arc :: R <", "arc :: Rar <", 1);
                            format!(
                                "
        {ext}fn impl_{fn_name}_ar{gen}{fn_args_str}{ret} {fn_body}

                                        "
                            )
                        } else {
                            format!(
                                "
    {ext}fn impl_{fn_name}{gen}{fn_args_str}{ret} {fn_body}

                                    "
                            )
                        }
                    };

                    if !skip {
                        impl_trait_functions.push(impl_fn);
                    }

                    if !is_optional && !skip {
                        let impl_sel = format!(
                            "
    fn sel_{fn_name}() -> &'static objc::Sel {{ {register_sel} }}
        "
                        );
                        impl_trait_functions.push(impl_sel);
                    }

                    is_optional = false;
                    sel.clear();
                    fn_name.clear();
                    fn_body = Cow::Borrowed("");
                    fn_args_str.clear();
                    generics.clear();
                    skip = false;
                    //fn_args.clear();
                    result.clear();
                }
            }
            TokenTree::Punct(p) => match p.as_char() {
                '#' => {
                    let TokenTree::Group(g) = iter.next().unwrap() else {
                        panic!("not a group");
                    };
                    match Attr::from_stream(g.stream()) {
                        Some(Attr::Optional) => {
                            has_optionals = true;
                            is_optional = true;
                        }
                        Some(Attr::MsgSend(s)) => sel = s,
                        Some(_) => continue,
                        None => continue,
                    }
                }
                _ => panic!("other char '{p}'"),
            },
            TokenTree::Literal(l) => println!("lit {l}"),
        }
    }
    let pre = TokenStream::from_iter(pre_tokens).to_string();
    let obj_trait_name = format!("{trait_name}Impl");
    let fns = impl_trait_functions.join("\n");

    let add_methods = if has_optionals {
        Cow::Borrowed("fn cls_add_methods<O: objc::Obj>(cls: &objc::Class<O>);")
    } else {
        Cow::Owned(add_methods_fn(&fn_names))
    };

    let add_protocol = format!(
        "
    fn cls_add_protocol<O: objc::Obj>(cls: &objc::Class<O>) {{
        unsafe {{
            let cls: &objc::Class<objc::Id> = std::mem::transmute(cls);
            if let Some(proto) =  objc::objc_getProtocol(c\"{protocol_name}\".as_ptr())  {{
                cls.add_protocol(proto);
            }}
        }}
    }}
        "
    );

    let code = format!(
        "

{pre} {obj_trait_name}: {trait_name} {{
    {fns}
    {add_methods}
    {add_protocol}
}}
        "
    );

    let ts: TokenStream = code.parse().unwrap();

    original_trait.extend(ts);
    original_trait
}

fn add_methods_fn(fns: &[(String, bool)]) -> String {
    let mut res = "
    fn cls_add_methods<O: objc::Obj>(cls: &objc::Class<O>) {
        let cls: &objc::Class<objc::Id> = unsafe { std::mem::transmute(cls) };
        "
    .to_string();
    for (f, ar) in fns {
        let suffix = if *ar { "_ar" } else { "" };
        let add = format!(
            "
        let sel = Self::sel_{f}();
        unsafe {{
            let imp: extern \"C\" fn() = std::mem::transmute(Self::impl_{f}{suffix} as *const u8);
            objc::class_addMethod(cls, sel, imp, std::ptr::null());
        }}
            ",
        );
        res.push_str(&add);
    }
    res.push_str("\n}");
    res
}
/// Marker for a method override inside an `#[objc::add_methods] impl Type { .. }` block.
///
/// `#[objc::overrides(layout)] fn layout(&mut self) { .. }` registers the method for the
/// selector `layout` and generates `fn super_layout(&mut self)`, which calls the
/// superclass implementation. Consumed by `add_methods`; on its own it is an error.
#[proc_macro_attribute]
pub fn overrides(_args: TokenStream, _item: TokenStream) -> TokenStream {
    panic!("#[objc::overrides(..)] must be used inside an #[objc::add_methods] impl block")
}

/// Declares an Objective-C initializer inside the type's own `impl` block.
///
/// ```ignore
/// impl TextField {
///     #[objc::init(initWithFrame:)]
///     pub fn init_with_frame(self, frame: cg::Rect) -> arc::R<TextField>;
/// }
/// ```
///
/// The method is emitted on `arc::A<TextField>` (the type is taken from the return
/// type, so write it out rather than `Self`), and is called as
/// `TextField::alloc().init_with_frame(frame)`. Generic parameters of the type are
/// assumed to be `objc::Obj`. The selector must belong to the `init` family.
///
/// Inside an `#[objc::add_methods] impl Type { .. }` block, where `impl arc::A<Type>`
/// is not possible for crates other than cidre, the declaration is instead moved to a
/// generated `trait TypeInit` implemented for `arc::A<Type>`, and `Self` may be used.
#[proc_macro_attribute]
pub fn init(sel: TokenStream, func: TokenStream) -> TokenStream {
    let sel = sel.to_string().replace([' ', '\n'], "");
    assert!(
        in_method_family(&sel, "init"),
        "#[objc::init] expects a selector of the init family, got `{sel}`"
    );
    let mut iter = func.into_iter();
    let mut pending = vec![];
    for tt in iter.by_ref() {
        if matches!(&tt, TokenTree::Ident(i) if i.to_string() == "fn") {
            break;
        }
        pending.push(tt);
    }
    let decl = parse_fn_decl(&mut iter);
    assert!(
        decl.body.is_none(),
        "#[objc::init] declares a selector and has no body"
    );
    let (vis, quals, attrs) = split_fn_prefix(&pending);
    let ty = init_target_type(&decl.ret).unwrap_or_else(|| {
        panic!(
            "#[objc::init] needs the initialized type in the return type, e.g. `-> arc::R<TextField>`, got `{}`",
            decl.ret
        )
    });
    let generics = impl_generics_for(&ty);
    let FnDecl {
        name,
        generics: fn_generics,
        args,
        ret,
        ..
    } = &decl;
    let code = format!(
        "
    #[doc(hidden)]
    #[allow(non_upper_case_globals, non_local_definitions)]
    const cidre_init_{name}: () = {{
        impl{generics} arc::A<{ty}> {{
            #[objc::msg_send({sel})]
            {attrs}
            {vis} {quals} fn {name}{fn_generics}{args} {ret};
        }}
    }};
        "
    );
    code.parse()
        .unwrap_or_else(|e| panic!("objc::init generated invalid code: {e}\n{code}"))
}

/// `-> arc :: R < Dictionary < K , V > >` -> `Dictionary < K , V >`; also `Retained` and
/// `Option < .. >` wrappers.
fn init_target_type(ret: &str) -> Option<String> {
    let ret = ret.trim().trim_start_matches("->").trim();
    let inner = |s: &str, wrapper: &str| -> Option<String> {
        let start = s.find(wrapper)? + wrapper.len();
        let rest = &s[start..];
        let mut depth = 0;
        for (i, c) in rest.char_indices() {
            match c {
                '<' => depth += 1,
                '>' if depth == 0 => return Some(rest[..i].trim().to_string()),
                '>' => depth -= 1,
                _ => {}
            }
        }
        None
    };
    let ty = inner(ret, "R <").or_else(|| inner(ret, "Retained <"))?;
    if ty == "Self" {
        return None;
    }
    Some(ty)
}

/// `Dictionary < K , V >` -> `<K: objc::Obj, V: objc::Obj>`; bare single identifiers
/// among the type arguments are taken as type parameters.
fn impl_generics_for(ty: &str) -> String {
    let Some(start) = ty.find('<') else {
        return String::new();
    };
    let Some(end) = ty.rfind('>') else {
        return String::new();
    };
    let stream: TokenStream = ty[start + 1..end].parse().unwrap_or_default();
    let params: Vec<String> = split_top_level_args(stream)
        .iter()
        .map(|a| a.trim().to_string())
        .filter(|a| !a.is_empty() && a.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .map(|a| format!("{a}: objc::Obj"))
        .collect();
    if params.is_empty() {
        String::new()
    } else {
        format!("<{}>", params.join(", "))
    }
}

/// Registers Objective-C methods for a runtime-defined class.
///
/// On `impl ProtocolImpl for Type` it registers every `fn impl_*` under the selector
/// the protocol trait declares. On an inherent `impl Type` it handles
/// `#[objc::overrides(sel)]` and `#[objc::init(sel)]` functions.
#[proc_macro_attribute]
pub fn add_methods(_args: TokenStream, tr_impl: TokenStream) -> TokenStream {
    let mut header = vec![];
    let mut body = None;
    for tt in tr_impl {
        match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace && body.is_none() => {
                body = Some(g)
            }
            _ => header.push(tt),
        }
    }
    let body = body.expect("objc::add_methods expects an impl block");
    let is_trait_impl = header
        .iter()
        .any(|t| matches!(t, TokenTree::Ident(i) if i.to_string() == "for"));
    if is_trait_impl {
        add_trait_impl_methods(header, body)
    } else {
        add_own_methods(header, body)
    }
}

fn add_trait_impl_methods(header: Vec<TokenTree>, body: Group) -> TokenStream {
    let mut fns = vec![];
    let mut iter = body.stream().into_iter();
    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Ident(i) if i.to_string().eq("fn") => {
                let Some(TokenTree::Ident(f)) = iter.next() else {
                    panic!("expected function name");
                };
                let f = f.to_string().replacen("impl_", "", 1);
                if let Some(f) = f.strip_suffix("_ar") {
                    fns.push((f.to_string(), true));
                } else {
                    fns.push((f, false));
                }
            }
            _ => continue,
        }
    }
    let imp: TokenStream = add_methods_fn(&fns).parse().unwrap();
    let mut stream = body.stream();
    stream.extend(imp);
    let mut tokens = header;
    tokens.push(TokenTree::Group(Group::new(body.delimiter(), stream)));
    TokenStream::from_iter(tokens)
}

struct FnDecl {
    name: String,
    generics: String,
    args: Group,
    /// return type and where clause, without the trailing `;`
    ret: String,
    body: Option<Group>,
    /// everything after `fn`, verbatim
    tokens: Vec<TokenTree>,
}

fn parse_fn_decl(iter: &mut impl Iterator<Item = TokenTree>) -> FnDecl {
    let mut tokens = vec![];
    let Some(TokenTree::Ident(name)) = iter.next() else {
        panic!("expected function name");
    };
    tokens.push(TokenTree::Ident(name.clone()));
    let mut generics = vec![];
    let args = loop {
        let Some(tt) = iter.next() else {
            panic!("expected function arguments");
        };
        tokens.push(tt.clone());
        match tt {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => break g,
            _ => generics.push(tt),
        }
    };
    let mut ret = vec![];
    let mut body = None;
    loop {
        let Some(tt) = iter.next() else {
            panic!("expected `;` or function body");
        };
        tokens.push(tt.clone());
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == ';' => break,
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                body = Some(g);
                break;
            }
            _ => ret.push(tt),
        }
    }
    FnDecl {
        name: name.to_string(),
        generics: TokenStream::from_iter(generics).to_string(),
        args,
        ret: TokenStream::from_iter(ret).to_string(),
        body,
        tokens,
    }
}

/// Takes `#[objc::overrides(..)]` / `#[objc::init(..)]` out of `pending`, returning it
/// with the position it had, so other attributes keep their order around it.
fn take_method_attr(pending: &mut Vec<TokenTree>) -> Option<(Attr, usize)> {
    let mut i = 0;
    while i + 1 < pending.len() {
        if let (TokenTree::Punct(p), TokenTree::Group(g)) = (&pending[i], &pending[i + 1]) {
            if p.as_char() == '#' && g.delimiter() == Delimiter::Bracket {
                let path = g.stream().to_string().replace(' ', "");
                let ours = path.starts_with("objc::overrides") || path.starts_with("objc::init");
                if ours {
                    if let attr @ Some(Attr::Overrides(_) | Attr::Init(_)) =
                        Attr::from_stream(g.stream())
                    {
                        pending.drain(i..i + 2);
                        return attr.map(|a| (a, i));
                    }
                }
            }
        }
        i += 1;
    }
    None
}

struct ImplHeader {
    /// attributes before `impl`
    attrs: Vec<TokenTree>,
    /// `<S: Obj, I: Obj>` or empty
    generics: String,
    /// the implemented type, e.g. `View` or `Dict < K , V >`
    ty: String,
    /// `where ..` or empty
    where_clause: String,
}

fn parse_impl_header(header: &[TokenTree]) -> ImplHeader {
    let mut iter = header.iter().cloned().peekable();
    let mut attrs = vec![];
    for tt in iter.by_ref() {
        if matches!(&tt, TokenTree::Ident(i) if i.to_string() == "impl") {
            break;
        }
        attrs.push(tt);
    }
    let mut generics = vec![];
    if matches!(iter.peek(), Some(TokenTree::Punct(p)) if p.as_char() == '<') {
        let mut depth = 0;
        for tt in iter.by_ref() {
            match &tt {
                TokenTree::Punct(p) if p.as_char() == '<' => depth += 1,
                TokenTree::Punct(p) if p.as_char() == '>' => depth -= 1,
                _ => {}
            }
            generics.push(tt);
            if depth == 0 {
                break;
            }
        }
    }
    let mut ty = vec![];
    let mut where_clause = vec![];
    let mut in_where = false;
    for tt in iter {
        if matches!(&tt, TokenTree::Ident(i) if i.to_string() == "where") {
            in_where = true;
        }
        if in_where {
            where_clause.push(tt);
        } else {
            ty.push(tt);
        }
    }
    ImplHeader {
        attrs,
        generics: TokenStream::from_iter(generics).to_string(),
        ty: TokenStream::from_iter(ty).to_string(),
        where_clause: TokenStream::from_iter(where_clause).to_string(),
    }
}

/// `<S: Obj, I: Obj, 'a>` -> `<S, I, 'a>`
fn generic_param_names(generics: &str) -> String {
    let inner = generics.trim();
    let inner = inner.strip_prefix('<').unwrap_or(inner);
    let inner = inner.strip_suffix('>').unwrap_or(inner);
    let stream: TokenStream = inner.parse().unwrap_or_default();
    let names: Vec<String> = split_top_level_args(stream)
        .iter()
        .map(|p| {
            p.split_once(':')
                .map(|(n, _)| n)
                .unwrap_or(p)
                .trim()
                .to_string()
        })
        .filter(|n| !n.is_empty())
        .collect();
    if names.is_empty() {
        String::new()
    } else {
        format!("<{}>", names.join(", "))
    }
}

struct InitDecl {
    vis: String,
    /// attributes written before `#[objc::init]`
    attrs: String,
    /// attributes written after `#[objc::init]`
    attrs_after: String,
    quals: String,
    sel: String,
    name: String,
    generics: String,
    /// arguments and return type with `Self` replaced by the implemented type
    sig: String,
}

fn add_own_methods(header: Vec<TokenTree>, body: Group) -> TokenStream {
    let ImplHeader {
        attrs: impl_attrs,
        generics: impl_generics,
        ty,
        where_clause,
    } = parse_impl_header(&header);
    // `foo::Bar<T>` -> `Bar`
    let ty_ident = ty
        .split('<')
        .next()
        .unwrap()
        .split("::")
        .last()
        .unwrap()
        .trim()
        .to_string();

    let mut out = TokenStream::new();
    let mut pending: Vec<TokenTree> = vec![];
    let mut extra = String::new();
    let mut registrations = String::new();
    let mut inits: Vec<InitDecl> = vec![];

    let mut iter = body.stream().into_iter();
    while let Some(tt) = iter.next() {
        let is_fn = matches!(&tt, TokenTree::Ident(i) if i.to_string() == "fn");
        if !is_fn {
            pending.push(tt);
            continue;
        }
        let attr = take_method_attr(&mut pending);
        let decl = parse_fn_decl(&mut iter);
        let pos = attr.as_ref().map(|(_, i)| *i).unwrap_or(0);
        match attr.map(|(a, _)| a) {
            None => {
                out.extend(pending.drain(..));
                out.extend(std::iter::once(tt));
                out.extend(decl.tokens.iter().cloned());
            }
            Some(Attr::Overrides(sel)) => {
                out.extend(pending.drain(..));
                out.extend(std::iter::once(tt));
                out.extend(decl.tokens.iter().cloned());
                let (tramp, reg) = gen_override(&sel, &decl);
                extra.push_str(&tramp);
                registrations.push_str(&reg);
            }
            Some(Attr::Init(sel)) => {
                assert!(
                    decl.body.is_none(),
                    "#[objc::init] declares a selector and has no body"
                );
                let (vis, quals, attrs) = split_fn_prefix(&pending[..pos]);
                let (vis2, quals2, attrs_after) = split_fn_prefix(&pending[pos..]);
                pending.clear();
                let sig = regex_self(&format!("{}{}", decl.args, decl.ret), &ty);
                inits.push(InitDecl {
                    vis: if vis.is_empty() { vis2 } else { vis },
                    attrs,
                    attrs_after,
                    quals: if quals.is_empty() { quals2 } else { quals },
                    sel,
                    name: decl.name.clone(),
                    generics: decl.generics.clone(),
                    sig,
                });
            }
            Some(_) => unreachable!(),
        }
    }
    out.extend(pending);

    if !registrations.is_empty() {
        extra.push_str(&format!(
            "
    #[doc(hidden)]
    pub fn cls_add_own_methods(cls: &objc::Class<objc::Id>) {{
        unsafe {{
            {registrations}
        }}
    }}
            "
        ));
    }
    out.extend(extra.parse::<TokenStream>().unwrap());

    let mut tokens = header;
    tokens.push(TokenTree::Group(Group::new(body.delimiter(), out)));
    let mut result = TokenStream::from_iter(tokens);

    if inits.is_empty() {
        return result;
    }

    let impl_attrs = TokenStream::from_iter(impl_attrs).to_string();
    // `impl arc::A<T>` is only possible inside cidre; a local trait implemented for
    // `arc::A<T>` is what the orphan rule permits everywhere else.
    let trait_name = format!("{ty_ident}Init");
    let trait_vis = inits
        .iter()
        .map(|i| i.vis.as_str())
        .find(|v| !v.is_empty())
        .unwrap_or("");
    let mut decls = String::new();
    let mut impls = String::new();
    for i in &inits {
        let InitDecl {
            attrs,
            attrs_after,
            quals,
            sel,
            name,
            generics,
            sig,
            ..
        } = i;
        decls.push_str(&format!(
            "\n    {attrs}\n    {attrs_after}\n    {quals} fn {name}{generics}{sig};\n"
        ));
        impls.push_str(&format!(
            "\n    {attrs}\n    #[objc::msg_send({sel})]\n    {attrs_after}\n    {quals} fn {name}{generics}{sig};\n"
        ));
    }
    let names = generic_param_names(&impl_generics);
    let code = format!(
        "
/// Initializers of `{ty}`, callable on `arc::A<{ty}>`
#[allow(non_camel_case_types)]
{impl_attrs}
{trait_vis} trait {trait_name}{impl_generics} {where_clause} {{
    {decls}
}}

{impl_attrs}
impl{impl_generics} {trait_name}{names} for arc::A<{ty}> {where_clause} {{
    {impls}
}}
        "
    );
    result.extend(
        code.parse::<TokenStream>()
            .unwrap_or_else(|e| panic!("objc::add_methods generated invalid code: {e}\n{code}")),
    );
    result
}

/// Splits the tokens before `fn` into (`pub(..)`, qualifiers like `unsafe`, attributes).
fn split_fn_prefix(pending: &[TokenTree]) -> (String, String, String) {
    let mut vis = vec![];
    let mut quals = vec![];
    let mut attrs = vec![];
    let mut iter = pending.iter().cloned().peekable();
    while let Some(tt) = iter.next() {
        match &tt {
            TokenTree::Ident(i) if i.to_string() == "pub" => {
                vis.push(tt.clone());
                if let Some(TokenTree::Group(g)) = iter.peek() {
                    if g.delimiter() == Delimiter::Parenthesis {
                        vis.push(iter.next().unwrap());
                    }
                }
            }
            TokenTree::Ident(i)
                if matches!(i.to_string().as_str(), "unsafe" | "const" | "async") =>
            {
                quals.push(tt.clone());
            }
            TokenTree::Ident(i) if i.to_string() == "extern" => {
                quals.push(tt.clone());
                if let Some(TokenTree::Literal(_)) = iter.peek() {
                    quals.push(iter.next().unwrap());
                }
            }
            _ => attrs.push(tt),
        }
    }
    (
        TokenStream::from_iter(vis).to_string(),
        TokenStream::from_iter(quals).to_string(),
        TokenStream::from_iter(attrs).to_string(),
    )
}

/// Replaces the `Self` type with `ty` in a signature string.
fn regex_self(sig: &str, ty: &str) -> String {
    let mut res = String::with_capacity(sig.len());
    let bytes = sig.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let is_word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
        if sig[i..].starts_with("Self")
            && (i == 0 || !is_word(bytes[i - 1]))
            && (i + 4 >= bytes.len() || !is_word(bytes[i + 4]))
        {
            res.push_str(ty);
            i += 4;
        } else {
            res.push(sig.as_bytes()[i] as char);
            i += 1;
        }
    }
    res
}

/// Returns the `extern "C"` trampoline plus `super_*` for an override, and its
/// registration statement.
fn gen_override(sel: &str, decl: &FnDecl) -> (String, String) {
    assert!(
        decl.generics.is_empty(),
        "#[objc::overrides] methods can't be generic"
    );
    let params = split_top_level_args(decl.args.stream());
    let receiver = params
        .first()
        .map(|p| p.replace(' ', ""))
        .unwrap_or_default();
    let receiver = match receiver.as_str() {
        "&self" => "&self",
        "&mutself" => "&mut self",
        _ => panic!("#[objc::overrides] methods take `&self` or `&mut self`"),
    };
    let rest = &params[1..];
    let names: Vec<String> = rest
        .iter()
        .map(|p| {
            let name = p.split_once(':').expect("typed parameter").0.trim();
            name.trim_start_matches("mut ").trim().to_string()
        })
        .collect();
    let rest_args = if rest.is_empty() {
        String::new()
    } else {
        format!(", {}", rest.join(", "))
    };
    let names = names.join(", ");
    let name = &decl.name;
    let args = decl.args.to_string();
    let ret = decl
        .ret
        .split("where")
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    let sel_args_count = sel.matches(':').count();
    assert_eq!(
        sel_args_count,
        rest.len(),
        "selector `{sel}` and arguments of `{name}` don't match"
    );

    // an object returned at +0 must be autoreleased on the way out
    let autorelease = ret.contains("arc :: R <") && !returns_retained(sel);
    let (tramp_ret, call) = if autorelease {
        let tramp_ret = ret.replacen("arc :: R <", "arc :: Rar <", 1);
        let call = if ret.contains("Option <") {
            format!("self.{name}({names}).map(|r| unsafe {{ r.return_ar() }})")
        } else {
            format!("unsafe {{ self.{name}({names}).return_ar() }}")
        };
        (tramp_ret, call)
    } else {
        (ret.clone(), format!("self.{name}({names})"))
    };

    let tramp = format!(
        "
    #[doc(hidden)]
    extern \"C\" fn cidre_imp_{name}({receiver}, _cmd: *const std::ffi::c_void{rest_args}) {tramp_ret} {{
        {call}
    }}

    /// Calls the superclass implementation of `{sel}`
    #[objc::msg_send_super({sel})]
    fn super_{name}{args} {ret};
        "
    );
    let reg = format!(
        "
            objc::class_addMethod(
                cls,
                objc::sel_reg_name(c\"{sel}\".as_ptr()),
                std::mem::transmute(Self::cidre_imp_{name} as *const u8),
                std::ptr::null(),
            );
        "
    );
    (tramp, reg)
}

#[proc_macro_attribute]
pub fn msg_send_debug(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, x86_64, true, false)
}

#[proc_macro_attribute]
pub fn msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, x86_64, false, false)
}

#[proc_macro_attribute]
pub fn msg_send_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, x86_64, false, false)
}

/// Sends `sel` to the superclass implementation, like `[super sel]` in Objective-C.
///
/// The receiver type must provide `Self::super_cls()` (generated by `define_obj_type!`
/// for runtime-registered classes, or available through `objc::Subclass` in trait
/// default methods). Unlike `msg_send`, the selector is registered at call time because
/// the linker provides no `objc_msgSendSuper` selector stubs.
#[proc_macro_attribute]
pub fn msg_send_super(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, x86_64, false, true)
}

#[proc_macro_attribute]
pub fn msg_send_super_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, x86_64, false, true)
}

fn gen_msg_send(
    sel: TokenStream,
    func: TokenStream,
    x86_64: bool,
    debug: bool,
    super_call: bool,
) -> TokenStream {
    let sel = sel.to_string().replace([' ', '\n'], "");
    let sel_args_count = sel.matches(':').count();

    let mut iter = func.into_iter();
    let mut meta: Vec<TokenTree> = Vec::new();
    let mut unsafe_already = false;
    let mut optional_already = false;
    let mut versions = Versions::default();

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Group(ref g) => {
                if g.delimiter() == Delimiter::Bracket {
                    match Attr::from_stream(g.stream()) {
                        Some(Attr::Optional) => optional_already = true,
                        Some(Attr::ApiAvailable(v)) => {
                            versions = v;
                            meta.pop();
                            continue;
                        }
                        Some(Attr::DocAvailable) => {
                            iter.next(); // Punct('#')
                            let Some(TokenTree::Group(g)) = iter.next() else {
                                panic!("Expect doc with versions");
                            };
                            let mut doc_iter = g.stream().into_iter();
                            doc_iter.next(); // Ident("doc")
                            doc_iter.next(); // Punct('=')
                            let Some(TokenTree::Literal(s)) = doc_iter.next() else {
                                panic!("Expect doc with versions");
                            };
                            let str = s.to_string();
                            versions = Versions::from_doc_str(&str[1..str.len() - 1]);
                            meta.push(tt.clone());
                            meta.push(TokenTree::Punct(Punct::new(
                                '#',
                                proc_macro::Spacing::Joint,
                            )));
                            meta.push(TokenTree::Group(g));
                            continue;
                        }
                        Some(Attr::MsgSend(_)) => panic!("only one msg_send is allowed"),
                        Some(Attr::Overrides(_) | Attr::Init(_)) => {
                            panic!("objc::overrides/objc::init can't be combined with msg_send")
                        }
                        None => {}
                    }
                }
            }
            TokenTree::Ident(ref i) => {
                let i = i.to_string();
                match i.as_str() {
                    "fn" => break,
                    "unsafe" => unsafe_already = true,
                    _ => {}
                }
            }
            TokenTree::Punct(_) => {}
            TokenTree::Literal(_) => {}
        }
        meta.push(tt);
    }

    let Some(TokenTree::Ident(fn_name)) = iter.next() else {
        panic!("expected function name");
    };

    let fn_name = fn_name.to_string();
    let mut generics = Vec::new();
    let doc_alias = if fn_name != sel {
        format!("#[doc(alias = \"{sel}\")]")
    } else {
        String::new()
    };

    let args = loop {
        let Some(tt) = iter.next() else {
            panic!("need more tokens");
        };
        match tt {
            TokenTree::Group(args) => break args,
            _ => generics.push(tt),
        }
    };

    let gen = TokenStream::from_iter(generics).to_string();

    let mut ret = TokenStream::from_iter(iter).to_string();
    assert_eq!(ret.pop().expect(";"), ';');
    let ret_full = ret.to_string();
    if let Some((a, _)) = ret.split_once("where") {
        ret = a.to_string();
    }
    let option = ret_full.contains("-> Option <");
    if debug {
        println!("{option}: {ret_full}");
    }
    let gen_rar_version = ret.contains("arc :: R <") && !returns_retained(&sel);

    if debug {
        println!("option: {option}, gen_rar_version {gen_rar_version} ret: {ret}");
    }

    let fn_args = args.to_string();

    let (class, vars) = fn_args_from_stream(args.stream());
    let fn_args_count = vars.len();

    assert_eq!(
        sel_args_count, fn_args_count,
        "selector and function args don't match {vars:?}"
    );

    let (mut fn_args, mut call_args) = if x86_64 {
        let fn_args = fn_args.replacen('(', "(id:", 1).replacen(
            "self",
            "Self, imp: *const std::ffi::c_void",
            1,
        );
        (
            fn_args,
            format!("sig(self, x86_64_sel, {})", vars.join(", ")),
        )
    } else if fn_args_count == 0 {
        let fn_args = fn_args
            .replacen("( &", "(id: &", 1)
            .replacen("self", "Self", 1);
        (fn_args, "sig(self)".to_string())
    } else {
        let fn_args = fn_args
            .replacen('(', "(id:", 1)
            .replace("self", "Self, imp: *const std::ffi::c_void");
        (
            fn_args,
            format!("sig(self, std::ptr::null(), {})", vars.join(", ")),
        )
    };

    if class {
        if x86_64 {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, imp: *const std::ffi::c_void, ",
                1,
            );
            call_args = call_args.replacen("sig(self", "sig(Self::cls_ptr()", 1);
        } else if fn_args_count == 0 {
            fn_args = fn_args.replacen('(', "(cls: *const std::ffi::c_void", 1);
            call_args = call_args.replacen("sig(self", "sig(Self::cls_ptr()", 1);
        } else {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, imp: *const std::ffi::c_void, ",
                1,
            );
            call_args = call_args.replacen("sig(self", "sig(Self::cls_ptr()", 1);
        }
    }

    let available = versions.available_cfg();
    let unavailable = versions.unavailable_cfg();

    let mut flow = String::new();
    let pre = TokenStream::from_iter(meta).to_string();
    let self_ = if class { "Self::" } else { "self." };
    let vars = vars.join(", ");
    let mut impl_fn_name = fn_name.clone();
    let impl_ret_full = if gen_rar_version {
        ret_full.replacen("arc :: R <", "arc :: Rar <", 1)
    } else {
        ret_full.clone()
    };
    let impl_ret = if gen_rar_version {
        ret.replacen("arc :: R <", "arc :: Rar <", 1)
    } else {
        ret
    };
    // Apple x86_64 returns values larger than two eightbytes indirectly.
    // cidre does not expose C long double, the only scalar exception.
    let x86_64_return_type = impl_ret.strip_prefix("-> ").unwrap_or("()");

    if gen_rar_version {
        impl_fn_name.push_str("_ar");
    }
    if super_call {
        assert!(
            !versions.any(),
            "#[objc::available] is not supported on msg_send_super"
        );
        // (& mut self, a: A) -> (sup: *const objc::Super, sel: *const c_void, a: A)
        let super_args = "(sup: *const objc::Super, sel: *const std::ffi::c_void";
        let fn_args = args
            .to_string()
            .replace("& mut self", "&mut self")
            .replace("& self", "&self");
        let fn_args = if class {
            fn_args.replacen('(', &format!("{super_args}, "), 1)
        } else if fn_args.contains("&mut self") {
            fn_args.replacen("(&mut self", super_args, 1)
        } else {
            fn_args.replacen("(&self", super_args, 1)
        };
        let fn_args = fn_args.replacen(", )", ")", 1);
        let (receiver, super_class) = if class {
            (
                "Self::cls_ptr() as *mut objc::Id",
                "Self::super_cls().meta_cls() as *const objc::Class<objc::Id>",
            )
        } else {
            (
                "self as *const Self as *mut objc::Id",
                "Self::super_cls() as *const _ as *const objc::Class<objc::Id>",
            )
        };
        let call_args = if vars.is_empty() {
            "sig(&sup, sel)".to_string()
        } else {
            format!("sig(&sup, sel, {vars})")
        };
        let (externs, fn_ptr) = if x86_64 {
            (
                "
        extern \"C\" {
            #[link_name = \"objc_msgSendSuper\"]
            fn msg_send_super();
            #[link_name = \"objc_msgSendSuper_stret\"]
            fn msg_send_super_stret();
        }",
                format!(
                    "if std::mem::size_of::<{x86_64_return_type}>() <= 16 {{
                msg_send_super as *const std::ffi::c_void
            }} else {{
                msg_send_super_stret as *const std::ffi::c_void
            }}"
                ),
            )
        } else {
            (
                "
        extern \"C\" {
            #[link_name = \"objc_msgSendSuper\"]
            fn msg_send_super();
        }",
                "msg_send_super as *const std::ffi::c_void".to_string(),
            )
        };
        flow.push_str(&format!(
            "
    {doc_alias}
    #[inline]
    {pre} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        {externs}
        extern \"C-unwind\" {{
            fn sel_registerName(name: *const i8) -> *const std::ffi::c_void;
        }}

        unsafe {{
            let sel = sel_registerName(c\"{sel}\".as_ptr());
            let sup = objc::Super {{
                receiver: {receiver},
                super_class: {super_class},
            }};
            let fn_ptr = {fn_ptr};
            let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        ));
    } else if x86_64 {
        flow.push_str(&format!(
            "
    {available}
    {doc_alias}
    #[inline]
    {pre} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend\"]
            fn msg_send();
            #[link_name = \"objc_msgSend_stret\"]
            fn msg_send_stret();
        }}
        extern \"C-unwind\" {{
            fn sel_registerName(name: *const i8) -> *const std::ffi::c_void;
        }}

        unsafe {{
            let x86_64_sel = sel_registerName(c\"{sel}\".as_ptr());
            let fn_ptr = if std::mem::size_of::<{x86_64_return_type}>() <= 16 {{
                msg_send as *const std::ffi::c_void
            }} else {{
                msg_send_stret as *const std::ffi::c_void
            }};
            let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        ));
        if versions.any() {
            let unsafe_str = if unsafe_already { "" } else { "unsafe" };
            let optional = if optional_already {
                String::new()
            } else {
                format!(
                    "
    /// `@selector({sel})` but dynamic
    /// use this function to check if object responds to selector
    #[inline]
    pub fn sel_{fn_name}() -> &'static objc::Sel {{
        unsafe {{ objc::sel_reg_name(c\"{sel}\".as_ptr()) }}
    }}
        "
                )
            };

            flow.push_str(&format!(
                "
    {optional}

    {unavailable}
    {doc_alias}
    #[inline]
    {pre} {unsafe_str} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend\"]
            fn msg_send();
            #[link_name = \"objc_msgSend_stret\"]
            fn msg_send_stret();
        }}
        extern \"C-unwind\" {{
            fn sel_registerName(name: *const i8) -> *const std::ffi::c_void;
        }}

        unsafe {{
            let x86_64_sel = sel_registerName(c\"{sel}\".as_ptr());
            let fn_ptr = if std::mem::size_of::<{x86_64_return_type}>() <= 16 {{
                msg_send as *const std::ffi::c_void
            }} else {{
                msg_send_stret as *const std::ffi::c_void
            }};
            let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
                "
            ));
        }
    } else {
        flow.push_str(&format!(
            "
    {available}
    {doc_alias}
    #[inline]
    {pre} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend${sel}\"]
            fn msg_send();
        }}

        unsafe {{
            let fn_ptr = msg_send as *const std::ffi::c_void;
            let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        ));
        if versions.any() {
            let unsafe_str = if unsafe_already { "" } else { "unsafe" };
            let optional = if optional_already {
                String::new()
            } else {
                format!(
                    "
    /// `@selector({sel})` but dynamic
    /// use this function to check if object responds to selector
    #[inline]
    pub fn sel_{fn_name}() -> &'static objc::Sel {{
        unsafe {{ objc::sel_reg_name(c\"{sel}\".as_ptr()) }}
    }}
        "
                )
            };

            flow.push_str(&format!(
                "
    {optional}

    {unavailable}
    {doc_alias}
    #[inline]
    {pre} {unsafe_str} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend${sel}\"]
            fn msg_send();
        }}

        let fn_ptr = msg_send as *const std::ffi::c_void;
        let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

        {call_args}
    }}
                "
            ));
        }
    };

    if gen_rar_version {
        if debug {
            println!("get rar version");
        }
        if option {
            flow.push_str(&format!(
                "

    {available}
    {doc_alias}
    #[inline]
    {pre} fn {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain_option({self_}{fn_name}_ar({vars}) )
    }}
                "
            ));
            if versions.any() {
                let unsafe_str = if unsafe_already { "" } else { "unsafe" };
                flow.push_str(&format!(
                    "

    {unavailable}
    {doc_alias}
    #[inline]
    /// Check availability with selector1 `Self::sel_{fn_name}()`
    {pre} {unsafe_str} fn {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain_option({self_}{fn_name}_ar({vars}) )
    }}
                      ",
                ));
            }
        } else {
            // not option
            flow.push_str(&format!(
                "

    {available}
    {doc_alias}
    #[inline]
    {pre} fn {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain({self_}{fn_name}_ar({vars}))
    }}
                ",
            ));
            if versions.any() {
                let unsafe_str = if unsafe_already { "" } else { "unsafe" };
                flow.push_str(&format!(
                    "

    {unavailable}
    {doc_alias}
    /// Check availability with selector `Self::sel_{fn_name}()`
    #[inline]
    {pre} {unsafe_str} fn {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain({self_}{fn_name}_ar({vars}))
    }}
                "
                ));
            }
        }
    }
    if debug {
        println!("{flow}");
    }

    flow.parse().unwrap()
}

/// Splits `(a: A, b: Option<B, C>)` into `["a: A", "b: Option<B, C>"]`.
fn split_top_level_args(stream: TokenStream) -> Vec<String> {
    let mut res = Vec::new();
    let mut current = TokenStream::new();
    let mut nesting = 0;
    for tt in stream {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == '<' => nesting += 1,
            TokenTree::Punct(ref p) if p.as_char() == '>' => nesting -= 1,
            TokenTree::Punct(ref p) if p.as_char() == ',' && nesting == 0 => {
                res.push(std::mem::take(&mut current).to_string());
                continue;
            }
            _ => {}
        }
        current.extend(std::iter::once(tt));
    }
    if !current.is_empty() {
        res.push(current.to_string());
    }
    res
}

fn fn_args_from_stream(stream: TokenStream) -> (bool, Vec<String>) {
    if stream.is_empty() {
        return (true, Vec::new());
    }
    let mut res = Vec::new();
    let mut pos = 0;
    let mut self_arg = false;
    let mut skip_ident = false;
    let mut lifetime = false;
    let mut nesting = 0;
    for s in stream.into_iter() {
        match s {
            TokenTree::Group(_) => {}
            TokenTree::Ident(ref i) => {
                if lifetime {
                    lifetime = false;
                    continue;
                }
                if !skip_ident {
                    let str = i.to_string();
                    if str == "mut" {
                        continue;
                    }
                    if pos == 0 && str == "self" {
                        self_arg = true;
                        continue;
                    }
                    res.push(str);
                    skip_ident = true;
                }
            }
            // #[objc::msg_send_debug(objectForKey:)]
            // pub fn get<'a>(&'a self, key: &K) -> Option<&'a V>;
            TokenTree::Punct(p) => match p.as_char() {
                '<' => nesting += 1,
                '>' => nesting -= 1,
                '\'' => lifetime = true,
                // '&' => skip_ident = true,
                ',' if nesting == 0 => {
                    pos += 1;
                    skip_ident = false;
                }
                ':' => skip_ident = true,
                _ => {}
            },
            TokenTree::Literal(ref _l) => {}
        }
    }
    (!self_arg, res)
}

#[proc_macro_attribute]
pub fn api_weak(_ts: TokenStream, body: TokenStream) -> TokenStream {
    let mut original_body = body.clone();
    let mut iter = body.into_iter();
    let mut versions = None;
    let mut tokens: Vec<TokenTree> = Vec::new();
    let mut vars: Vec<(Versions, String, String)> = Vec::new(); // Version, Name, Type
    while let Some(t) = iter.next() {
        match t {
            // extern "C" {
            TokenTree::Group(ref p) if p.delimiter() == Delimiter::Brace => {
                let mut group = p.stream().into_iter();
                while let Some(t) = group.next() {
                    match t {
                        TokenTree::Punct(ref p) if p.as_char() == ':' => {
                            if let Some(version) = versions.take() {
                                let var_name = tokens.last().unwrap().to_string();
                                let _t = group.next().unwrap(); // &
                                let _t = group.next().unwrap(); // '
                                if let TokenTree::Ident(ident) = group.next().unwrap() {
                                    assert_eq!(ident.to_string(), "static");
                                    let mut ty = Vec::new();
                                    while let Some(t) = group.next() {
                                        match t {
                                            TokenTree::Punct(ref p) if p.as_char() == ';' => break,
                                            t => ty.push(t),
                                        }
                                    }
                                    let ty = if ty.len() == 1 {
                                        ty[0].to_string()
                                    } else {
                                        TokenStream::from_iter(ty).to_string()
                                    };
                                    vars.push((version, var_name, ty));
                                }
                            }
                        }
                        TokenTree::Punct(ref p) if p.as_char() == ';' => {
                            tokens.clear();
                            versions = None;
                        }
                        TokenTree::Group(ref p) if p.delimiter() == Delimiter::Parenthesis => {
                            if let Some(version) = versions.take() {
                                let mut ty: Vec<TokenTree> =
                                    TokenStream::from_str("extern \"C\" fn ")
                                        .unwrap()
                                        .into_iter()
                                        .collect();
                                let name = tokens.pop().unwrap();
                                let var_name = name.to_string();
                                // ty.push(name);
                                ty.push(t.clone());
                                while let Some(t) = group.next() {
                                    match t {
                                        TokenTree::Punct(ref p) if p.as_char() == ';' => break,
                                        t => ty.push(t),
                                    }
                                }
                                let ty = if ty.len() == 1 {
                                    ty[0].to_string()
                                } else {
                                    TokenStream::from_iter(ty).to_string()
                                };

                                // println!("ty: {ty:?}");

                                vars.push((version, var_name, ty));
                            }
                        }
                        TokenTree::Group(ref p) if p.delimiter() == Delimiter::Bracket => {
                            let mut attr = p.stream().into_iter();
                            while let Some(ref ident) = attr.next() {
                                match ident {
                                    // TokenTree::Group(_) => todo!(),
                                    TokenTree::Ident(i) => {
                                        let st = i.to_string();
                                        match st.as_str() {
                                            // api::available
                                            "api" => {
                                                attr.next();
                                                attr.next();
                                                continue;
                                            }
                                            // objc::available
                                            "objc" => {
                                                attr.next();
                                                attr.next();
                                                continue;
                                            }
                                            // direct available
                                            "available" => {
                                                if let Some(TokenTree::Group(g)) = attr.next() {
                                                    versions =
                                                        Some(Versions::from_stream(g.stream()));
                                                    // println!("features {features:?}");
                                                } else {
                                                    break;
                                                }
                                                break;
                                            }
                                            _ => break,
                                        }
                                    }
                                    _ => break, // TokenTree::Punct(_) => todo!(),
                                                // TokenTree::Literal(_) => todo!(),
                                }
                            }

                            // println!("found {p:?}")
                        }
                        _ => {}
                    }
                    tokens.push(t);
                    // println!("t: {t:?}")
                }
            }
            _x => {
                // println!("x: {x:?}")
            }
        }
    }
    // println!("{vars:?}");
    let vars = vars
        .iter()
        .map(|(version, name, ty)| {
            let upper_name = upper_case(name);
            let availability = version.unavailable_cfg();
            format!(
            "{availability}\nstatic {upper_name}: api::DlSym<{ty}> = api::DlSym::new(c\"{name}\");"
        )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let stream = TokenStream::from_str(&vars).unwrap();
    original_body.extend(stream);
    original_body
}

#[derive(Default, Debug, Copy, Clone)]
struct Version(u32, u32);

impl Version {
    fn from_str(str: &str) -> Option<Self> {
        if let Some((major, minor)) = str.split_once('.') {
            Some(Self(str::parse(major).unwrap(), str::parse(minor).unwrap()))
        } else if let Some((major, minor)) = str.split_once('_') {
            Some(Self(str::parse(major).unwrap(), str::parse(minor).unwrap()))
        } else {
            None
        }
    }
}

#[derive(Default, Debug)]
struct Versions {
    macos: Option<Version>,
    ios: Option<Version>,
    tvos: Option<Version>,
    watchos: Option<Version>,
    visionos: Option<Version>,
    maccatalyst: Option<Version>,
}

impl Versions {
    fn any(&self) -> bool {
        self.macos.is_some()
            || self.ios.is_some()
            || self.tvos.is_some()
            || self.watchos.is_some()
            || self.visionos.is_some()
            || self.maccatalyst.is_some()
    }

    fn available_cfg_ts(&self) -> TokenStream {
        TokenStream::from_str(&self.available_cfg()).unwrap()
    }

    fn available_cfg(&self) -> String {
        let mut vec = Vec::with_capacity(6);
        if let Some(v) = self.macos {
            vec.push(format!(
                "all(target_os=\"macos\", feature=\"macos_{}_{}\")",
                v.0, v.1
            ));
        }
        if let Some(v) = self.ios {
            vec.push(format!(
                "all(target_os=\"ios\", feature=\"ios_{}_{}\")",
                v.0, v.1
            ));
        }
        if let Some(v) = self.tvos {
            vec.push(format!(
                "all(target_os=\"tvos\", feature=\"tvos_{}_{}\")",
                v.0, v.1,
            ));
        }
        if let Some(v) = self.watchos {
            vec.push(format!(
                "all(target_os=\"watchos\", feature=\"watchos_{}_{}\")",
                v.0, v.1,
            ));
        }
        if let Some(v) = self.visionos {
            vec.push(format!(
                "all(target_os=\"visionos\", feature=\"visionos_{}_{}\")",
                v.0, v.1
            ));
        }
        if let Some(v) = self.maccatalyst {
            vec.push(format!(
                "all(target_os=\"ios\", target_abi=\"macabi\", feature=\"maccatalyst_{}_{}\")",
                v.0, v.1
            ));
        }

        match vec.len() {
            0 => String::new(),
            1 => format!("#[cfg({})]\n", vec[0]),
            _ => format!("#[cfg(any({}))]\n", vec.join(", ")),
        }
    }
    fn unavailable_cfg_ts(&self) -> TokenStream {
        TokenStream::from_str(&self.unavailable_cfg()).unwrap()
    }

    fn unavailable_cfg(&self) -> String {
        let mut vec = Vec::with_capacity(6);
        if let Some(v) = self.macos {
            vec.push(format!(
                "all(target_os=\"macos\", not(feature=\"macos_{}_{}\"))",
                v.0, v.1
            ));
        }
        if let Some(v) = self.ios {
            vec.push(format!(
                "all(target_os=\"ios\", not(feature=\"ios_{}_{}\"))",
                v.0, v.1
            ));
        }
        if let Some(v) = self.tvos {
            vec.push(format!(
                "all(target_os=\"tvos\", not(feature=\"tvos_{}_{}\"))",
                v.0, v.1
            ));
        }
        if let Some(v) = self.watchos {
            vec.push(format!(
                "all(target_os=\"watchos\", not(feature=\"watchos_{}_{}\"))",
                v.0, v.1
            ));
        }
        if let Some(v) = self.visionos {
            vec.push(format!(
                "all(target_os=\"visionos\", not(feature=\"visionos_{}_{}\"))",
                v.0, v.1
            ));
        }
        if let Some(v) = self.maccatalyst {
            vec.push(format!(
                "all(target_os=\"ios\", target_abi=\"macabi\", not(feature=\"maccatalyst_{}_{}\"))",
                v.0, v.1
            ));
        }

        match vec.len() {
            0 => String::new(),
            1 => format!("#[cfg({})]\n", vec[0]),
            _ => format!("#[cfg(any({}))]\n", vec.join(", ")),
        }
    }
    fn available_doc_ts(&self) -> TokenStream {
        TokenStream::from_str(&self.available_doc()).unwrap()
    }

    fn available_doc(&self) -> String {
        let mut vec = Vec::with_capacity(6);
        if let Some(v) = self.macos {
            vec.push(format!("macos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.ios {
            vec.push(format!("ios_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.tvos {
            vec.push(format!("tvos_{}_{}", v.0, v.1,));
        }
        if let Some(v) = self.watchos {
            vec.push(format!("watchos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.visionos {
            vec.push(format!("visionos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.maccatalyst {
            vec.push(format!("maccatalyst_{}_{}", v.0, v.1));
        }

        match vec.len() {
            0 => String::new(),
            1 => format!("/// # Availability\n/// {0}", vec[0]),
            _ => format!("/// # Availability\n/// {0}", vec.join(", ")),
        }
    }
    fn unavailable_doc_ts(&self) -> TokenStream {
        TokenStream::from_str(&self.unavailable_doc()).unwrap()
    }

    fn unavailable_doc(&self) -> String {
        let mut vec = Vec::with_capacity(6);
        if let Some(v) = self.macos {
            vec.push(format!("macos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.ios {
            vec.push(format!("ios_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.tvos {
            vec.push(format!("tvos_{}_{}", v.0, v.1,));
        }
        if let Some(v) = self.watchos {
            vec.push(format!("watchos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.visionos {
            vec.push(format!("visionos_{}_{}", v.0, v.1));
        }
        if let Some(v) = self.maccatalyst {
            vec.push(format!("maccatalyst_{}_{}", v.0, v.1));
        }

        match vec.len() {
            0 => String::new(),
            1 => format!("/// # Availability\n/// Not {0}", vec[0]),
            _ => format!("/// # Availability\n/// Not {0}", vec.join(", ")),
        }
    }

    fn from_stream(versions: TokenStream) -> Self {
        let mut iter = versions.into_iter();
        let mut versions = Self::default();
        while let Some(t) = iter.next() {
            let target_os = match t {
                TokenTree::Ident(ident) => ident.to_string(),
                _ => panic!("Unexpected token {t:?}"),
            };
            let Some(TokenTree::Punct(ident)) = iter.next() else {
                panic!("Expecting = ");
            };

            assert_eq!(ident, '=', "expecting =");

            let Some(TokenTree::Literal(val)) = iter.next() else {
                panic!("expecting version");
            };

            let v = Version::from_str(&val.to_string());
            match target_os.as_str() {
            "macos" => versions.macos = v,
            "ios" => versions.ios = v,
            "tvos" => versions.tvos = v,
            "watchos" => versions.watchos = v,
            "visionos" => versions.visionos = v,
            "maccatalyst" => versions.maccatalyst = v,
            t => panic!("Unsupported platform. Platform should be macos, ios, watchos, visionos or maccatalyst. Found {t:?}"),
        };

            if let Some(TokenTree::Punct(p)) = iter.next() {
                assert_eq!(p, ',', "expect ,");
            };
        }

        versions
    }

    fn from_doc_str(str: &str) -> Self {
        let mut res = Self::default();
        for str in str.split_whitespace() {
            for str in str.split_terminator(',') {
                if str.starts_with("macos_") {
                    res.macos = Version::from_str(&str[6..]);
                } else if str.starts_with("ios_") {
                    res.ios = Version::from_str(&str[4..]);
                } else if str.starts_with("tvos_") {
                    res.tvos = Version::from_str(&str[5..]);
                } else if str.starts_with("watchos_") {
                    res.watchos = Version::from_str(&str[8..]);
                } else if str.starts_with("visionos_") {
                    res.visionos = Version::from_str(&str[8..]);
                } else if str.starts_with("maccatalyst_") {
                    res.maccatalyst = Version::from_str(&str[12..]);
                }
            }
        }
        res
    }
}

#[proc_macro_attribute]
pub fn api_available(versions: TokenStream, body: TokenStream) -> TokenStream {
    let versions = Versions::from_stream(versions);
    let available = versions.available_cfg_ts();
    let available_doc = versions.available_doc_ts();
    let unavailable = versions.unavailable_cfg_ts();
    let unavailable_doc = versions.unavailable_doc_ts();
    if available.is_empty() {
        return body;
    }

    let mut no_args = false;
    let mut no_body = false;

    let mut available = Some(available);
    let mut available_doc = Some(available_doc);
    let mut unavailable = Some(unavailable);
    let mut unavailable_doc = Some(unavailable_doc);
    let mut res = Vec::new();
    let mut maybe_res: Vec<TokenTree> = Vec::new();
    let mut fn_index = 0usize;
    let mut unsafe_already = false;

    let mut body_iter = body.into_iter();

    while let Some(t) = body_iter.next() {
        if available.is_some() {
            res.extend(available.take().unwrap());
        }
        if unavailable.is_some() {
            maybe_res.extend(unavailable.take().unwrap());
        }
        if available_doc.is_some() {
            match t {
                TokenTree::Ident(ref _i) => {
                    let doc = available_doc.take().unwrap();
                    res.extend(doc);
                }
                _ => {}
            }
        }
        if unavailable_doc.is_some() {
            match t {
                TokenTree::Ident(ref _i) => {
                    let doc = unavailable_doc.take().unwrap();
                    maybe_res.extend(doc);
                }
                _ => {}
            }
        }
        match t {
            TokenTree::Punct(ref p) if p.as_char() == ';' => {
                no_body = true;
            }
            TokenTree::Ident(ref ident) => match ident.to_string().as_str() {
                "fn" => {
                    fn_index = maybe_res.len();
                }
                "unsafe" => {
                    unsafe_already = true;
                }
                "define_cls" => {
                    let token = TokenTree::Ident(Ident::new("define_weak_cls", t.span()));
                    maybe_res.push(token);
                    res.push(t.clone());
                    while let Some(t) = body_iter.next() {
                        maybe_res.push(t.clone());
                        res.push(t);
                    }
                    break;
                }
                "define_cls_init" => {
                    let token = TokenTree::Ident(Ident::new("define_weak_cls_init", t.span()));
                    maybe_res.push(token);
                    res.push(t.clone());
                    while let Some(t) = body_iter.next() {
                        maybe_res.push(t.clone());
                        res.push(t);
                    }
                    break;
                }
                _ => {}
            },
            _ => {}
        }

        maybe_res.push(t.clone());

        if let TokenTree::Group(ref g) = t {
            // function without args ()
            if g.delimiter() == Delimiter::Parenthesis {
                no_args = g.stream().is_empty();
                // we are in function.
            }
            // function body {}
            if g.delimiter() == Delimiter::Brace {
                if no_args && try_replace_return(&mut maybe_res) {}
                let mut make_result_optional = false;
                if try_replace_fn(&mut maybe_res, &mut make_result_optional) {
                    if make_result_optional {
                        let mut i = maybe_res.len() - 2;

                        while i > 0 {
                            if let TokenTree::Punct(ref p) = maybe_res[i] {
                                if p.as_char() == '-' && p.spacing() == Spacing::Joint {
                                    i += 2;
                                    break;
                                }
                            }
                            i -= 1;
                        }

                        maybe_res.insert(i, TokenTree::Punct(Punct::new('<', Spacing::Alone)));
                        maybe_res
                            .insert(i, TokenTree::Ident(Ident::new("Option", Span::call_site())));
                        maybe_res.insert(
                            maybe_res.len() - 2,
                            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
                        );
                    }
                } else {
                    if !unsafe_already {
                        maybe_res.insert(
                            fn_index,
                            TokenTree::Ident(Ident::new("unsafe", Span::call_site())),
                        );
                    }
                }
            }
        }

        res.push(t);
    }
    if !no_body {
        res.extend(maybe_res);
    }

    TokenStream::from_iter(res)
}

fn try_replace_fn(tokens: &mut Vec<TokenTree>, make_result_optional: &mut bool) -> bool {
    let Some(TokenTree::Group(ref g)) = tokens.last() else {
        return false;
    };
    if g.delimiter() != Delimiter::Brace {
        return false;
    }
    // check fn body. it should be function call.
    let mut body_stream = g.stream().into_iter();

    match body_stream.next().unwrap() {
        TokenTree::Group(scope) => {
            if scope.delimiter() == Delimiter::Brace {
                *make_result_optional = true;
                return true;
            }
        }
        TokenTree::Ident(ident) => match ident.to_string().as_str() {
            "unsafe" => {
                let Some(TokenTree::Group(g)) = body_stream.next() else {
                    return false;
                };
                if g.delimiter() != Delimiter::Brace {
                    return false;
                }
                let mut block = g.stream().into_iter();
                let Some(TokenTree::Ident(ident)) = block.next() else {
                    return false;
                };
                let Some(TokenTree::Group(args)) = block.next() else {
                    return false;
                };
                if args.delimiter() != Delimiter::Parenthesis {
                    return false;
                }
                if block.next().is_none() {
                    tokens.pop();
                    let var = upper_case(&ident.to_string());
                    let stream = TokenStream::from_str(&format!(
                        "{{ unsafe {{ {var}.get_fn().unwrap(){} }} }}",
                        args.to_string()
                    ))
                    .unwrap();
                    tokens.extend(stream);

                    return true;
                }
            }
            "Self" => {
                // `Self::alloc()`

                let mut new_body = Vec::new();
                new_body.push(TokenTree::Ident(ident));
                let Some(TokenTree::Punct(p)) = body_stream.next() else {
                    return false;
                };

                if p.as_char() != ':' {
                    return false;
                }
                new_body.push(TokenTree::Punct(p));
                let Some(TokenTree::Punct(p)) = body_stream.next() else {
                    return false;
                };

                if p.as_char() != ':' {
                    return false;
                }
                new_body.push(TokenTree::Punct(p));
                let Some(TokenTree::Ident(ident)) = body_stream.next() else {
                    return false;
                };

                if ident.to_string() != "alloc" {
                    return false;
                };
                new_body.push(TokenTree::Ident(ident));

                let Some(TokenTree::Group(g)) = body_stream.next() else {
                    return false;
                };

                if g.delimiter() != Delimiter::Parenthesis {
                    return false;
                }
                new_body.push(TokenTree::Group(g));

                // Some(Self::alloc()?.)
                new_body.push(TokenTree::Punct(Punct::new('?', Spacing::Alone)));
                while let Some(t) = body_stream.next() {
                    new_body.push(t)
                }
                let s = TokenStream::from_iter(new_body.drain(..));
                new_body.push(TokenTree::Ident(Ident::new("Some", Span::call_site())));
                let g = TokenTree::Group(Group::new(Delimiter::Parenthesis, s));
                new_body.push(g);
                let s = TokenStream::from_iter(new_body.drain(..));
                let g = TokenTree::Group(Group::new(Delimiter::Brace, s));

                tokens.pop();
                tokens.push(g);

                *make_result_optional = true;
                return true;
                // println!("found");
            }
            _ => return false,
        },

        _ => return false,
    };
    false
}

fn try_replace_return(tokens: &mut Vec<TokenTree>) -> bool {
    let mut idx = tokens.len() - 1;
    let mut has_static = false;
    // going reverse till `&'static` or ()
    while idx > 0 {
        match &tokens[idx] {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Parenthesis => return false,
            TokenTree::Ident(i) if i.to_string() == "static" => has_static = true,
            TokenTree::Punct(p) if p.as_char() == '\'' => break,
            // TokenTree::Literal(_) => todo!(),
            _ => {}
        }
        idx -= 1;
    }

    if !has_static {
        return false;
    }

    let mut body_stream = {
        let Some(TokenTree::Group(ref g)) = tokens.last() else {
            return false;
        };

        // check fn body. it should return single var, no function call.
        g.stream().into_iter()
    };

    // check for
    // unsafe { VAR }
    // or
    // unsafe { fn_call(args) }

    let var = match body_stream.next().unwrap() {
        TokenTree::Ident(ident) => match ident.to_string().as_str() {
            "unsafe" => {
                let TokenTree::Group(g) = body_stream.next().unwrap() else {
                    return false;
                };
                if g.delimiter() != Delimiter::Brace {
                    return false;
                }
                let mut block = g.stream().into_iter();
                let TokenTree::Ident(ident) = block.next().unwrap() else {
                    return false;
                };
                if let Some(TokenTree::Group(_g)) = block.next() {
                    return false;
                };
                ident.to_string()
            }
            x => x.to_string(),
        },

        _ => return false,
    };

    idx -= 1; // &
    tokens.pop(); // {}
    let stream = TokenStream::from_iter(tokens.drain(idx..));
    let ty = stream.to_string();
    let stream = TokenStream::from_str(&format!("Option<{ty}>")).unwrap();
    tokens.extend(stream);
    let var = upper_case(&var.to_string());
    let stream = TokenStream::from_str(&format!("{{ unsafe {{ {var}.get_var() }} }}")).unwrap();
    tokens.extend(stream);
    true
}

// Super simple, but stable upper_case impl
fn upper_case(str: &str) -> String {
    let len = str.len();
    let mut res = Vec::<u8>::with_capacity(len + 10);
    let bytes = str.as_bytes();
    let mut was_lowercase = false;
    for ch in bytes {
        let is_upper = ch.is_ascii_uppercase();
        if was_lowercase && is_upper {
            res.push(b'_');
        }
        res.push(ch.to_ascii_uppercase());
        was_lowercase = !is_upper;
    }

    String::from_utf8(res).unwrap()
}

/// Whether `sel` belongs to an Objective-C method family.
///
/// A selector is in family `F` when, after any leading underscores, it begins
/// with `F` and the next character is not a lowercase letter. That word
/// boundary is what separates `initWithFoo:`/`initFileURLWithPath:` (the `init`
/// family, which returns at +1) from `initialize` (which does not).
fn in_method_family(sel: &str, family: &str) -> bool {
    match sel.trim_start_matches('_').strip_prefix(family) {
        Some(rest) => !rest.starts_with(|c: char| c.is_ascii_lowercase()),
        None => false,
    }
}

/// Whether the callee hands back an object it already owns, so the caller must
/// not retain it again.
///
/// These are Objective-C's returns-retained method families. `alloc` is absent
/// only because it yields `arc::A` rather than `arc::R`, so it never reaches
/// the retaining path.
///
/// A binding can also opt out of the retain by spelling its return type
/// `arc::Retained<T>` instead of the `arc::R<T>` alias, which is how cidre
/// marks a method annotated `CF_RETURNS_RETAINED` in its header. Getting that
/// spelling wrong on a +0 method releases an object the caller never owned, so
/// only use it where the header says so.
fn returns_retained(sel: &str) -> bool {
    const RETAINING_FAMILIES: [&str; 4] = ["init", "new", "mutableCopy", "copy"];
    // `mutableCopy` is tested before `copy` so the longer name wins.
    RETAINING_FAMILIES
        .iter()
        .any(|family| in_method_family(sel, family))
}

// fn is_upper_case(str: &str) -> bool {
//     let bytes = str.as_bytes();
//     for ch in bytes {
//         if ch != &b'_' {
//             if !ch.is_ascii_uppercase() {
//                 return false;
//             }
//         }
//     }
//     true
// }
