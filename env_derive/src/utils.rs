use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::{self, FoundCrate};
use quote::quote;
use syn::Ident;

pub fn get_crate_name() -> TokenStream2 {
    let name = match proc_macro_crate::crate_name("env_rs") {
        Ok(FoundCrate::Name(name)) => name,
        Ok(FoundCrate::Itself) => "crate".to_string(),
        Err(_) => "env_rs".to_string(),
    };

    let name = Ident::new(name.as_str(), Span::call_site());
    quote!(#name)
}
