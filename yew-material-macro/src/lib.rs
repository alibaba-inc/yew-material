extern crate proc_macro;
use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(FormRes)]
pub fn form_props_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_form_props_macro(&ast)
}
fn impl_form_props_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl FormRes for #name {}
    };
    gen.into()
}
