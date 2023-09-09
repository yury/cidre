use std::borrow::Cow;

/// This is all dirty hacks. We need to reimplement it with syn and quote
use proc_macro::{Delimiter, Group, TokenStream, TokenTree};

enum ObjcAttr {
    Optional,
    MsgSend(String),
}

fn read_objc_attr(group: Group) -> Option<ObjcAttr> {
    let mut iter = group.stream().into_iter();
    let Some(TokenTree::Ident(ident)) = iter.next() else {
        return None;
    };

    if ident.to_string() != "objc" {
        return None;
    }

    let Some(TokenTree::Punct(p)) = iter.next() else {
        return None;
    };

    assert_eq!(p.as_char(), ':');

    let Some(TokenTree::Punct(p)) = iter.next() else {
        return None;
    };

    assert_eq!(p.as_char(), ':');

    while let Some(tt) = iter.next() {
        match tt {
            TokenTree::Group(v) => panic!("didnt expect group {v}"),
            TokenTree::Ident(v) => {
                if v.to_string().eq("optional") {
                    return Some(ObjcAttr::Optional);
                } else if v.to_string().eq("msg_send") {
                    let Some(TokenTree::Group(a)) = iter.next() else {
                        return None;
                    };
                    let sel = a.stream().to_string().replace([' ', '\n'], "");
                    return Some(ObjcAttr::MsgSend(sel));
                }
                return None;
            }
            TokenTree::Punct(v) => panic!("did't expect punct {v}"),
            TokenTree::Literal(v) => panic!("did't expect literal {v}"),
        }
    }

    panic!("Unexpected attribute")
}

// fn sel_args_count(sel: TokenStream) -> usize {
//     sel.into_iter()
//         .filter(|t| matches!(t, TokenTree::Punct(v) if v.as_char() == ':'))
//         .count()
// }

fn get_fn_args(group: TokenStream, class: bool, debug: bool) -> Vec<String> {
    let mut prev = None;
    let mut vars = vec![];
    if debug {
        println!("tokens {:?}", group);
    }

    let mut can_be_name = class;
    let mut level = 0;
    for t in group.into_iter() {
        match t {
            TokenTree::Ident(i) => {
                prev = Some(i);
            }
            TokenTree::Punct(p) => match p.as_char() {
                ':' if can_be_name && level == 0 => {
                    if let Some(id) = prev.take() {
                        vars.push(id.to_string())
                    }
                    can_be_name = false;
                }
                '>' => level -= 1,
                '<' => level += 1,
                ',' => can_be_name = true,
                _ => prev = None,
            },
            _ => prev = None,
        }
    }
    vars
}

#[proc_macro_attribute]
pub fn rar_retain(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, true, false, false)
}

#[proc_macro_attribute]
pub fn cls_rar_retain(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, true, true, false)
}

#[proc_macro_attribute]
pub fn msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, false, false)
}

