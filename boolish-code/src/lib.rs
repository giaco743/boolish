use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{ext::IdentExt, Data, DeriveInput, Ident, LitStr, Meta, MetaList, Variant};

fn extract_attribute_from_variant<'a>(v: &'a Variant, name: &'a str) -> Option<&'a syn::Attribute> {
    v.attrs.iter().find(|&attr| attr.path().is_ident(name))
}

pub fn make_boolish(item: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(item).unwrap();
    let name = ast.ident;
    let variants = match ast.data {
        Data::Enum(data) => data.variants,
        _ => unreachable!("Only implemented for enums"),
    };
    assert_eq!(variants.len(), 2, "Only for enums with 2 variants");
    let first = variants.first().unwrap();
    let second = variants.last().unwrap();
    let boolval_first_str = extract_attribute_from_variant(first, "boolval")
        .map(|a| &a.meta)
        .map(|m| match m {
            Meta::List(nested) => {
                let a: LitStr = nested.parse_args().unwrap();
                Ident::new(&a.value(), a.span())
            }
            _ => unreachable!("Only implemented for lists"),
        });
    let first_val = match &boolval_first_str.unwrap().to_string().as_str() {
        &"true" => true,
        &"false" => false,
        _ => unreachable!("First val is not boolean???"),
    };
    let first_variant = &first.ident;
    let boolval_second_str = extract_attribute_from_variant(second, "boolval")
    .map(|a| &a.meta)
    .map(|m| match m {
        Meta::List(nested) => {
            let a: LitStr = nested.parse_args().unwrap();
            Ident::new(&a.value(), a.span())
        }
        _ => unreachable!("Only implemented for lists"),
    });
    let second_val = match &boolval_second_str.unwrap().to_string().as_str() {
        &"true" => true,
        &"false" => false,
        _ => unreachable!("Second val is not boolean???"),
    };
    let second_variant = &second.ident;
    quote! {
        impl From<bool> for #name {
            fn from(boolean: bool) -> Self{
                match boolean {
                    #first_val => #name::#first_variant,
                    #second_val => #name::#second_variant,
                }
            }
        }

        impl Into<bool> for #name {
            fn into(self) -> bool {
                match self {
                    #name::#first_variant => #first_val,
                    #name::#second_variant => #second_val,
                }
            }
        }
    }
}
