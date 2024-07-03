/// This is all dirty hacks. We need to reimplement it with syn and quote
///
use std::{borrow::Cow, str::FromStr};

use proc_macro::{Delimiter, Group, Punct, TokenStream, TokenTree};

enum Attr {
    Optional,
    MsgSend(String),
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
                        "msg_send" | "msg_send2" => {
                            let Some(TokenTree::Group(a)) = iter.next() else {
                                return None;
                            };
                            let sel = a.stream().to_string().replace([' ', '\n'], "");
                            Some(Attr::MsgSend(sel))
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
    let x86_64 = false;
    gen_msg_send(sel, func, true, false, false, x86_64)
}

#[proc_macro_attribute]
pub fn rar_retain_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, true, false, false, x86_64)
}

#[proc_macro_attribute]
pub fn cls_rar_retain(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, true, true, false, x86_64)
}

#[proc_macro_attribute]
pub fn cls_rar_retain_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, true, true, false, x86_64)
}

#[proc_macro_attribute]
pub fn msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, false, false, false, x86_64)
}

#[proc_macro_attribute]
pub fn msg_send_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, false, false, false, x86_64)
}

/// Should generate static fn sel_xxx function that gets selector.
/// So user can check selector with is_reponds_to_sel
#[proc_macro_attribute]
pub fn optional(_sel: TokenStream, func: TokenStream) -> TokenStream {
    let mut iter = func.clone().into_iter();
    let Some(TokenTree::Punct(p)) = iter.next() else {
        panic!("expect #[objc::msg_send(...)]")
    };

    if p != '#' {
        panic!("expect #[objc::msg_send(...)]")
    }
    let Some(TokenTree::Group(g)) = iter.next() else {
        panic!("expect #[objc::msg_send(...)]")
    };

    let Some(Attr::MsgSend(extern_name)) = Attr::from_stream(g.stream()) else {
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
pub fn obj_trait(_args: TokenStream, tr: TokenStream) -> TokenStream {
    let mut original_trait = tr.clone();

    let iter = tr.into_iter();
    let mut before_trait_name_tokens = vec![];
    let mut after_trait_name_tokens = vec![];
    let mut trait_name = Cow::Borrowed("");
    let mut expect_trait_name = false;
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
                                    fn_args_str = fn_args_str.replacen("(& self", "(&self", 1);
                                    fn_args_str = fn_args_str.replacen(
                                        "(&self",
                                        "(&self, _cmd: Option<&objc::Sel>",
                                        1,
                                    );

                                    fn_args_str =
                                        fn_args_str.replacen("(& mut self", "(&mut self", 1);
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

    let pre = TokenStream::from_iter(before_trait_name_tokens).to_string();
    let obj_trait_name = format!("{trait_name}Impl");
    //let after = TokenStream::from_iter(after_trait_name_tokens.into_iter()).to_string();
    let fns = impl_trait_functions.join("\n");

    let add_methods = if has_optionals {
        Cow::Borrowed("fn cls_add_methods<O: objc::Obj>(cls: &objc::Class<O>);")
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
        let sel = Self::sel_{f}();
        unsafe {{
            let imp: extern \"C\" fn() = std::mem::transmute(Self::impl_{f} as *const u8);
            objc::class_addMethod(cls, sel, imp, std::ptr::null());
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

    TokenStream::from_iter(tokens)
}

#[proc_macro_attribute]
pub fn msg_send_debug_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, false, false, true, x86_64)
}

#[proc_macro_attribute]
pub fn msg_send_debug(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, false, false, true, x86_64)
}

#[proc_macro_attribute]
pub fn cls_msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send(sel, func, false, true, false, x86_64)
}

#[proc_macro_attribute]
pub fn cls_msg_send_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    gen_msg_send(sel, func, false, true, false, x86_64)
}

#[proc_macro_attribute]
pub fn cls_msg_send_debug(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    let debug = true;
    gen_msg_send(sel, func, false, true, debug, x86_64)
}

