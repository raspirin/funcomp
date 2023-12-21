use proc_macro::TokenStream;
use std::fmt::format;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Error, Type, Ident};
use syn::spanned::Spanned;

#[proc_macro_derive(ItemKind)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // collect ident and data
    let item_ident = input.ident;
    let item_data = input.data;
    let item_generic = input.generics.params;

    // collect ident and fields
    let mut var_ident = vec![];
    let mut var_ident_lower = vec![];
    let mut var_fields = vec![];
    let mut var_fields_ident = vec![];

    if let Data::Enum(data_enum) = item_data {
        for var in data_enum.variants {
            var_ident_lower.push(Ident::new(&var.ident.to_string().to_lowercase(), var.span()));
            var_ident.push(var.ident);
            let mut var_field_idents = vec![];
            let mut var_tys = vec![];
            if let Fields::Unnamed(fields) = var.fields {
                let mut counter = 0;
                for field in fields.unnamed {
                    let span = field.ty.span();
                    var_tys.push(field.ty);
                    var_field_idents.push(Ident::new(&format!("a{counter}"), span));
                    counter += 1;
                }
            } else {
                return Error::new(var.fields.span(), "ItemKind can only used with unnamed fields.").into_compile_error().into()
            }
            var_fields_ident.push(var_field_idents);
            var_fields.push(var_tys);
        }
    } else {
        return Error::new(item_ident.span(), "ItemKind can only used with enum.").into_compile_error().into()
    }

    let out = quote! {
        impl<#item_generic> #item_ident<#item_generic> {
            #(pub fn #var_ident_lower( #(#var_fields_ident: #var_fields, )* ) -> Self {
                Self::#var_ident( #(#var_fields_ident, )* )
            })*
        }
    };
    proc_macro::TokenStream::from(out)
}