// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DBusObjectManager;
use glib::{prelude::*, subclass::prelude::*};

pub trait DBusObjectManagerImpl: ObjectImpl + ObjectSubclass<Type: IsA<DBusObjectManager>> {}

pub trait DBusObjectManagerImplExt: DBusObjectManagerImpl {}

impl<T: DBusObjectManagerImpl> DBusObjectManagerImplExt for T {}

unsafe impl<T: DBusObjectManagerImpl> IsImplementable<T> for DBusObjectManager {
    fn interface_init(_iface: &mut glib::Interface<Self>) {}
}
