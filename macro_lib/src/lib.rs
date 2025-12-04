use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component)]
pub fn derive_component(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let name = input.ident;
    let expanded = quote::quote! {
        impl Component for #name {}
    };
    TokenStream::from(expanded)
}