/// Should generate static fn sel_xxx function that gets selector.
/// So user can check selector with is_reponds_to_sel
#[proc_macro_attribute]
pub fn optional(_sel: TokenStream, func: TokenStream) -> TokenStream {
    let mut iter = func.clone().into_iter();
    let Some(TokenTree::Punct(p)) = iter.next() else {
        panic!("expect #[objc::msg_send(...)]")
    };

    if p.as_char() != '#' {
        panic!("expect #[objc::msg_send(...)]")
    }
    let Some(TokenTree::Group(g)) = iter.next() else {
        panic!("expect #[objc::msg_send(...)]")
    };

    let Some(ObjcAttr::MsgSend(extern_name)) = read_objc_attr(g) else {
        panic!("expect #[objc::msg_send(...)]")
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
    fn sel_{fn_name}() -> Option<&'static objc::Sel> {{
        unsafe {{ Some(objc::sel_registerName(b\"{extern_name}\\0\".as_ptr())) }}
    }}
        "
    )
    .parse()
    .unwrap();

    let mut func = func;
    func.extend(getter.into_iter());
    func
}

#[proc_macro_attribute]
pub fn obj_trait(_args: TokenStream, tr: TokenStream) -> TokenStream {
    let mut original_trait = tr.clone();

    let iter = tr.into_iter();
    let mut before_trait_name_tokens = vec![];
    let mut after_trait_name_tokens = vec![];
    let mut trait_name = Cow::Borrowed("");
    let mut expect_trait_name = false;
    let mut is_optional = false;
    let mut skip = false;
    let mut sel = "".to_string();
    let mut fn_name; // = "".to_string();
    let mut generics = Vec::new();
    //let mut fn_args = Vec::new();
    let mut fn_args_str; // = "".to_string();
    let mut result = Vec::new();
    let mut fn_body = Cow::Borrowed("");

    let mut impl_trait_functions = vec![];
    let mut has_optionals = false;
    let mut fn_names = vec![];

    for token in iter {
        let collection = if trait_name.is_empty() {
            &mut before_trait_name_tokens
        } else {
            &mut after_trait_name_tokens
        };
        match &token {
            TokenTree::Group(g) => {
                let mut iter = g.stream().into_iter();
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
                                        TokenTree::Group(ref g)
                                            if g.delimiter() == Delimiter::Brace =>
                                        {
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
                                    fn_args_str = fn_args_str.replacen(
                                        "(& self",
                                        "(&self, _cmd: Option<&objc::Sel>",
                                        1,
                                    );

                                    fn_args_str = fn_args_str.replacen(
                                        "(& mut self",
                                        "(&mut self, _cmd: Option<&objc::Sel>",
                                        1,
                                    );
                                    Cow::Owned(format!("Some(unsafe {{ objc::sel_registerName(b\"{sel}\\0\".as_ptr()) }})"))
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
                                    Cow::Owned(
                                        TokenStream::from_iter(result.clone().into_iter())
                                            .to_string(),
                                    )
                                };

                                let gen = if generics.is_empty() {
                                    Cow::Borrowed("")
                                } else {
                                    Cow::Owned(
                                        TokenStream::from_iter(generics.clone().into_iter())
                                            .to_string(),
                                    )
                                };

                                let impl_fn = if skip {
                                    format!(
                                        "
    {ext}fn {fn_name}{gen}{fn_args_str}{ret} {fn_body}

                                    "
                                    )
                                } else {
                                    fn_names.push(fn_name.clone());
                                    format!(
                                        "
    {ext}fn impl_{fn_name}{gen}{fn_args_str}{ret} {fn_body}

                                    "
                                    )
                                };

                                impl_trait_functions.push(impl_fn);

                                if !is_optional && !skip {
                                    let impl_sel = format!(
                                        "
    fn sel_{fn_name}() -> Option<&'static objc::Sel> {{ {register_sel} }}
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
                                match read_objc_attr(g) {
                                    Some(ObjcAttr::Optional) => {
                                        has_optionals = true;
                                        is_optional = true;
                                    }
                                    Some(ObjcAttr::MsgSend(s)) => sel = s,
                                    None => continue,
                                }
                            }
                            _ => panic!("other char '{p}'"),
                        },
                        TokenTree::Literal(l) => println!("lit {l}"),
                    }
                }
            }
            TokenTree::Ident(i) => {
                let str = i.to_string();
                if expect_trait_name {
                    expect_trait_name = false;
                    trait_name = Cow::Owned(str);
                    continue;
                }
                collection.push(token.clone());
                if str == "trait" {
                    expect_trait_name = true;
                }
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => collection.push(token),
        }
    }

    let pre = TokenStream::from_iter(before_trait_name_tokens.into_iter()).to_string();
    let obj_trait_name = format!("{trait_name}Impl");
    //let after = TokenStream::from_iter(after_trait_name_tokens.into_iter()).to_string();
    let fns = impl_trait_functions.join("\n");

    let add_methods = if has_optionals {
        Cow::Borrowed("fn cls_add_methods<O: objc::Obj>(cls: &objc::Class<O>);")
    //" { panic!(\"use #[objc::cls_builder]\") }".to_string()
    } else {
        Cow::Owned(add_methods_fn(&fn_names))
    };

    let code = format!(
        "

{pre} {obj_trait_name}: {trait_name} {{
    {fns}
    {add_methods}
}}
        "
    );

    let ts: TokenStream = code.parse().unwrap();

    original_trait.extend(ts);
    original_trait
}

fn add_methods_fn(fns: &[String]) -> String {
    let mut res = "
    fn cls_add_methods<O: objc::Obj>(cls: &objc::Class<O>) {
        let cls: &objc::Class<objc::Id> = unsafe { std::mem::transmute(cls) };
        "
    .to_string();
    for f in fns {
        let add = format!(
            "
    if let Some(sel) = Self::sel_{f}() {{
        unsafe {{
            let imp: extern \"C\" fn() = std::mem::transmute(Self::impl_{f} as *const u8);
            objc::class_addMethod(cls, sel, imp, std::ptr::null());
        }}
    }}
            "
        );
        res.push_str(&add);
    }
    res.push_str("\n}");
    res
}

#[proc_macro_attribute]
pub fn add_methods(_args: TokenStream, tr_impl: TokenStream) -> TokenStream {
    let mut tokens = vec![];

    let iter = tr_impl.into_iter();
    let mut fns = vec![];

    for tt in iter {
        match tt {
            TokenTree::Group(g) => {
                let mut body = g.stream().into_iter();
                while let Some(tt) = body.next() {
                    match tt {
                        TokenTree::Ident(i) if i.to_string().eq("fn") => {
                            let Some(TokenTree::Ident(f)) = body.next() else {
                                panic!("expected function name");
                            };
                            fns.push(f.to_string().replacen("impl_", "", 1));
                        }
                        _ => continue,
                    }
                }
                let imp: TokenStream = add_methods_fn(&fns).parse().unwrap();
                let mut stream = g.stream();
                stream.extend(imp);
                let g = Group::new(g.delimiter(), stream);
                tokens.push(TokenTree::Group(g));
            }
            _ => tokens.push(tt),
        }
    }

    // println!("fns {fns:?}");

    TokenStream::from_iter(tokens.into_iter())
}

#[proc_macro_attribute]
pub fn msg_send_debug(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, false, true)
}

#[proc_macro_attribute]
pub fn cls_msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, true, false)
}