#[proc_macro_attribute]
pub fn cls_msg_send_debug_x86_64(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = true;
    let debug = true;
    gen_msg_send(sel, func, false, true, debug, x86_64)
}

#[proc_macro_attribute]
pub fn msg_send2(sel: TokenStream, func: TokenStream) -> TokenStream {
    let x86_64 = false;
    gen_msg_send2(sel, func, x86_64)
}

fn gen_msg_send2(sel: TokenStream, func: TokenStream, x86_64: bool) -> TokenStream {
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
    // println!("{option}: {ret_full}");
    let gen_rar_version = !sel.starts_with("new") && ret.contains("arc :: R <");

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
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        } else if fn_args_count == 0 {
            fn_args = fn_args.replacen('(', "(cls: *const std::ffi::c_void", 1);
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        } else {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, imp: *const std::ffi::c_void, ",
                1,
            );
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        }
    }

    let available = versions.available_cfg();
    let unavailable = versions.unavailable_cfg();

    let mut flow = String::new();
    let pre = TokenStream::from_iter(meta).to_string();
    let self_ = if class { "Self::" } else { "self." };
    let vars = vars.join(", ");
    let mut impl_fn_name = fn_name.clone();
    let impl_ret_full = ret_full.replacen("arc :: R <", "arc :: Rar <", 1);
    let impl_ret = ret.replacen("arc :: R <", "arc :: Rar <", 1);

    if gen_rar_version {
        impl_fn_name.push_str("_ar");
        // println!("gen_rar_version!!!!");
    }
    if x86_64 {
        flow.push_str(&format!(
            "

    {doc_alias}
    #[inline]
    {pre} fn {impl_fn_name}{gen}{args}{impl_ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend\"]
            fn msg_send();

            fn sel_registerName(name: *const i8) -> *const std::ffi::c_void;
        }}

        unsafe {{
            let x86_64_sel = sel_registerName(c\"{sel}\".as_ptr());
            let fn_ptr = msg_send as *const std::ffi::c_void;
            let sig: extern \"C\" fn{fn_args} {impl_ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        ))
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
    // if debug {
    // println!("{flow}");
    // }

    flow.parse().unwrap()
}

fn fn_args_from_stream(stream: TokenStream) -> (bool, Vec<String>) {
    if stream.is_empty() {
        return (true, Vec::new());
    }
    let mut res = Vec::new();
    let mut pos = 0;
    let mut self_arg = false;
    let mut skip_ident = false;
    let mut nesting = 0;
    for s in stream.into_iter() {
        match s {
            TokenTree::Group(_) => {}
            TokenTree::Ident(ref i) => {
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
            TokenTree::Punct(p) => match p.as_char() {
                '<' => nesting += 1,
                '>' => nesting -= 1,
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

fn gen_msg_send(
    sel: TokenStream,
    func: TokenStream,
    retain: bool,
    class: bool,
    debug: bool,
    x86_64: bool,
) -> TokenStream {
    let extern_name = sel.to_string().replace([' ', '\n'], "");
    let args_count = extern_name.matches(':').count();

    let mut iter = func.into_iter();
    let mut pre: Vec<String> = Vec::with_capacity(3);

    for t in iter.by_ref() {
        let s = t.to_string();
        pre.push(s);
        if let TokenTree::Ident(v) = t {
            if v.to_string() == "fn" {
                break;
            }
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
            let gen = TokenStream::from_iter(generics).to_string();
            Cow::Owned(gen.replacen('<', "<'ar,", 1))
        }
    } else {
        Cow::Owned(TokenStream::from_iter(generics).to_string())
    };

    let mut ret = TokenStream::from_iter(iter).to_string();
    assert_eq!(ret.pop().expect(";"), ';');
    let ret_full = ret.to_string();
    if let Some((a, _)) = ret.split_once("where") {
        ret = a.to_string();
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

    let (mut fn_args, mut call_args) = if x86_64 {
        let fn_args = fn_args.replacen('(', "(id:", 1).replacen(
            "self",
            "Self, imp: *const std::ffi::c_void",
            1,
        );
        (fn_args, format!("sig(self, x86_64_sel, {vars})"))
    } else if fn_args_count == 0 {
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
        if x86_64 {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, imp: *const std::ffi::c_void,",
                1,
            );
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        } else if fn_args_count == 0 {
            fn_args = fn_args.replacen('(', "(cls: *const std::ffi::c_void", 1);
            call_args = call_args.replacen(
                "sig(self",
                "sig(Self::cls() as *const _ as *const std::ffi::c_void",
                1,
            );
        } else {
            fn_args = fn_args.replacen(
                "(id:",
                "(cls: *const std::ffi::c_void, imp: *const std::ffi::c_void,",
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
    } else if x86_64 {
        format!(
            "

    #[inline]
    {pre} {fn_name}{gen}{args}{ret_full} {{
        extern \"C\" {{
            #[link_name = \"objc_msgSend\"]
            fn msg_send();

            fn sel_registerName(name: *const i8) -> *const std::ffi::c_void;
        }}

        unsafe {{
            let x86_64_sel = sel_registerName(c\"{extern_name}\".as_ptr());
            let fn_ptr = msg_send as *const std::ffi::c_void;
            let sig: extern \"C\" fn{fn_args} {ret} = std::mem::transmute(fn_ptr);

            {call_args}
        }}
    }}
            "
        )
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

#[proc_macro_attribute]
pub fn api_weak(_ts: TokenStream, body: TokenStream) -> TokenStream {
    let mut original_body = body.clone();
    let mut iter = body.into_iter();
    let mut versions = None;
    let mut _is_fn = false;
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
                            // println!("{is_fn};");
                            tokens.clear();
                            versions = None;
                            _is_fn = false;
                        }
                        TokenTree::Group(ref p) if p.delimiter() == Delimiter::Parenthesis => {
                            _is_fn = true;
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
                "all(target_os=\"visionos\", feature=\"vision_{}_{}\")",
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
                "all(target_os=\"visionos\", not(feature=\"vision_{}_{}\"))",
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
            vec.push(format!("vision_{}_{}", v.0, v.1));
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
            vec.push(format!("vision_{}_{}", v.0, v.1));
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
    for t in body.into_iter() {
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
        if let TokenTree::Punct(ref p) = t {
            if p.as_char() == ';' {
                no_body = true;
            }
        }
        maybe_res.push(t.clone());

        if let TokenTree::Group(ref g) = t {
            // function without args ()
            if g.delimiter() == Delimiter::Parenthesis {
                no_args = g.stream().is_empty();
            }
            // function body {}
            if no_args && g.delimiter() == Delimiter::Brace {
                if replace_return(&mut maybe_res) {
                    let mut iter = g.stream().into_iter();
                    let ident = iter.next().unwrap().to_string();
                    assert_eq!(ident, "unsafe");
                    match iter.next().expect("token") {
                        TokenTree::Group(g) => {
                            let TokenTree::Ident(var) = g.stream().into_iter().next().unwrap()
                            else {
                                panic!("expecting variable");
                            };
                            let var = upper_case(&var.to_string());
                            let stream =
                                TokenStream::from_str(&format!("{{ unsafe {{ {var}.get() }} }}"))
                                    .unwrap();
                            maybe_res.extend(stream);
                        }
                        _ => panic!("expecting unsafe block"),
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

fn replace_return(tokens: &mut Vec<TokenTree>) -> bool {
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
    idx -= 1; // &
    tokens.pop(); // {}
    let stream = TokenStream::from_iter(tokens.drain(idx..));
    let ty = stream.to_string();
    let stream = TokenStream::from_str(&format!("Option<{ty}>")).unwrap();
    tokens.extend(stream);
    return true;
}

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
