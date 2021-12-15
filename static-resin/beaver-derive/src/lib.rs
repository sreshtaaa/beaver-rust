extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::{TokenStream};
use syn::Meta;
use syn::export::Span;

// TODO: better error messages!
#[proc_macro_derive(Policied, attributes(policied, policy_protected))]
pub fn policied_derive(input: TokenStream) -> TokenStream {

  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  // get the name of the type we want to implement the trait for
  let unpolicied_name = &input.ident;
  let mut policied_name = unpolicied_name.clone();

  for attr in input.attrs.iter() {
    match attr.parse_meta().unwrap() {
      Meta::List(inner_list) => {
        for ty in inner_list.nested.iter() {
          match ty {
            syn::NestedMeta::Meta(ty_meta) => {
              policied_name = ty_meta.clone().name();
              
            }
            _ => panic!("Inner list must be type, not string literal"),
          }
        }
      },
      _ => panic!("policied attr must designate Policied type")
    }
    
  }

  // all fields - name, type, and whether it is policy-protected
  let mut all_fields: Vec<(syn::Ident, syn::Type, Option<syn::Ident>, bool)> = vec![];
  match input.data {
      // Only process structs
      syn::Data::Struct(ref data_struct) => {
          // Check the kind of fields the struct contains
          match data_struct.fields {
              // Structs with named fields
              syn::Fields::Named(ref fields_named) => {
                  // Iterate over the fields
                  for field in fields_named.named.iter() {
                      let field_name = field.clone().ident.unwrap();
                      let mut is_policy_protected = false;
                      // Get attributes #[..] on each field
                      for attr in field.attrs.iter() {
                          // Parse the attribute
                          let meta = attr.parse_meta().unwrap();
                          if meta.name().to_string() == "policy_protected" {
                            match meta {
                              syn::Meta::List(inner_list) => {
                                // Get nested return types #[policy_protected(...)]
                                for ty in inner_list.nested.iter() {
                                  match ty {
                                    syn::NestedMeta::Meta(ty_meta) => {
                                      is_policy_protected = true;
                                      all_fields.push((field_name.clone(), field.clone().ty, Some(ty_meta.clone().name()), true));
                                    }
                                    _ => panic!("Inner list must be type, not string literal"),
                                  }
                                }
                              }
                              _ => panic!("Must have return type in inner list"),
                            }
                          } 
                      }
                      if !is_policy_protected {
                        all_fields.push((field_name.clone(), field.clone().ty, None, false));
                      }
                  }
              }

              // Struct with unnamed fields
              _ => (),
          }
      }

      // Panic when we don't have a struct
      _ => panic!("Must be a struct"),
  }

  let make_decomposed_arguments = all_fields.clone().iter().fold(
    quote!(), |es, (name, ty_original, ty_protected, is_protected)| 
    if *is_protected {
      match ty_protected {
        Some(ty) => {
          quote! { #es #name: #ty, }
        },
        None => panic!("No protected type")
      }
    } else {
      quote! {
        #es #name: #ty_original,
      }
    }
  );

  let policies = all_fields.clone().iter().fold(
    quote!(let new_policy = policy), |es, (name, _, _, is_protected)|
    if *is_protected {
      quote! {
        #es.merge(#name.get_policy()).unwrap()
      }
    } else {
      quote!{#es}
    }
  );

  let make_decomposed_constructor_inner = all_fields.clone().iter().fold(
    quote!(), |es, (name, _, _, is_protected)| 
    if *is_protected {
      quote! {
        #es
        #name: #name.export(),
      }
    } else {
      quote! {
        #es
        #name,
      }
    }
  );

  let make_decomposed = quote! {
    pub fn make_decomposed(#make_decomposed_arguments policy: Box<dyn Policy>) -> Self {
      #policies;
      #policied_name::make(
        #unpolicied_name {
          #make_decomposed_constructor_inner
        },
        new_policy
      )
    }
  };

  let expanded_protected = all_fields.clone().iter().fold(
    quote!(), |es, (name, _, ty_protected, is_protected)| 
    if *is_protected {
      match ty_protected {
        Some(ty) => {
          quote! { #es
            pub fn #name(&self) -> #ty {
              #ty::make(
                self.inner.clone().#name,
                self.policy.clone()
              )}}
        },
        None => panic!("No protected type")
      }
      
    } else {
      quote!(#es)
    });
  
  let expanded_derive = quote! {
    impl #policied_name {
      #make_decomposed
      #expanded_protected
    }
  };

  

  TokenStream::from(expanded_derive)
}
