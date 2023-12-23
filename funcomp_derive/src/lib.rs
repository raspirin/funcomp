use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput, Error, Fields, Ident};

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
    let mut unit_ident = vec![];
    let mut unit_ident_lower = vec![];

    if let Data::Enum(data_enum) = item_data {
        for var in data_enum.variants {
            let mut var_field_idents = vec![];
            let mut var_tys = vec![];
            let span = var.span();
            match var.fields {
                Fields::Unit => {
                    unit_ident_lower.push(Ident::new(&var.ident.to_string().to_lowercase(), span));
                    unit_ident.push(var.ident);
                }
                Fields::Unnamed(fields) => {
                    var_ident_lower.push(Ident::new(&var.ident.to_string().to_lowercase(), span));
                    var_ident.push(var.ident);
                    for (counter, field) in fields.unnamed.into_iter().enumerate() {
                        let span = field.ty.span();
                        var_tys.push(field.ty);
                        var_field_idents.push(Ident::new(&format!("a{counter}"), span));
                    }
                }
                Fields::Named(_) => {
                    return Error::new(
                        var.fields.span(),
                        "ItemKind can only used with unnamed fields.",
                    )
                    .into_compile_error()
                    .into();
                }
            }

            var_fields_ident.push(var_field_idents);
            var_fields.push(var_tys);
        }
    } else {
        return Error::new(item_ident.span(), "ItemKind can only used with enum.")
            .into_compile_error()
            .into();
    }

    let out = quote! {
        impl<#item_generic> #item_ident<#item_generic> {
            #(pub fn #var_ident_lower( #(#var_fields_ident: #var_fields, )* ) -> Self {
                Self::#var_ident( #(#var_fields_ident, )* )
            })*

            #(pub fn #unit_ident_lower() -> Self {
                Self::#unit_ident
            })*
        }
    };
    proc_macro::TokenStream::from(out)
}
