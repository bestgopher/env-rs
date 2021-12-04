use crate::utils;
use darling::FromField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::Result as SynResult;

#[derive(FromField, Debug)]
#[darling(attributes(env))]
pub struct Field {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    pub vis: syn::Visibility,
    pub attrs: Vec<syn::Attribute>,
    #[darling(default)]
    rename: Option<String>,
}

impl Field {
    pub fn expand(&self) -> SynResult<TokenStream2> {
        let ident = self.ident.as_ref().unwrap();
        let key = self.get_key();
        let crate_name = utils::get_crate_name();
        Ok(quote!(#ident: #crate_name::var(#key)?))
    }

    pub fn get_key(&self) -> String {
        if let Some(ref s) = self.rename {
            return s.clone();
        }

        let key = self.ident.as_ref().unwrap().to_string();

        key
    }
}
