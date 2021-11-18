extern crate proc_macro;
use proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Policied, attributes(policy_protected))]
pub fn policied_derive(input: TokenStream) -> TokenStream {
  // println!("input {:#?}", input);

  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  // get the name of the type we want to implement the trait for
  let name = &input.ident;

  // Find the name of members we need to duplicate
  let mut protected: Vec<(syn::Ident, syn::Ident)> = vec![];

  match input.data {
      // Only process structs
      syn::Data::Struct(ref data_struct) => {
          // Check the kind of fields the struct contains
          match data_struct.fields {
              // Structs with named fields
              syn::Fields::Named(ref fields_named) => {
                  // Iterate over the fields
                  for field in fields_named.named.iter() {
                      // Get attributes #[..] on each field
                      for attr in field.attrs.iter() {
                          // Parse the attribute
                          let meta = attr.parse_meta().unwrap();
                          if meta.name().to_string() == "policy_protected" 
                              {
                                match meta {
                                  syn::Meta::List(ml) => {
                                    for nested_meta in ml.nested.iter() {
                                      match nested_meta {
                                        syn::NestedMeta::Meta(m) => {
                                          // Save the protected elements
                                          let attr = field.clone();
                                          protected.push((attr.ident.unwrap(), m.clone().name()));
                                        }
                                        _ => (),
                                      }
                                    }
                                  }
                                  _ => (),
                                }
                              }
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

  let expanded_protected = protected.iter().fold(
    quote!(), |es, (name, ty)| quote! {
      #es
      pub fn #name(&self) -> #ty {
        #ty::make(
          self.#name.clone(),
          self.policy.clone()
        )
      }
    });

  println!("expanded {:#?}", expanded_protected);

  let expanded_derive = quote! {
    impl Policied for #name {
      fn get_policy(&self) -> &Box<dyn Policy> { &self.policy }
    }

    impl #name {
      #expanded_protected
    }
  };

  

  TokenStream::from(expanded_derive)
}