#[proc_macro_attribute]
pub fn cls_msg_send_debug(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, true, true)
}

fn gen_msg_send(
    sel: TokenStream,
    func: TokenStream,
    retain: bool,
    class: bool,
    debug: bool,
) -> TokenStream {
    let extern_name = sel.to_string().replace([' ', '\n'], "");
    // let args_count = sel_args_count(sel);
    // reduce allocations and just search for ':'
    let args_count = extern_name.matches(':').count();
    // let _is_init = extern_name.starts_with("init");

    let mut iter = func.into_iter();
    let mut pre: Vec<String> = Vec::with_capacity(3);

    for t in iter.by_ref() {
        match t {
            TokenTree::Ident(v) => {
                let s = v.to_string();
                if s == "fn" {
                    pre.push(s);
                    break;
                } else {
                    pre.push(s);
                }
            }
            _ => continue,
        }
    }
    let Some(TokenTree::Ident(fn_name)) = iter.next() else {
        panic!("expected function name");
    };

    let fn_name = fn_name.to_string();
    if extern_name.starts_with("new") && fn_name.ends_with("_ar") {
        panic!("can't use _ar functions with methods started with `new`. See #3");
    }
    let mut generics = Vec::new();

    let args = loop {
        let Some(tt) = iter.next() else {
            panic!("need more tokens");
        };
        match tt {
            TokenTree::Group(args) => break args,
            _ => generics.push(tt),
        }
    };
    let gen = if fn_name.ends_with("_ar") {
        if generics.is_empty() {
            Cow::Borrowed("<'ar>")
        } else {
            let gen = TokenStream::from_iter(generics.into_iter()).to_string();
            Cow::Owned(gen.replacen('<', "<'ar,", 1))
        }
    } else {
        Cow::Owned(TokenStream::from_iter(generics.into_iter()).to_string())
    };

    let ts = TokenStream::from_iter(iter);
    let mut ret = ts.to_string();
    assert_eq!(ret.pop().expect(";"), ';');
    let ret_full = ret.to_string();
    if let Some((a, _)) = ret.split_once("where") {
        ret = format!("{a}")
    }
    let option = ret_full.contains("-> Option");

    let fn_args = args.to_string();
    if debug {
        println!("ret: {ret}");
        println!("fn_args: {fn_args}");
    }

    let vars = get_fn_args(args.stream(), class, debug);
    let fn_args_count = vars.len();
    if retain {
        assert_eq!(args_count, 0, "retain should not have selector args");
    } else {
        assert_eq!(
            fn_args_count, args_count,
            "left: fn_args_count, right: sel_args_count"
        );
    }
    let pre = pre.join(" ");
    let vars = if vars.is_empty() {
        Cow::Borrowed("")
    } else {
        Cow::Owned(vars.join(", "))
    };

    let (mut fn_args, mut call_args) = if fn_args_count == 0 {
        let fn_args = fn_args
            .replacen("( &", "(id: &", 1)
            .replacen("self", "Self", 1);
        (fn_args, "sig(self)".to_string())
    } else {
        let fn_args = fn_args
            .replacen('(', "(id:", 1)
            .replace("self", "Self, imp: *const std::ffi::c_void");
        (fn_args, format!("sig(self, std::ptr::null(), {vars})"))
    };

    if class {
        if fn_args_count == 0 {
            fn_args = fn_args.replacen('(', "(cls: *const std::ffi::c_void", 1);
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        } else {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, id: *const std::ffi::c_void,",
                1,
            );
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        }
    }

    let flow = if retain {
        let self_ = if class { "Self::" } else { "self." };
        if option {
            format!(
                "
    #[inline]
    {pre} {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain_option({self_}{fn_name}_ar({vars}) )
    }}
                "
            )
        } else {
            format!(
                "
    #[inline]
    {pre} {fn_name}{gen}{args}{ret_full} {{
        arc::rar_retain({self_}{fn_name}_ar({vars}))
    }}
                "
            )
        }
    } else {
        format!(
            "
    #[inline]
    {pre} {fn_name}{gen}{args}{ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend${extern_name}\"]
            fn msg_send();
        }}

        unsafe {{
            let fn_ptr = msg_send as *const std::ffi::c_void;
            let sig: extern \"C\" fn{fn_args} {ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        )
    };
    if debug {
        println!("{flow}");
    }

    flow.parse().unwrap()
}
