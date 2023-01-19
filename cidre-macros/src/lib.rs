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

    for t in group.into_iter() {
        match t {
            TokenTree::Ident(i) => {
                prev = Some(i.to_string());
            }
            TokenTree::Punct(p) if p.as_char() == ':' => {
                if let Some(id) = prev.take() {
                    vars.push(id)
                }
            }
            _ => prev = None,
        }
    }
    vars
}
#[proc_macro_attribute]
pub fn rar_retain(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, true)
}
#[proc_macro_attribute]
pub fn msg_send(sel: TokenStream, func: TokenStream) -> TokenStream {
    gen_msg_send(sel, func, false)
}

fn gen_msg_send(sel: TokenStream, func: TokenStream, retain: bool) -> TokenStream {
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

    let Some(TokenTree::Group(args)) = iter.next() else {
        panic!("foo");
    };

    let ts = TokenStream::from_iter(iter);
    let mut ret = ts.to_string();
    assert_eq!(ret.pop().expect(";"), ';');
    let option = ret.contains("-> Option");

    let fn_args = args.to_string();
    let vars = get_fn_args(args.stream());
    let fn_args_count = vars.len();
    if !retain {
        assert!(fn_args_count == args_count);
    }

    let pre = pre.join(" ");
    let vars = vars.join(", ");

    let (fn_args, call_args) = if fn_args_count == 0 {
        let fn_args = fn_args.replace("& self", "id: &Self");
        let fn_args = fn_args.replace("& mut self", "id: &mut Self");
        (fn_args, "sig(self)".to_string())
    } else {
        let fn_args = fn_args.replace("& self", "id: &Self, imp: *const std::ffi::c_void");
        let fn_args = fn_args.replace("& mut self", "id: &mut Self, imp: *const std::ffi::c_void");
        (fn_args, format!("sig(self, std::ptr::null(), {})", vars))
    };

    let flow = if retain {
        if option {
            format!(
                "
                #[inline]
                {pre} {fn_name}{args} {ret} {{
                    arc::Rar::option_retain(self.{fn_name}_ar({vars}) )
                }}
                "
            )
        } else {
            format!(
                "
                #[inline]
                {pre} {fn_name}{args} {ret} {{
                    self.{fn_name}_ar({vars}).retain()
                }}
                "
            )
        }
    } else {
        format!(
            "
        #[inline]
        {pre} {fn_name}{args} {ret} {{
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
