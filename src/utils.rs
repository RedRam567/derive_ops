use proc_macro::TokenStream;
use quote::quote;
use syn::Field;
use syn::{Data, DeriveInput, Fields};

// basicly calls a method on each field of self with other, and scalar
pub(crate) fn struct_field_impler(
    input: TokenStream,
    trait_name: &str,
    trait_generics: &str,
    trait_fn: &str,
    prefix: &str,
    suffix: &str,
    ret_new: bool,
) -> TokenStream {
    let trait_name = trait_name.parse::<proc_macro2::TokenStream>().unwrap();
    let trait_generics = trait_generics.parse::<proc_macro2::TokenStream>().unwrap();
    let trait_fn = trait_fn.parse::<proc_macro2::TokenStream>().unwrap();
    let prefix = prefix.parse::<proc_macro2::TokenStream>().unwrap();
    let suffix = suffix.parse::<proc_macro2::TokenStream>().unwrap();

    // Construct a representation of Rust code as a syntax tree that we can manipulate
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;
    // all generic information for this struct or smth
    let generics = ast.generics;
    let (_impl, ty_generics, where_clause) = generics.split_for_impl();

    // get fields of the struct
    let fields: Vec<&Field> = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(f) => f.named.iter().collect(),
            _ => panic!("Only named structs are allowed for this derive."),
        },
        _ => panic!("Only structs allowed this derive."),
    };

    // zip fields with methods
    let mut exprs = vec![];
    let mut exprs_scalar = vec![];
    if ret_new {
        for field in fields {
            let id = field.ident.as_ref().unwrap();
            // generates: self.x.method(rhs.x)
            let expr = quote! {
                #id: self.#id.#trait_fn(rhs.#id)
            };
            exprs.push(expr);

            // generates: self.x.method(rhs)
            let expr_scalar = quote! {
                #id: self.#id.#trait_fn(rhs)
            };
            exprs_scalar.push(expr_scalar);
        }
    } else {
        for field in fields {
            let id = field.ident.as_ref().unwrap();
            // generates: self.x.method(rhs.x)
            let expr = quote! {
                self.#id.#trait_fn(rhs.#id)
            };
            exprs.push(expr);

            // generates: self.x.method(rhs)
            let expr_scalar = quote! {
                self.#id.#trait_fn(rhs)
            };
            exprs_scalar.push(expr_scalar);
        }
    }

    // Build the trait implementation
    if ret_new {
        quote! {
            #[automatically_derived]
            impl<T: #trait_name #trait_generics> #trait_name<&Self> for #name #ty_generics #where_clause {
                #prefix
                fn #trait_fn(self, rhs: &Self) -> Self {
                    // use #trait_name;
                    Self {
                        #( #exprs, )*
                    }
                    #suffix
                }
            }

            #[automatically_derived]
            impl<T: #trait_name #trait_generics> #trait_name<T> for #name #ty_generics #where_clause {
                #prefix
                fn #trait_fn(self, rhs: T) -> Self {
                    // use #trait_name;
                    Self {
                        #( #exprs_scalar, )*
                    }
                    #suffix
                }
            }
        }
        .into()
    } else {
        quote! {
            #[automatically_derived]
            impl<T: #trait_name #trait_generics> #trait_name<&Self> for #name #ty_generics #where_clause {
                #prefix
                fn #trait_fn(&mut self, rhs: &Self) {
                    #( #exprs; )*
                    #suffix
                }
            }

            #[automatically_derived]
            impl<T: #trait_name #trait_generics> #trait_name<T> for #name #ty_generics #where_clause {
                #prefix
                fn #trait_fn(&mut self, rhs: T) {
                    #( #exprs_scalar; )*
                    #suffix
                }
            }
        }
        .into()
    }
}
