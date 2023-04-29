extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Generics};

#[proc_macro_derive(Type)]
pub fn derive_typed(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Enum(e) => panic!("cannot use enum"),
        Data::Struct(e) => derive_struct_typed(ident, generics, e),
        _ => panic!(""),
    }
}

fn derive_struct_typed(name: Ident, generics: Generics, item: DataStruct) -> TokenStream {
    let odu_types_name = format_ident!("odu_types");

    let len = item.fields.len();

    let fields = item.fields.iter().map(|m| {
        //
        let name = m.ident.as_ref().unwrap();
        let name_str = name.to_string();
        let ty = &m.ty;
        quote!(

            #odu_types_name::Field {
                name: #name_str,
                kind: #ty::typed()

            }
        )
    });

    let name_str = name.to_string();

    quote!(

        impl #odu_types_name::HasStaticType for #name {
            fn create_type_info() -> #odu_types_name::ComplexType {
                static L: #odu_types_name::Lazy<#odu_types_name::ComplexType> = #odu_types_name::Lazy::new(|| {
                    #odu_types_name::ComplexType::Struct(std::sync::Arc::new(#odu_types_name::Struct::new(#name_str, vec![#(#fields),*])))
                });
                L.clone()
            }
        }

        impl #odu_types_name::Typed for #name {
            fn typed(&self) -> #odu_types_name::Type {
                <#name as #odu_types_name::StaticTyped>::typed()
            }
        }

    )
    .into()
}

//

#[proc_macro_derive(IntoValue)]
pub fn derive_into_value(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Enum(e) => panic!("cannot use enum"),
        Data::Struct(e) => derive_struct_into_value(ident, generics, e),
        _ => panic!(""),
    }
}

#[proc_macro_derive(FromValue)]
pub fn derive_from_value(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Enum(e) => panic!("cannot use enum"),
        Data::Struct(e) => derive_struct_from_value(ident, generics, e),
        _ => panic!(""),
    }
}

fn derive_struct_into_value(name: Ident, generics: Generics, item: DataStruct) -> TokenStream {
    let odu_value_name = format_ident!("odu_value");

    let len = item.fields.len();

    let fields = item.fields.iter().map(|m| {
        //
        let name = m.ident.as_ref().unwrap();
        let name_str = name.to_string();
        quote!(
            map.insert(#name_str, from.#name);
        )
    });

    quote!(

        impl From<#name> for #odu_value_name::Value {
            fn from(from: #name) ->  #odu_value_name::Value {
                let mut map = #odu_value_name::Map::with_capacity(#len);
                #(
                    #fields
                )*
                #odu_value_name::Value::Map(map)
            }
        }

    )
    .into()
}

fn derive_struct_from_value(name: Ident, generics: Generics, item: DataStruct) -> TokenStream {
    let odu_value_name = format_ident!("odu_value");

    let len = item.fields.len();

    let fields = item.fields.iter().map(|m| {
        //
        let name = m.ident.as_ref().unwrap();
        let name_str = name.to_string();
        quote!(
            #name: match map.get(#name_str) {
                Some(value) => value.clone().try_into()?,
                None => return Err(#odu_value_name::FromValueErr::Value(#odu_value_name::Value::Map(map)))
            }
        )
    }).collect::<Vec<_>>();

    let fields_ref = item
        .fields
        .iter()
        .map(|m| {
            //
            let name = m.ident.as_ref().unwrap();
            let name_str = name.to_string();
            quote!(
                #name: match map.get(#name_str) {
                    Some(value) => value.clone().try_into()?,
                    None => return Err(#odu_value_name::FromValueErr::Ref(from))
                }
            )
        })
        .collect::<Vec<_>>();
    quote!(

        impl TryFrom<#odu_value_name::Value> for #name {
            type Error = #odu_value_name::FromValueErr<'static>;
            fn try_from(from: #odu_value_name::Value) -> Result<#name, Self::Error> {
                let map = from.into_map().map_err(#odu_value_name::FromValueErr::Value)?;


                Ok(#name {
                    #(#fields),*
                })

            }
        }

        impl<'a> TryFrom<&'a #odu_value_name::Value> for #name {
            type Error = #odu_value_name::FromValueErr<'a>;
            fn try_from(from: &'a #odu_value_name::Value) -> Result<#name, Self::Error> {
                let map = match from.as_map() {
                    Some(map) => map,
                    None => return Err(#odu_value_name::FromValueErr::Ref(from))
                };


                Ok(#name {
                    #(#fields_ref),*
                })

            }
        }

    )
    .into()
}
