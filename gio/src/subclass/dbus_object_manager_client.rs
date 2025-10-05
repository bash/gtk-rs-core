// Take a look at the license at the top of the repository in the LICENSE file.

use crate::DBusObjectManagerClient;
use glib::{prelude::*, subclass::prelude::*};

pub trait DBusObjectManagerClientImpl:
    ObjectImpl + ObjectSubclass<Type: IsA<DBusObjectManagerClient>>
{
}

pub trait DBusObjectManagerClientImplExt: DBusObjectManagerClientImpl {}

impl<T: DBusObjectManagerClientImpl> DBusObjectManagerClientImplExt for T {}

unsafe impl<T: DBusObjectManagerClientImpl> IsSubclassable<T> for DBusObjectManagerClient {
    fn class_init(class: &mut ::glib::Class<Self>) {
        Self::parent_class_init::<T>(class);
    }
}
