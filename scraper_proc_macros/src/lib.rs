use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn selector(input: TokenStream) -> TokenStream {
    let selector = parse_macro_input!(input as LitStr).value();
    
    match scraper::Selector::parse(&selector) {
        Ok(_) => quote!(
            ::scraper::Selector::parse(#selector).unwrap()
        ).into(),
        Err(e) => syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Failed to parse CSS selector: {}", e)
        ).to_compile_error().into(),
    }
}