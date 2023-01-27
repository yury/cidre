use proc_macro::{Delimiter, TokenStream, TokenTree};

fn sel_args_count(sel: TokenStream) -> usize {
    sel.into_iter()
        .filter(|t| match t {
            TokenTree::Punct(v) if v.as_char() == ':' => true,
            _ => false,
        })
        .count()
}

fn get_fn_args(group: TokenStream, class: bool, debug: bool) -> Vec<String> {
    let mut prev = None;
    let mut vars = Vec::with_capacity(10);
    if debug {
        println!("tokens {:?}", group);
    }

    let mut can_be_name = class;
    let mut level = 0;
    for t in group.into_iter() {
        match t {
            TokenTree::Ident(i) => {
                prev = Some(i.to_string());
            }
            TokenTree::Punct(p) if can_be_name && level == 0 && p.as_char() == ':' => {
                if let Some(id) = prev.take() {
                    vars.push(id)
                }
                can_be_name = false;
            }
            TokenTree::Punct(p) if p.as_char() == '<' => level += 1,
            TokenTree::Punct(p) if p.as_char() == '>' => level -= 1,
            TokenTree::Punct(p) if p.as_char() == ',' => can_be_name = true,
            _ => prev = None,
        }
    }
    vars
}

#[proc_macro_attribute]
pub fn register_cls(attr: TokenStream, body: TokenStream) -> TokenStream {
    println!("{:#?}", attr);
    let iter = body.into_iter();
    let (len, _) = iter.size_hint();
    let mut tokens = Vec::with_capacity(len);
    // let func_names = Vec::with_capacity(5);
    for t in iter {
        match &t {
            TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => {
                // println!("{g:?}");
                for t in g.stream() {
                    println!("--- {t:?}");
                }
            }
            // TokenTree::Ident(i) => {
            //     println!("{i:?}");
            // }
            TokenTree::Literal(l) => {
                println!("{l:?}");
            }
            _ => {}
        };
        tokens.push(t);
    }
    println!("size {len}");
    "".parse().unwrap()
}

#[proc_macro_attribute]
pub fn proto_msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    let extern_name = sel.to_string().replace(' ', "");
    let args_count = sel_args_count(sel);
    let mut iter = func.into_iter();
    let mut pre: Vec<String> = Vec::with_capacity(3);

    while let Some(t) = iter.next() {
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
        panic!("foo");
    };

    let fn_name = fn_name.to_string();
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
    let gen = TokenStream::from_iter(generics.into_iter()).to_string();

    let ts = TokenStream::from_iter(iter);
    let mut ret_full = ts.to_string();
    assert_eq!(ret_full.pop().expect(";"), ';');
    let pre = pre.join(" ");
    let class = false;
    let debug = false;
    let vars = get_fn_args(args.stream(), class, debug);
    let fn_args_count = vars.len();
    assert_eq!(
        fn_args_count, args_count,
        "left: fn_args_count, right: sel_args_count"
    );

    let vars = vars.join(", ");
    let fn_args = args.to_string();
    let (fn_args, call_args) = if fn_args_count == 0 {
        let fn_args = "(id: *const std::ffi::c_void, cmd: *const std::ffi::c_void)".to_string();
        (fn_args, "()".to_string())
    } else {
        let fn_args = fn_args
            .replacen(
                "(& self",
                "(id: *const std::ffi::c_void, cmd: *const std::ffi::c_void",
                1,
            )
            .replacen(
                "(& mut self",
                "(id: *const std::ffi::c_void, cmd: *const std::ffi::c_void",
                1,
            );
        (fn_args, format!("({})", vars))
    };
    let opt_impl = if fn_name.starts_with("opt_") {
        "{ unimplemented!() }"
    } else {
        ";"
    };

    let flow = format!(
        "
        #[inline]
        {pre} {fn_name}{gen}{args}{ret_full}{opt_impl}
        
        fn sel_{fn_name}() -> &'static objc::Sel {{
            unsafe {{ objc::sel_registerName(b\"{extern_name}\\0\".as_ptr()) }}
        }}

        extern \"C\" fn iml_{fn_name}{gen}{fn_args}{ret_full} {{
            unsafe {{
                let slf: &mut Self = std::mem::transmute(objc::object_getIndexedIvars(id));
                slf.{fn_name}{call_args}
            }}
         }}
    "
    );

    flow.parse().unwrap()
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
    let extern_name = sel.to_string().replace(' ', "");
    let _is_init = extern_name.starts_with("init");
    let args_count = sel_args_count(sel);

    let mut iter = func.into_iter();
    let mut pre: Vec<String> = Vec::with_capacity(3);

    while let Some(t) = iter.next() {
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
        panic!("foo");
    };

    let fn_name = fn_name.to_string();
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
    let gen = TokenStream::from_iter(generics.into_iter()).to_string();

    let ts = TokenStream::from_iter(iter);
    let mut ret = ts.to_string();
    let mut ret_full = ts.to_string();
    if let Some((a, _)) = ret.split_once("where") {
        ret = format!("{};", a)
    }
    assert_eq!(ret.pop().expect(";"), ';');
    assert_eq!(ret_full.pop().expect(";"), ';');
    let option = ret_full.contains("-> Option");

    let fn_args = args.to_string();
    if debug {
        println!("ret: {}", ret);
        println!("fn_args: {}", fn_args);
    }

    let vars = get_fn_args(args.stream(), class, debug);
    let fn_args_count = vars.len();
    if !retain {
        assert_eq!(
            fn_args_count, args_count,
            "left: fn_args_count, right: sel_args_count"
        );
    }
    let pre = pre.join(" ");
    let vars = vars.join(", ");

    let (mut fn_args, mut call_args) = if fn_args_count == 0 {
        let fn_args = fn_args
            .replacen("( &", "(id: &", 1)
            .replacen("self", "Self", 1);
        (fn_args, "sig(self)".to_string())
    } else {
        let fn_args = fn_args
            .replacen("(", "(id:", 1)
            .replace("self", "Self, imp: *const std::ffi::c_void");
        (fn_args, format!("sig(self, std::ptr::null(), {})", vars))
    };

    if class {
        if fn_args_count == 0 {
            fn_args = fn_args.replacen("(", "(cls: *const std::ffi::c_void", 1);
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
        let mut self_ = "self.".to_string();
        if class {
            self_ = "Self::".to_string();
        }
        if option {
            format!(
                "
                #[inline]
                {pre} {fn_name}{gen}{args}{ret_full} {{
                    arc::Rar::option_retain({self_}{fn_name}_ar({vars}) )
                }}
                "
            )
        } else {
            format!(
                "
                #[inline]
                {pre} {fn_name}{gen}{args}{ret_full} {{
                    {self_}{fn_name}_ar({vars}).retain()
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
        println!("{}", flow.to_string());
    }

    flow.parse().unwrap()
}
