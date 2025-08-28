use convert_case::Casing;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{
    Data, DeriveInput, Fields, Generics, Ident, Index, Type, parse_macro_input, spanned::Spanned,
};

/// Derives the [Getters](abstract_getters::Getters) trait for a struct.
#[proc_macro_derive(Getters)]
pub fn derive_getters(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let generics = input.generics;
    let struct_mod_ident = Ident::new(
        &name.to_string().to_case(convert_case::Case::Snake),
        name.span(),
    );

    let field_impls = match input.data {
        Data::Struct(data_struct) => {
            generate_for_fields(&name, &generics, data_struct.fields, None)
        }
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.into_iter().map(|variant| {
                let variant_module = Ident::new(
                    &variant.ident.to_string().to_case(convert_case::Case::Snake),
                    variant.ident.span(),
                );
                let field_impls =
                    generate_for_fields(&name, &generics, variant.fields, Some(&variant.ident));
                quote! {
                    pub mod #variant_module {
                        use super::*;
                        #field_impls
                    }
                }
            });
            quote! {
                #(#variants)*
            }
        }
        Data::Union(union_data) => syn::Error::new(
            union_data.union_token.span(),
            "Getters cannot be derived for unions",
        )
        .to_compile_error(),
    };

    let expanded = quote! {
        pub mod #struct_mod_ident {
            use super::*;
            #field_impls
        }
    };

    TokenStream::from(expanded)
}

fn generate_for_fields(
    name: &Ident,
    generics: &Generics,
    fields: Fields,
    enum_variant_name: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let field_impls_iter =
        match fields {
            Fields::Named(fields_named) => &mut fields_named.named.into_iter().map(|field| {
                let field_ident = field.ident.expect("A named field");
                generate_for_field(
                    field_ident.clone(),
                    field_ident,
                    name,
                    field.ty,
                    generics,
                    enum_variant_name,
                )
            }) as &mut dyn Iterator<Item = _>,

            Fields::Unnamed(fields_unnamed) => &mut fields_unnamed
                .unnamed
                .into_iter()
                .enumerate()
                .map(|(index, field)| {
                    let field_struct = Ident::new(&format!("_{index}"), field.span());
                    let field_index = Index::from(index);
                    generate_for_field(
                        field_struct,
                        field_index,
                        name,
                        field.ty,
                        generics,
                        enum_variant_name,
                    )
                }) as &mut dyn Iterator<Item = _>,

            _ => &mut std::iter::empty() as &mut dyn Iterator<Item = _>,
        };

    quote! {
        #(#field_impls_iter)*
    }
}

/// Generate an owned, mutable and referential impl for a field by generation a struct with the field's name
/// and implementing the [Field](abstract_getters::Field) trait for it.
fn generate_for_field<N: ToTokens>(
    field_struct: Ident,
    field_name: N,
    struct_name: &Ident,
    ty: Type,
    generics: &Generics,
    enum_variant_name: Option<&Ident>,
) -> proc_macro2::TokenStream {
    let struct_params = &generics.params;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    if let Some(variant_name) = enum_variant_name {
        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub struct #field_struct;
            impl #impl_generics abstract_getters::Field<#field_struct> for #struct_name #ty_generics #where_clause {
                type Type = Option<#ty>;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type {
                    match self {
                        Self::#variant_name{#field_name: __get_field, ..} => Some(__get_field),
                        _ => None,
                    }
                }
            }

            impl <'__top_level, #struct_params> abstract_getters::Field<#field_struct> for &'__top_level #struct_name #ty_generics #where_clause {
                type Type = Option<&'__top_level #ty>;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type {
                    match self {
                        #struct_name::#variant_name{#field_name: __get_field, ..} => Some(__get_field),
                        _ => None,
                    }
                }
            }
            impl <'__top_level, #struct_params> abstract_getters::Field<#field_struct> for &'__top_level mut #struct_name #ty_generics #where_clause {
                type Type = Option<&'__top_level mut #ty>;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type {
                    match self {
                        #struct_name::#variant_name{#field_name: __get_field, ..} => Some(__get_field),
                        _ => None,
                    }
                }
            }
        }
    } else {
        quote! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[allow(non_camel_case_types)]
            pub struct #field_struct;
            impl #impl_generics abstract_getters::Field<#field_struct> for #struct_name #ty_generics #where_clause {
                type Type = #ty;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type { self.#field_name }
            }
            impl<'__top_level, #struct_params> abstract_getters::Field<#field_struct> for &'__top_level #struct_name #ty_generics #where_clause {
                type Type = &'__top_level #ty;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type { &self.#field_name }
            }
            impl<'__top_level, #struct_params> abstract_getters::Field<#field_struct> for &'__top_level mut #struct_name #ty_generics #where_clause {
                type Type = &'__top_level mut #ty;
                fn field(self) -> <Self as abstract_getters::Field<#field_struct>>::Type { &mut self.#field_name }
            }
        }
    }
}
