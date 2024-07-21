use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    let ident = input.ident;
    let generics = input.generics;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = &fields.unnamed.first().expect("should have one field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics From<#ty> for #ident #generics{
                            fn from(value: #ty) -> Self {
                                #ident::#var(value)
                            }
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
