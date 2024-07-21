use darling::{
    ast::{Data, Style},
    FromDeriveInput, FromField, FromVariant,
};
use proc_macro2::TokenStream;
use quote::quote;

#[derive(Debug, FromDeriveInput)]
struct EnumFromDarling {
    ident: syn::Ident,
    generics: syn::Generics,
    data: darling::ast::Data<EnumDeriveVariant, ()>,
}

#[derive(Debug, FromVariant)]
struct EnumDeriveVariant {
    ident: syn::Ident,
    fields: darling::ast::Fields<EnumDeriveField>,
}

#[derive(Debug, FromField)]
struct EnumDeriveField {
    ty: syn::Type,
}

pub(crate) fn process_enum_from_darling(input: syn::DeriveInput) -> TokenStream {
    let EnumFromDarling {
        ident,
        generics,
        data: Data::Enum(data),
    } = EnumFromDarling::from_derive_input(&input).expect("parse failed")
    else {
        panic!("not enum")
    };

    let from_impls = data.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let style = &variant.fields.style;

        match style {
            Style::Tuple if variant.fields.len() == 1 => {
                let field = variant.fields.iter().next().expect("should have one field");
                let ty = &field.ty;

                quote! {
                    impl #generics  From<#ty> for #ident #generics {
                        fn from(value: #ty) -> Self {
                            #ident::#variant_ident(value)
                        }
                    }
                }
            }
            _ => quote! {},
        }
    });

    quote! {
        #(#from_impls)*
    }
}
