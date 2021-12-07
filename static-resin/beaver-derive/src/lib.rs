extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Policied, attributes(policy_protected))]
pub fn policied_derive(input: TokenStream) -> TokenStream {

  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  // get the name of the type we want to implement the trait for
  let name = &input.ident;

  // Find the name of members we need to duplicate
  let mut protected: Vec<(syn::Ident, syn::Ident)> = vec![];
  let mut fields: Vec<(syn::Ident, syn::Type)> = vec![];

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
                          if meta.name().to_string() == "policy_protected" {
                            match meta {
                              syn::Meta::List(inner_list) => {
                                // Get nested return types #[policy_protected(...)]
                                for ty in inner_list.nested.iter() {
                                  match ty {
                                    syn::NestedMeta::Meta(ty_meta) => {
                                      // Save the protected elements
                                      let attr = field.clone();
                                      protected.push((attr.ident.unwrap(), ty_meta.clone().name()));
                                    }
                                    _ => panic!("Inner list must be type, not string literal"),
                                  }
                                }
                              }
                              _ => panic!("Must have return type in inner list"),
                            }
                          }
                      }
                      let field_clone = field.clone();
                      let field_name = field_clone.ident.unwrap().to_string();
                      if !field_name.eq("policy") {
                        fields.push((field.clone().ident.unwrap(), field.clone().ty))
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

  let expanded_fields = fields.iter().fold(
    quote!(), |es, (name, ty)| quote! {
      #es
      pub #name: #ty
  });

  let expanded_set_fields = fields.iter().fold(
    quote!(), |es, (name, _)| quote! {
      #es
      #name: self.#name
  });

  let mut new_type = name.to_string().clone();
  new_type.push_str("Unpolicied");
  let new_name = syn::Ident::new(&new_type, name.span());

  let expanded_derive = quote! {
    pub struct #new_name {
      #expanded_fields
    }

    impl Policied for #name {
      fn get_policy(&self) -> &Box<dyn Policy> { &self.policy }
      fn remove_policy<#new_type>(self) -> #new_type { 
        #new_name {
          #expanded_set_fields
        }
      }
    }

    impl #name {
      #expanded_protected
    }
  };

  

  TokenStream::from(expanded_derive)
}
