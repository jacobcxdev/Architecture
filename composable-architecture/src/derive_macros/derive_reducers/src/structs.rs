use proc_macro::TokenStream;

use quote::quote;
use syn::{DataStruct, Ident};

use crate::util;

pub fn derive_macro(identifier: Ident, data: DataStruct) -> TokenStream {
    let child_reducers = data
        .fields
        .iter()
        .filter(|field| {
            field.attrs.iter().all(|attr| {
                !attr.path().is_ident("reducer")
                    || attr
                    .parse_args::<Ident>()
                    .map(|arg| arg != "skip")
                    .unwrap_or(true)
            })
        })
        .map(|field| {
            let name = &field.ident;
            let ty = &field.ty;

            if util::is_keyed_state(ty) {
                let into_state = quote! { self.#name };
                let recurse = util::keyed_child_reduce(into_state);

                quote! { #recurse }
            } else {
                quote! {
                    if let Ok(action) = action.clone().try_into() {
                        composable::Reducer::reduce(&mut self.#name, action, send.scope());
                    }
                }
            }
        });

    let expanded = quote! {
        #[automatically_derived]
        impl composable::Reducer for #identifier
            where <Self as RecursiveReducer>::Action: Clone
        {
            type Action = <Self as RecursiveReducer>::Action;
            type Output = Self;

            fn reduce(
                &mut self,
                action: Self::Action,
                send: impl composable::Effects<Self::Action>,
            ) {
                <Self as RecursiveReducer>::reduce(self, action.clone(), send.clone());

                #( #child_reducers )*
            }
        }
    };

    TokenStream::from(expanded)
}