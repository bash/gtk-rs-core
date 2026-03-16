// Take a look at the license at the top of the repository in the LICENSE file.

#![deny(unsafe_op_in_unsafe_fn)]

use std::{pin::Pin, ptr};

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{
    DBusInterfaceInfo, DBusInterfaceSkeleton, DBusInterfaceVTable, DBusMethodInvocation, ffi,
};

pub trait DBusInterfaceSkeletonImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<DBusInterfaceSkeleton>>
{
    fn info() -> DBusInterfaceInfo;

    fn vtable() -> DBusInterfaceVTable<Self>;

    fn get_properties(&self) -> glib::Variant;

    // [TODO] required vfunc but I think it is more convenient to have a default no-op, does this make sense?
    fn flush(&self) {}

    fn g_authorize_method(&self, invocation: &DBusMethodInvocation) -> bool {
        self.parent_g_authorize_method(invocation)
    }
}

pub trait DBusInterfaceSkeletonImplExt: DBusInterfaceSkeletonImpl {
    unsafe fn parent_get_info(&self) -> *mut ffi::GDBusInterfaceInfo {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;
            let f = (*parent_class)
                .get_info
                .expect("no parent \"get_info\" implementation");
            f(self
                .obj()
                .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                .to_glib_none()
                .0)
        }
    }

    unsafe fn parent_get_vtable(&self) -> *mut ffi::GDBusInterfaceVTable {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;
            let f = (*parent_class)
                .get_vtable
                .expect("no parent \"get_vtable\" implementation");
            f(self
                .obj()
                .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                .to_glib_none()
                .0)
        }
    }

    fn parent_get_properties(&self) -> glib::Variant {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;
            let f = (*parent_class)
                .get_properties
                .expect("no parent \"get_properties\" implementation");
            from_glib_full(f(self
                .obj()
                .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                .to_glib_none()
                .0))
        }
    }

    fn parent_flush(&self) {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;
            let f = (*parent_class)
                .flush
                .expect("no parent \"flush\" implementation");
            f(self
                .obj()
                .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                .to_glib_none()
                .0);
        }
    }

    fn parent_g_authorize_method(&self, invocation: &DBusMethodInvocation) -> bool {
        unsafe {
            let data = Self::type_data();
            let parent_class =
                data.as_ref().parent_class() as *const ffi::GDBusInterfaceSkeletonClass;

            let f = (*parent_class)
                .g_authorize_method
                .expect("no parent \"g_authorize_method\" implementation");
            from_glib(f(
                self.obj()
                    .unsafe_cast_ref::<DBusInterfaceSkeleton>()
                    .to_glib_none()
                    .0,
                invocation.to_glib_none().0,
            ))
        }
    }
}

impl<T: DBusInterfaceSkeletonImpl> DBusInterfaceSkeletonImplExt for T {}

unsafe impl<T: DBusInterfaceSkeletonImpl> IsSubclassable<T> for DBusInterfaceSkeleton {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
        let class = class.as_mut();
        class.get_info = Some(get_info::<T>);
        class.get_vtable = Some(get_vtable::<T>);
        class.get_properties = Some(get_properties::<T>);
        class.flush = Some(flush::<T>);
        class.g_authorize_method = Some(g_authorize_method::<T>);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        Self::parent_instance_init(instance);
        let info = T::info();
        let vtable = T::vtable();
        let ffi_vtable = Box::pin(create_ffi_vtable(&vtable));
        instance.set_instance_data(
            DBusInterfaceSkeleton::static_type(),
            InstanceData {
                info,
                vtable,
                ffi_vtable,
            },
        );
    }
}

fn create_ffi_vtable<T: DBusInterfaceSkeletonImpl>(
    vtable: &DBusInterfaceVTable<T>,
) -> ffi::GDBusInterfaceVTable {
    ffi::GDBusInterfaceVTable {
        method_call: Some(vfunc_method_call::<T>),
        get_property: vtable
            .get_property
            .is_some()
            .then_some(vfunc_get_property::<T>),
        set_property: vtable
            .set_property
            .is_some()
            .then_some(vfunc_set_property::<T>),
        padding: Default::default(),
    }
}

struct InstanceData<T> {
    info: DBusInterfaceInfo,
    vtable: DBusInterfaceVTable<T>,
    ffi_vtable: Pin<Box<ffi::GDBusInterfaceVTable>>,
}

// [TODO] Safety?
unsafe impl<T> Send for InstanceData<T> {}
// [TODO] Safety?
unsafe impl<T> Sync for InstanceData<T> {}

// SAFETY: The returned pointer lives as long as the skeleton instance,
// because its object is stored as qdata on the skeleton instance.
unsafe extern "C" fn get_info<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceInfo {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    let instance_data = imp
        .instance_data::<InstanceData<T>>(DBusInterfaceSkeleton::static_type())
        .expect("instance data should be initialized");
    // SAFETY: We keep a strong reference to the `DBusInterfaceInfo` in our instance data.
    instance_data.info.to_glib_none().0
}

unsafe extern "C" fn get_vtable<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceVTable {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    let instance_data = imp
        .instance_data::<InstanceData<T>>(DBusInterfaceSkeleton::static_type())
        .expect("instance data should be initialized");
    // SAFETY:
    // * lifetime: We keep the boxed struct alive in our instance data.
    // * mut: *chuckles* I'm in danger.
    (&*instance_data.ffi_vtable as *const ffi::GDBusInterfaceVTable)
        as *mut ffi::GDBusInterfaceVTable
}

