use proc_macro::TokenStream;

/// fn foo(&self) -> arc::R<T> will generate
/// fn foo(&self) -> arc::R<T>
//
/// fn foo(&self) -> arc::Rar<T> will generate
/// fn foo(&self) -> arc::R<T>
/// fn foo_ar(&self) -> arc::Rar<T>
#[proc_macro_attribute]
pub fn msg_send(attr: TokenStream, item: TokenStream) -> TokenStream {
    let extern_name = attr.to_string().replace(' ', "");
    println!("attr: {:?}", attr);
    println!("item: {:?}", item);

    let flow = format!(
        "
        #[inline]
        pub fn foo(&self) {{
            extern \"C\" {{
                #[link_name = \"objc_msgSend${extern_name}\"]
                fn msg_send();
            }}

            unsafe {{
                let fn_ptr = msg_send as *const std::ffi::c_void;
                let sig: extern \"C\" fn(id: &Self) = std::mem::transmute(fn_ptr);

                sig(self)
            }}
        }}
        "
    );

    println!("flow: {flow}");

    let flow = flow.parse().unwrap();

    flow
}
