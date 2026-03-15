// Take a look at the license at the top of the repository in the LICENSE file.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::manual_c_str_literals)]
#![doc = include_str!("../README.md")]

pub use gio_sys as ffi;
pub use glib;

mod action_entry;
mod action_map;
#[cfg(feature = "v2_60")]
mod app_info;
mod application;
pub use action_entry::{ActionEntry, ActionEntryBuilder};
pub use application::{ApplicationBusyGuard, ApplicationHoldGuard};
mod application_command_line;
mod async_initable;
mod cancellable;
pub use cancellable::CancelledHandlerId;
mod cancellable_future;
pub use crate::cancellable_future::{CancellableFuture, Cancelled};
mod content_type;
mod converter;
mod credentials;
mod data_input_stream;
mod datagram_based;
mod dbus;
pub use self::dbus::*;
mod dbus_connection;
pub use self::dbus_connection::{
    ActionGroupExportId, DBusSignalRef, FilterId, MenuModelExportId, RegistrationBuilder,
    RegistrationId, SignalSubscription, SignalSubscriptionId, SubscribedSignalStream, WatcherId,
    WeakSignalSubscription,
};
mod dbus_interface_info;
mod dbus_message;
mod dbus_method_invocation;
mod dbus_node_info;
#[cfg(feature = "v2_72")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
mod debug_controller_dbus;
mod error;
mod file;
mod file_attribute_info;
pub use crate::file_attribute_info::FileAttributeInfo;
mod file_attribute_info_list;
mod file_attribute_matcher;
pub use crate::file_attribute_matcher::FileAttributematcherIter;
mod file_attribute_value;
pub use file_attribute_value::FileAttributeValue;
mod file_enumerator;
pub use crate::file_enumerator::FileEnumeratorStream;
mod file_info;
mod flags;
mod inet_address;
pub use crate::inet_address::InetAddressBytes;
mod inet_socket_address;
mod io_stream;
pub use crate::io_stream::IOStreamAsyncReadWrite;
mod initable;
mod input_stream;
pub use crate::input_stream::{InputStreamAsyncBufRead, InputStreamRead};
mod list_model;
mod list_store;
#[cfg(test)]
mod memory_input_stream;
#[cfg(test)]
mod memory_output_stream;
mod output_stream;
pub use crate::output_stream::OutputStreamWrite;
mod pollable_input_stream;
pub use crate::pollable_input_stream::InputStreamAsyncRead;
mod pollable_output_stream;
pub use crate::pollable_output_stream::OutputStreamAsyncWrite;
mod resource;
pub use crate::resource::resources_register_include_impl;
mod settings;
pub use crate::settings::BindingBuilder;
mod simple_proxy_resolver;
mod socket;
pub use socket::{InputMessage, InputVector, OutputMessage, OutputVector, SocketControlMessages};
mod dbus_object_manager_client;
mod socket_control_message;
mod socket_listener;
mod subprocess;
mod subprocess_launcher;
mod threaded_socket_service;
#[cfg(unix)]
mod unix_fd_list;
#[cfg(unix)]
mod unix_socket_address;

#[cfg(test)]
mod test_util;

pub mod builders {
    pub use super::async_initable::AsyncInitableBuilder;
    pub use super::auto::builders::*;
    pub use super::initable::InitableBuilder;
}

pub mod functions {
    pub use super::auto::functions::*;
    pub use super::content_type::content_type_guess;
}

pub use crate::auto::*;
pub use crate::functions::*;
pub mod prelude;

#[allow(clippy::missing_safety_doc)]
#[allow(clippy::new_ret_no_self)]
#[allow(unused_imports)]
#[allow(clippy::let_and_return)]
mod auto;

mod gio_future;
pub use crate::gio_future::*;

mod io_extension;
pub use crate::io_extension::*;

mod io_extension_point;
pub use crate::io_extension_point::*;

mod io_module;
pub use crate::io_module::*;

mod io_module_scope;
pub use crate::io_module_scope::*;

mod task;
pub use crate::task::*;

#[macro_use]
pub mod subclass;
mod read_input_stream;
pub use crate::read_input_stream::ReadInputStream;
mod write_output_stream;
pub use crate::write_output_stream::WriteOutputStream;
use glib::object::IsA;
use glib::signal::{DetailedSignalDescriptor, SignalDescriptor, TypedSignalGroup};
mod dbus_proxy;
mod tls_connection;

pub struct ActionGroupActionAddedSignal;

pub trait ActionGroupExt: IsA<ActionGroup> + 'static {
    const ACTION_ADDED: ActionGroupActionAddedSignal = ActionGroupActionAddedSignal;
}

impl<O: IsA<ActionGroup>> ActionGroupExt for O {}

impl<O: IsA<ActionGroup>> SignalDescriptor<O> for ActionGroupActionAddedSignal {
    const NAME: &str = "action-added";
    type Args<'a> = (&'a str,);
    type HandlerArgs<'a> = (&'a str,);
    type Output = ();
}

