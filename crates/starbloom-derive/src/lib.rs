use proc_macro::*;
use quote::quote;

#[proc_macro_derive(StateSetManager)]
pub fn derive_state_set_manager(input: TokenStream) -> TokenStream {
    let input: syn::DeriveInput = syn::parse_macro_input!(input as syn::DeriveInput);
    if let syn::Data::Struct(data) = input.data {
        let t: &syn::Type = &data
            .fields
            .iter()
            .find_map(move |field: &syn::Field| {
                if let Some(ident) = &field.ident {
                    if ident.to_string() == "state" {
                        return Some(field);
                    }
                }
                None
            })
            .expect("foo").ty;
        let name: syn::Ident = input.ident;
        return TokenStream::from(quote!(
            impl crate::StateSetManager<#t> for #name {
                fn configure(&self, schedule: &mut bevy_ecs::prelude::Schedule, state: #t) -> &Self {
                    schedule.configure_sets(state.run_if(move |manager: bevy_ecs::prelude::Res<Self>| manager.state == state));
                    self
                }
            }
        ));
    }
    TokenStream::from(syn::Error::new(input.ident.span(), "not a struct").to_compile_error())
}
