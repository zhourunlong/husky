use heck::{ToKebabCase, ToTitleCase};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, Result, Token, Type, Visibility};

use crate::util::{documentation, foundations};

/// Expand the `#[category]` macro.
pub fn category(_: TokenStream, item: syn::Item) -> Result<TokenStream> {
    let syn::Item::Verbatim(stream) = item else {
        bail!(item, "expected bare static");
    };

    let BareStatic {
        attrs,
        vis,
        ident,
        ty,
        ..
    } = syn::parse2(stream)?;

    let name = ident.to_string().to_kebab_case();
    let title = name.to_title_case();
    let docs = documentation(&attrs);

    Ok(quote! {
        #(#attrs)*
        #vis static #ident: #ty = {
            static DATA: #foundations::TexCategoryData = #foundations::TexCategoryData {
                name: #name,
                title: #title,
                docs: #docs,
            };
            #foundations::TexDefnKind::from_data(&DATA)
        };
    })
}

/// Parse a bare `pub static CATEGORY: Category;` item.
pub struct BareStatic {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub static_token: Token![static],
    pub ident: Ident,
    pub colon_token: Token![:],
    pub ty: Type,
    pub semi_token: Token![;],
}

impl Parse for BareStatic {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Self {
            attrs: input.call(Attribute::parse_outer)?,
            vis: input.parse()?,
            static_token: input.parse()?,
            ident: input.parse()?,
            colon_token: input.parse()?,
            ty: input.parse()?,
            semi_token: input.parse()?,
        })
    }
}
