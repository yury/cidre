use proc_macro::{TokenStream, TokenTree};

fn sel_args_count(sel: TokenStream) -> usize {
    sel.into_iter()
        .filter(|t| match t {
            TokenTree::Punct(v) if v.as_char() == ':' => true,
            _ => false,
        })
        .count()
}

fn get_fn_args(group: TokenStream) -> Vec<String> {
    let mut prev = None;
    let mut vars = Vec::with_capacity(10);

    let mut can_be_name = false;
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
pub fn rar_retain(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, true, false)
}

#[proc_macro_attribute]
pub fn msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, false)
}

#[proc_macro_attribute]
pub fn cls_msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false, true)
}

fn gen_msg_send(sel: TokenStream, func: TokenStream, retain: bool, class: bool) -> TokenStream {
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
    let mut ret = ts.to_string();
    assert_eq!(ret.pop().expect(";"), ';');
    let option = ret.contains("-> Option");

    let fn_args = args.to_string();
    let vars = get_fn_args(args.stream());
    let fn_args_count = vars.len();
    if !retain {
        assert_eq!(fn_args_count, args_count);
    }

    let pre = pre.join(" ");
    let vars = vars.join(", ");

    let (mut fn_args, mut call_args) = if fn_args_count == 0 {
        let fn_args = fn_args
            .replacen("( &", "(id:", 1)
            .replacen("self", "Self", 1);
        (fn_args, "sig(self)".to_string())
    } else {
        let fn_args = fn_args
            .replacen("(", "(id:", 1)
            .replace("self", "Self, imp: *const std::ffi::c_void");
        (fn_args, format!("sig(self, std::ptr::null(), {})", vars))
    };

    if class {
        fn_args = fn_args.replacen("(", "(cls: *const std::ffi::c_void, ", 1);
        call_args = call_args.replacen(
            "sig(self",
            "sig(Self::cls() as *const _ as *const std::ffi::c_void",
            1,
        );
    }

    let flow = if retain {
        if option {
            format!(
                "
                #[inline]
                {pre} {fn_name}{gen}{args} {ret} {{
                    arc::Rar::option_retain(self.{fn_name}_ar({vars}) )
                }}
                "
            )
        } else {
            format!(
                "
                #[inline]
                {pre} {fn_name}{gen}{args} {ret} {{
                    self.{fn_name}_ar({vars}).retain()
                }}
                "
            )
        }
    } else {
        format!(
            "
            #[inline]
            {pre} {fn_name}{gen}{args} {ret} {{
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

    flow.parse().unwrap()
}
