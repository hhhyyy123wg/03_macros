use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EnumFrom)]
pub fn derive_enum_from(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let variants = match input.data {
        Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom can only be derived for enums"),
    };

    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    let field = &fields.unnamed.first().expect("should have one field");
                    let ty = &field.ty;
                    quote! {
                        impl From<#ty> for #ident {
                            fn from(value: #ty) -> Self {
                                #ident::#var(value)
                            }
                        }
                    }
                } else {
                    quote!{}
                }
            }
            _ => quote!{},
        }
        
    });

    quote! {
        #(#from_impls)*
    }.into()
}