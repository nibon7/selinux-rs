extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use syn::Ident;

#[proc_macro]
pub fn wrap_context_get(ident: TokenStream) -> TokenStream {
    let s1 = format!("get_{}", ident.to_string());
    let s2 = format!("context_{}_get", ident.to_string());
    let f1 = Ident::new(&s1, Span::call_site());
    let f2 = Ident::new(&s2, Span::call_site());

    let exp = quote::quote! {
        pub fn #f1(&self) -> Option<&str> {
            unsafe {
                match crate::ffi::#f2(self.0) {
                    p if !p.is_null() => std::ffi::CStr::from_ptr(p).to_str().ok(),
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(exp)
}

#[proc_macro]
pub fn wrap_context_set(ident: TokenStream) -> TokenStream {
    let s1 = format!("set_{}", ident.to_string());
    let s2 = format!("context_{}_set", ident.to_string());

    let f1 = Ident::new(&s1, Span::call_site());
    let f2 = Ident::new(&s2, Span::call_site());

    let exp = quote::quote! {
        pub fn #f1(&mut self, s: &str) -> Option<&mut Self> {
            let cs = CString::new(s).ok()?;

            match unsafe { ffi::#f2(self.0, cs.as_ptr() as *const i8) } {
                0 => Some(self),
                _ => None,
            }
        }
    };

    TokenStream::from(exp)
}
