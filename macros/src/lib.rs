use proc_macro::TokenStream;
use std::iter::FromIterator;
use syn::{parse_macro_input, DeriveInput};

/// Extract and return the segements to a final path
fn extract_path_segements(segs: Vec<&syn::PathSegment>) -> Vec<syn::Ident> {
    segs.iter()
        .map(|s| s.ident.clone())
        .collect::<Vec<syn::Ident>>()
}

/// Merge a vector of identification tags into a single string
fn merge_idents(v: Vec<syn::Ident>) -> String {
    if v.is_empty() {
        panic!("Cannot merge empty indents!")
    }
    v.iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

/// Holds data needed for the IntoImpl macro generation
struct IntoImpl {
    variant_name: syn::Ident,
    path_segs: String,
}

/// This macro, when derived, will automatically create Into<T> implemtnations for
/// all variants of the enum it is derived on.
/// This macro can only be used on enums, and has support for custom types in variants.
#[proc_macro_derive(IntoImpl, attributes(exclude))]
pub fn derive_into_function_websockets(input: TokenStream) -> TokenStream {
    //Parse the structure this was called on
    let input = parse_macro_input!(input as DeriveInput);
    let mut data_enum: syn::DataEnum = match input.data {
        syn::Data::Enum(e) => e,
        _ => panic!("MessageInto only works on enums!"),
    };

    let name = input.ident;
    let mut scanned_types: Vec<String> = vec![];
    let mut variant_types: Vec<IntoImpl> = vec![];
    //Attempt to get the types of all of the different variants inside of the enum
    data_enum
        .variants
        .iter_mut()
        .for_each(|x: &mut syn::Variant| {
            let variant_name = x.ident.clone();

            let mut excluded = false;
            x.attrs.clone().iter().for_each(|t| {
                if merge_idents(extract_path_segements(t.path.segments.iter().collect())) == "exclude" {
                    excluded = true;
                }
            });
            if !excluded {
                let path_segs = x
                    .fields
                    .clone()
                    .iter()
                    .map(|field| match &field.ty {
                        syn::Type::Path(p) => extract_path_segements(p.path.segments.iter().collect()),
                        _ => panic!("Unsupported type!"),
                    })
                    .collect::<Vec<Vec<syn::Ident>>>();
                if !path_segs.is_empty() {
                    let path_segs = merge_idents(path_segs[0].clone());
                    if scanned_types.contains(&path_segs) {
                        panic!("Already implemented a type of `{}` please exclude this type with #[exclude]", path_segs);
                    }
                    scanned_types.push(path_segs.clone());
                    variant_types.push(IntoImpl {
                        variant_name,
                        path_segs,
                    })
                }
            }
        });

    //Generate the implementations for the struct, and return them.
    let mut expanded: Vec<TokenStream> = vec![];
    variant_types.iter().for_each(|t| {
        let inner = format!(
            "
            impl ::std::convert::Into<{}> for {} {{
                fn into(self) -> {} {{
                    {}::{}(self)
                }}
            }}
            ",
            name, t.path_segs, name, name, t.variant_name
        );
        expanded.push(inner.parse().unwrap());
    });

    TokenStream::from_iter(expanded)
}
