// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::crate_ident_new;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

mod utils;

// placed on the impl with the methods and properties
#[proc_macro_derive(DBusInterfaceSkeleton)]
pub fn derive_dbus_interface_skeleton(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let ident = &input.ident;
    let krate = crate_ident_new();
    quote! {
        impl #krate::subclass::prelude::DBusInterfaceSkeletonImpl for #ident {
            unsafe fn get_info(&self) -> *mut #krate::ffi::GDBusInterfaceInfo {
                todo!()
            }

            unsafe fn get_vtable(&self) -> *mut #krate::ffi::GDBusInterfaceVTable {
                todo!()
            }

            fn get_properties(&self) -> #krate::glib::Variant {
                todo!()
            }
        }
    }
    .into()
}

// placed on the DBusInterfaceSkeletonImpl impl
#[proc_macro_attribute]
pub fn dbus_interface(attr: TokenStream, input: TokenStream) -> TokenStream {
    input
}
