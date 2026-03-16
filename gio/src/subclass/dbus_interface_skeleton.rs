// Take a look at the license at the top of the repository in the LICENSE file.

#![deny(unsafe_op_in_unsafe_fn)]

use glib::{prelude::*, subclass::prelude::*, translate::*};

use crate::{DBusInterfaceSkeleton, DBusMethodInvocation, ffi};

pub trait DBusInterfaceSkeletonImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<DBusInterfaceSkeleton>>
{
    unsafe fn get_info(&self) -> *mut ffi::GDBusInterfaceInfo;

    unsafe fn get_vtable(&self) -> *mut ffi::GDBusInterfaceVTable;

    fn get_properties(&self) -> glib::Variant;

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
}

unsafe extern "C" fn get_info<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceInfo {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    unsafe { imp.get_info() }
}

unsafe extern "C" fn get_vtable<T: DBusInterfaceSkeletonImpl>(
    skeleton: *mut ffi::GDBusInterfaceSkeleton,
) -> *mut ffi::GDBusInterfaceVTable {
    let instance = unsafe { &*(skeleton as *mut T::Instance) };
    let imp = instance.imp();
    unsafe { imp.get_vtable() }
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
