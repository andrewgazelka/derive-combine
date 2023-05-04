use proc_macro::TokenStream;

use anyhow::{bail, Context};
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Type};

#[proc_macro_derive(Combine)]
pub fn combine_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match combine_derive_helper(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => {
            let err = err.to_string();
            let res = quote! {
                compile_error!(#err);
            };
            res.into()
        }
    }
}

fn combine_derive_helper(input: DeriveInput) -> anyhow::Result<proc_macro2::TokenStream> {
    let name = &input.ident;
    let fields = match input.data {
        syn::Data::Struct(data) => data.fields,
        _ => {
            bail!("Combine can only be derived for structs")
        }
    };

    let mut exprs = Vec::with_capacity(fields.len());

    for field in fields {
        let field_name = field
            .ident
            .context("Combine can only be derived for named fields")?;
        let field_type = &field.ty;

        let Type::Path(path) = field_type else {
            bail!("Type of field {field_name} is not a path");
        };

        let path = &path.path;

        let Some(first_segment) = path.segments.first() else {
            bail!("Type of field {field_name} does not have any segments");
        };

        let first_segment_ident = first_segment.ident.to_string();

        let expr = match first_segment_ident.as_str() {
            "Option" => {
                quote! {
                    self.#field_name = self.#field_name.or(other.#field_name);
                }
            }
            "Vec" => {
                quote! {
                    self.#field_name.append(&mut other.#field_name);
                }
            }
            _ => {
                quote! {} // Do nothing
            }
        };

        exprs.push(expr);
    }

    let expanded = quote! {
        impl #name {
            fn combine(&mut self, mut other: Self) {
                #(#exprs)*
            }
        }
    };

    Ok(expanded)
}
