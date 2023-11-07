extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[allow(clippy::panic)]
#[proc_macro_attribute]
pub fn assert_no_slop(_: TokenStream, input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    let struct_name = &derive_input.ident;

    let struct_name_uppercase = struct_name.to_string().to_uppercase();

    let expanded = match &derive_input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_sizes = fields.named.iter().map(|field| &field.ty);
                let sizes_sum = quote! { #(std::mem::size_of::<#field_sizes>())+* };
                let struct_size_name = format_ident! { "{}_STRUCT_SIZE", struct_name_uppercase };
                let field_sizes_name = format_ident! {  "{}_FIELD_SIZES", struct_name_uppercase };

                quote! {
                    const #struct_size_name : usize = std::mem::size_of::<#struct_name>();
                    const #field_sizes_name : usize = #sizes_sum;

                    const_assert_eq!(#struct_size_name, #field_sizes_name);
                }
            }
            Fields::Unnamed(fields) => {
                let field_types = fields.unnamed.iter().map(|field| &field.ty);
                let sizes_sum = quote! { #(std::mem::size_of::<#field_types>())+* };

                let struct_size_name = format_ident! { "{}_STRUCT_SIZE", struct_name_uppercase };
                let field_sizes_name = format_ident! {  "{}_FIELD_SIZES", struct_name_uppercase };

                quote! {
                    const #struct_size_name : usize = std::mem::size_of::<#struct_name>();
                    const #field_sizes_name : usize = #sizes_sum;

                    const_assert_eq!(#struct_size_name, #field_sizes_name);
                }
            }
            Fields::Unit => {
                panic!("assert_no_padding attribute cannot be used on unit structs");
            }
        },
        _ => {
            panic!("assert_no_padding attribute can only be used on structs");
        }
    };

    let output = quote! {
        #derive_input
        #expanded
    };
    output.into()
}