// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{DBusConnection, DBusMethodInvocation};

type MethodCallVFunc<T> = fn(
    &T,
    &DBusConnection,
    Option<&str>,
    &str,
    Option<&str>,
    &str,
    &glib::Variant,
    DBusMethodInvocation,
);
type GetPropertyVFunc<T> =
    fn(&T, &DBusConnection, Option<&str>, &str, &str, &str) -> Result<glib::Variant, glib::Error>;
type SetPropertyVFunc<T> = fn(
    &T,
    &DBusConnection,
    Option<&str>,
    &str,
    &str,
    &str,
    &glib::Variant,
) -> Result<(), glib::Error>;

#[non_exhaustive]
pub struct DBusInterfaceVTable<T> {
    pub method_call: MethodCallVFunc<T>,
    pub get_property: Option<GetPropertyVFunc<T>>,
    pub set_property: Option<SetPropertyVFunc<T>>,
}

impl<T> DBusInterfaceVTable<T> {
    pub fn builder() -> DBusInterfaceVTableBuilder<T> {
        DBusInterfaceVTableBuilder::default()
    }
}

pub struct DBusInterfaceVTableBuilder<T> {
    pub method_call: Option<MethodCallVFunc<T>>,
    pub get_property: Option<GetPropertyVFunc<T>>,
    pub set_property: Option<SetPropertyVFunc<T>>,
}

impl<T> Default for DBusInterfaceVTableBuilder<T> {
    fn default() -> Self {
        Self {
            method_call: None,
            get_property: None,
            set_property: None,
        }
    }
}

impl<T> DBusInterfaceVTableBuilder<T> {
    pub fn method_call(mut self, f: MethodCallVFunc<T>) -> Self {
        self.method_call = Some(f);
        self
    }

    pub fn get_property(mut self, f: GetPropertyVFunc<T>) -> Self {
        self.get_property = Some(f);
        self
    }

    pub fn set_property(mut self, f: SetPropertyVFunc<T>) -> Self {
        self.set_property = Some(f);
        self
    }

    /// # Panics
    /// This method will panic if [`Self::method_call`] was never called.
    #[must_use]
    pub fn build(self) -> DBusInterfaceVTable<T> {
        let Self {
            method_call,
            get_property,
            set_property,
        } = self;
        let method_call = method_call.expect("a `method_call` handler should be configured");
        DBusInterfaceVTable {
            method_call,
            get_property,
            set_property,
        }
    }
}