unsafe extern "C" fn get_properties<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut glib::ffi::GVariant {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.get_properties().to_glib_full()
}

unsafe extern "C" fn flush<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    imp.flush();
}

unsafe extern "C" fn g_authorize_method<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
    invocation: *mut ffi::GDBusMethodInvocation,
) -> i32 {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    let invocation = unsafe { from_glib_borrow(invocation) };
    imp.g_authorize_method(&invocation).into_glib()
}

#[doc(hidden)]
unsafe extern "C" fn vfunc_method_call<T: DBusInterfaceSkeletonImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    method_name: *const glib::ffi::gchar,
    parameters: *mut glib::ffi::GVariant,
    invocation: *mut ffi::GDBusMethodInvocation,
    user_data: glib::ffi::gpointer,
) {
    let connection = unsafe { from_glib_borrow(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<Option<glib::GString>> =
        unsafe { from_glib_borrow(interface_name) };
    let method_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(method_name) };
    let parameters = unsafe { from_glib_borrow(parameters) };
    let invocation = unsafe { from_glib_full(invocation) };

    // SAFETY: `DBusInterfaceSkeleton.get_vtable` guarantees that the interface skeleton instance
    // is passed as `user_data` to the vfuncs in the vtable.
    // [TODO] this is not sound when the derived class is further derived.
    let instance = unsafe { &*(user_data as *mut T::Instance) };
    let imp = instance.imp();
    let instance_data = imp
        .instance_data::<InstanceData<T>>(DBusInterfaceSkeleton::static_type())
        .expect("instance data should be initialized");

    (instance_data.vtable.method_call)(
        imp,
        &connection,
        sender.as_deref(),
        &object_path,
        interface_name.as_deref(),
        &method_name,
        &parameters,
        invocation,
    );
}

#[doc(hidden)]
unsafe extern "C" fn vfunc_get_property<T: DBusInterfaceSkeletonImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    property_name: *const glib::ffi::gchar,
    error_out: *mut *mut glib::ffi::GError,
    user_data: glib::ffi::gpointer,
) -> *mut glib::ffi::GVariant {
    let connection = unsafe { from_glib_borrow(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(interface_name) };
    let property_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(property_name) };

    // SAFETY: `DBusInterfaceSkeleton.get_vtable` guarantees that the interface skeleton instance
    // is passed as `user_data` to the vfuncs in the vtable.
    // [TODO] this is not sound when the derived class is further derived.
    let instance = unsafe { &*(user_data as *mut T::Instance) };
    let imp = instance.imp();
    let instance_data = imp
        .instance_data::<InstanceData<T>>(DBusInterfaceSkeleton::static_type())
        .expect("instance data should be initialized");

    let get_property_vfunc = instance_data
        .vtable
        .get_property
        .as_ref()
        .expect("get_property should always be set if get_property is set in the ffi vtable");
    let result = (get_property_vfunc)(
        imp,
        &connection,
        sender.as_deref(),
        &object_path,
        &interface_name,
        &property_name,
    );
    match result {
        Ok(variant) => variant.to_glib_full(),
        Err(error) => {
            unsafe { *error_out = error.to_glib_full() };
            ptr::null_mut()
        }
    }
}

#[doc(hidden)]
unsafe extern "C" fn vfunc_set_property<T: DBusInterfaceSkeletonImpl>(
    connection: *mut ffi::GDBusConnection,
    sender: *const glib::ffi::gchar,
    object_path: *const glib::ffi::gchar,
    interface_name: *const glib::ffi::gchar,
    property_name: *const glib::ffi::gchar,
    value: *mut glib::ffi::GVariant,
    error_out: *mut *mut glib::ffi::GError,
    user_data: glib::ffi::gpointer,
) -> glib::ffi::gboolean {
    let connection = unsafe { from_glib_borrow(connection) };
    let sender: Borrowed<Option<glib::GString>> = unsafe { from_glib_borrow(sender) };
    let object_path: Borrowed<glib::GString> = unsafe { from_glib_borrow(object_path) };
    let interface_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(interface_name) };
    let property_name: Borrowed<glib::GString> = unsafe { from_glib_borrow(property_name) };
    let value = unsafe { from_glib_borrow(value) };

    // SAFETY: `DBusInterfaceSkeleton.get_vtable` guarantees that the interface skeleton instance
    // is passed as `user_data` to the vfuncs in the vtable.
    // [TODO] this is not sound when the derived class is further derived.
    let instance = unsafe { &*(user_data as *mut T::Instance) };
    let imp = instance.imp();
    let instance_data = imp
        .instance_data::<InstanceData<T>>(DBusInterfaceSkeleton::static_type())
        .expect("instance data should be initialized");

    let set_property_vfunc = instance_data
        .vtable
        .set_property
        .as_ref()
        .expect("set_property should always be set if set_property is set in the ffi vtable");
    let result = (set_property_vfunc)(
        imp,
        &connection,
        sender.as_deref(),
        &object_path,
        &interface_name,
        &property_name,
        &value,
    );
    match result {
        Ok(()) => true.into_glib(),
        Err(error) => {
            unsafe { *error_out = error.to_glib_full() };
            false.into_glib()
        }
    }
}
