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

    #[darling(default, rename = "default")]
    default_val: Option<String>,
}

impl Field {
    pub fn expand(&self) -> SynResult<TokenStream2> {
        let ident = self.ident.as_ref().unwrap();
        let key = self.get_key();
        let default_val = self.get_default().unwrap_or_else(|| {
            quote!(.map_err(|e| format!("environment variable `{}` get failed! err: {:?}" , #key, e))?)
        });
        let mut token_stream = TokenStream2::new();
        token_stream.extend(quote!(#ident:));
        token_stream.extend(quote!(std::env::var(#key)));
        token_stream.extend(quote!(#default_val));
        token_stream.extend(quote!(.parse()));
        token_stream.extend(
            quote!(.map_err(|_e| format!("environment variable `{}` parse failed!", #key))?),
        );
        Ok(token_stream)
    }

    pub fn get_key(&self) -> String {
        let key = if let Some(ref s) = self.rename {
            s.clone()
        } else {
            let key = self.ident.as_ref().unwrap().to_string();
            key
        };

        key
    }

    pub fn get_default(&self) -> Option<TokenStream2> {
        self.default_val
            .as_ref()
            .map(|s| quote!(.unwrap_or_else(|_e| #s.to_string())))
    }
}
