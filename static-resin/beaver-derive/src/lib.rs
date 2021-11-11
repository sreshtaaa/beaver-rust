extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Policied)]
pub fn policied_derive(input: TokenStream) -> TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  // get the name of the type we want to implement the trait for
  let name = &input.ident;

  let expanded = quote! {
    impl Policied for #name {
      fn get_policy(&self) -> &Box<dyn Policy> { &self.policy }
      fn remove_policy(&mut self) -> () { self.policy = Box::new(NonePolicy); }
    }
  };

  TokenStream::from(expanded)
}