impl<O: IsA<ActionGroup>> DetailedSignalDescriptor<O> for ActionGroupActionAddedSignal {
    fn connect<F: for<'a> Fn(&'a O, Self::Args<'a>) -> Self::Output + 'static>(
        target: &O,
        detail: Option<&str>,
        f: F,
    ) -> glib::SignalHandlerId {
        use glib::object::Cast as _;
        use glib::signal::connect_raw_with_after;
        use glib::translate::FromGlibPtrBorrow as _;

        unsafe extern "C" fn action_removed_trampoline<
            P: glib::object::IsA<ActionGroup>,
            F: for<'a> Fn(&'a P, (&'a str,)) + 'static,
        >(
            this: *mut ffi::GActionGroup,
            action_name: *mut std::ffi::c_char,
            f: glib::ffi::gpointer,
        ) {
            unsafe {
                let f: &F = &*(f as *const F);
                f(
                    ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
                    (&glib::GString::from_glib_borrow(action_name),),
                )
            }
        }
        unsafe {
            let f: std::boxed::Box<F> = std::boxed::Box::new(f);
            let detailed_signal_name = detail.map(|name| format!("action-removed::{name}\0"));
            let signal_name: &[u8] = detailed_signal_name
                .as_ref()
                .map_or(c"action-removed".to_bytes(), |n| n.as_bytes());
            connect_raw_with_after(
                target.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    action_removed_trampoline::<O, F> as *const (),
                )),
                std::boxed::Box::into_raw(f),
                false,
            )
        }
    }

    fn emit(target: &O, detail: Option<&str>, args: Self::Args<'_>) -> Self::Output {
        todo!()
    }

    // fn connect_to_signal_group<F: for<'a> Fn(Self::Args<'a>) -> Self::Output + 'static>(
    //     target: &ActionGroup,
    //     detail: Option<&str>,
    //     handler: F,
    // ) -> glib::SignalHandlerId {
    //     todo!()
    // }
}

// impl ActionGroup {
//     // TODO: conflicts with the virtual method
//     pub fn action_added(&self) -> ActionGroupActionAddedSignalProxy<'_> {
//         ActionGroupActionAddedSignalProxy(self)
//     }
// }

// pub struct ActionGroupActionAddedSignalProxy<'a>(&'a ActionGroup);

// impl glib::signal::SignalProxy<ActionGroup> for ActionGroupActionAddedSignalProxy<'_> {
//     const NAME: &'static str = "action-added";

//     fn object(&self) -> &ActionGroup {
//         self.0
//     }
// }

// impl ActionGroupActionAddedSignalProxy<'_> {
//     pub fn connect<F: Fn(&ActionGroup, &str) + 'static>(
//         &self,
//         detail: Option<&str>,
//         f: F,
//     ) -> glib::SignalHandlerId {
//         self.connect_impl(detail, f, false)
//     }

//     pub fn connect_after<F: Fn(&ActionGroup, &str) + 'static>(
//         &self,
//         detail: Option<&str>,
//         f: F,
//     ) -> glib::SignalHandlerId {
//         self.connect_impl(detail, f, true)
//     }

//     // Q: is the first argument always the detail?
//     pub fn emit(&self, details: Option<&str>, action_name: &str) {
//         use glib::object::ObjectExt as _;
//         use glib::value::ToValue as _;
//         let details = details.map(glib::Quark::from_str);
//         let values = &[action_name.to_value()];
//         if let Some(details) = details {
//             self.0
//                 .emit_by_name_with_details_and_values("action-removed", details, values);
//         } else {
//             self.0.emit_by_name_with_values("action-removed", values);
//         }
//     }

//     fn connect_impl<F: Fn(&ActionGroup, &str) + 'static>(
//         &self,
//         detail: Option<&str>,
//         f: F,
//         after: bool,
//     ) -> glib::SignalHandlerId {
//         use glib::object::Cast as _;
//         use glib::object::ObjectType as _;
//         use glib::signal::connect_raw_with_after;
//         use glib::translate::FromGlibPtrBorrow as _;

//         unsafe extern "C" fn action_removed_trampoline<
//             P: glib::object::IsA<ActionGroup>,
//             F: Fn(&P, &str) + 'static,
//         >(
//             this: *mut ffi::GActionGroup,
//             action_name: *mut std::ffi::c_char,
//             f: glib::ffi::gpointer,
//         ) {
//             unsafe {
//                 let f: &F = &*(f as *const F);
//                 f(
//                     ActionGroup::from_glib_borrow(this).unsafe_cast_ref(),
//                     &glib::GString::from_glib_borrow(action_name),
//                 )
//             }
//         }
//         unsafe {
//             let f: std::boxed::Box<F> = std::boxed::Box::new(f);
//             let detailed_signal_name = detail.map(|name| format!("action-removed::{name}\0"));
//             let signal_name: &[u8] = detailed_signal_name
//                 .as_ref()
//                 .map_or(c"action-removed".to_bytes(), |n| n.as_bytes());
//             connect_raw_with_after(
//                 self.0.as_ptr() as *mut _,
//                 signal_name.as_ptr() as *const _,
//                 Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
//                     action_removed_trampoline::<ActionGroup, F> as *const (),
//                 )),
//                 std::boxed::Box::into_raw(f),
//                 after,
//             )
//         }
//     }
// }

fn example_signal_group() {
    let signal_group = TypedSignalGroup::<ActionGroup>::default();
    signal_group.connect(ActionGroup::ACTION_ADDED, |a, (action_name,)| todo!());
}

fn example_object(action_group: &ActionGroup) {
    use glib::signal::GugusExt as _;
    action_group.connect(ActionGroup::ACTION_ADDED, |a, (action_name,)| todo!());
}
