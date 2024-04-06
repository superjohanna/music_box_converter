extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{self, parse_macro_input, token::Type, Data, DeriveInput, Field, Fields};
use reflect::ValueType;

const STRING_TOKEN: TokenStream2 = quote! { String };
const BOOL_TOKEN: TokenStream2 = quote! { bool };
const F64_TOKEN: TokenStream2 = quote! { f64 };

#[proc_macro_derive(Reflect)]
pub fn reflect_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = ast.ident;

    let data_struct = match ast.data {
        Data::Struct(data_struct) => data_struct,
        _ => panic!("Can only derive reflect for structs"),
    };

    let fields_named = match data_struct.fields {
        Fields::Named(fields_named) => fields_named,
        _ => panic!("Can only derive reflect for structs with named fields. Struct needs at least one named field!")
    };

    let fields: Vec<_> = fields_named.named.iter().collect();

    let fields_type: Vec<syn::Type> = fields
        .iter()
        .map(|x| x.ty.clone())
        .map(|x| match x {
            syn::Type::Verbatim(STRING_TOKEN) => ValueType::String,
        })
        .collect();

    let fields_ident: Vec<_> = fields.iter().map(|x| x.ident.clone().unwrap()).collect();

    let fields_index: Vec<usize> = fields.iter().enumerate().map(|(i, _)| i).collect();

    let expansion = quote! {
        impl Reflect for #name {
            fn set(&mut self, index: usize, value: ValueWrapper) {
                match index {
                    #(#fields_index => self.#fields_ident = value.try_into().unwrap(),)*
                    _ => panic!("Index out of bounds for set function from trait Reflect!")
                }
            }

            fn get(&self, index: usize) -> ValueWrapper {
                match index {
                    #(#fields_index => ValueWrapper::from(self.#fields_ident.clone()),)*
                    _ => panic!("Index out of bounds for get function from trait Reflect!")
                }
            }

            fn field_name(index: usize) -> &'static str {
                match index {
                    #(#fields_index => stringify!(#fields_ident),)*
                    _ => panic!("Index out of bounds for field_name function from trait Reflect!")
                }
            }

            fn field_type(index: usize) -> ValueType {
                match index {
                    #(#fields_index => )*
                    _ => panic!("Index out of bounds for field_type function from trait Reflect!")
                }
            }
        }
    };

    proc_macro::TokenStream::from(expansion)
}
