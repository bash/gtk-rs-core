use gio::prelude::*;
use gio::subclass::prelude::*;

glib::wrapper! {
    pub struct SampleApplication(ObjectSubclass<imp::SampleApplication>)
        @extends gio::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for SampleApplication {
    fn default() -> Self {
        glib::Object::builder()
            .property(
                "application-id",
                "com.github.gtk-rs.examples.ReceiveDBusSignals",
            )
            .build()
    }
}

mod imp {
    use std::cell::RefCell;

    use crate::mpris::Mpris;
    use gio::prelude::*;
    use gio::subclass::prelude::*;
    use gio::{BusType, WeakSignalSubscription, bus_get_future};

    #[derive(Default)]
    pub struct SampleApplication {
        mpris: Mpris,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SampleApplication {
        const NAME: &'static str = "SampleApplication";

        type Type = super::SampleApplication;

        type ParentType = gio::Application;
    }

    impl ObjectImpl for SampleApplication {}

    impl ApplicationImpl for SampleApplication {
        fn startup(&self) {
            self.parent_startup();

            glib::spawn_future_local(glib::clone!(
                #[strong(rename_to = obj)]
                self.obj(),
                async move {
                    let session_bus = bus_get_future(BusType::Session).await.unwrap();
                    let mpris = &obj.imp().mpris;
                    mpris
                        .export(&session_bus, "/org/mpris/MediaPlayer2")
                        .unwrap();
                }
            ));
        }

        fn activate(&self) {}
    }
}

fn main() -> glib::ExitCode {
    let app = SampleApplication::default();
    let _guard = app.hold();
    app.run()
}

mod mpris {
    use gio::subclass::prelude::*;

    glib::wrapper! {
        pub(crate) struct Mpris(ObjectSubclass<imp::Mpris>)
            @extends gio::DBusInterfaceSkeleton,
            @implements gio::DBusInterface;
    }

    impl Default for Mpris {
        fn default() -> Self {
            glib::Object::new()
        }
    }

    mod imp {
        use super::*;
        use glib::variant::ToVariant;

        #[derive(Default, Debug)]
        pub(crate) struct Mpris {}

        #[glib::object_subclass]
        impl ObjectSubclass for Mpris {
            const NAME: &'static str = "SampleMpris";
            type Type = super::Mpris;
            type ParentType = gio::DBusInterfaceSkeleton;
        }

        impl ObjectImpl for Mpris {}

        impl DBusInterfaceSkeletonImpl for Mpris {
            fn info() -> gio::DBusInterfaceInfo {
                static INTROSPECTION_XML: &str = include_str!("introspection.xml");
                let node_info = gio::DBusNodeInfo::for_xml(INTROSPECTION_XML)
                    .expect("introspection XML should be valid");
                node_info
                    .lookup_interface("org.mpris.MediaPlayer2")
                    .expect("introspection XML should contain interface 'org.mpris.MediaPlayer2'")
            }

            fn vtable() -> gio::DBusInterfaceVTable<Self> {
                gio::DBusInterfaceVTable::builder()
                    .method_call(|_, _, _, _, _, _, _, invocation| {
                        invocation
                            .return_error(gio::DBusError::NotSupported, "not yet implemented");
                    })
                    .get_property(|_, _, _, _, _, property_name| match property_name {
                        "CanQuit" => Ok(true.to_variant()),
                        "CanRaise" => Ok(true.to_variant()),
                        "CanSetFullscreen" => Ok(false.to_variant()),
                        _ => Err(glib::Error::new(
                            gio::DBusError::UnknownProperty,
                            "unknown property",
                        )),
                    })
                    .build()
            }

            fn get_properties(&self) -> glib::Variant {
                glib::VariantDict::default().into()
            }
        }

        // #[gio::dbus_interface(name = "org.mpris.MediaPlayer2", xml_path = "introspection.xml")]
        // impl Mpris {
        //     fn raise(&self) {}

        //     fn quit(&self) {}

        //     #[dbus(property)]
        //     fn can_quit(&self) -> bool {
        //         false
        //     }

        //     #[dbus(property)]
        //     fn fullscreen(&self) -> bool {
        //         false
        //     }

        //     #[dbus(property)]
        //     fn can_set_fullscreen(&self) -> bool {
        //         false
        //     }

        //     #[dbus(property)]
        //     fn can_raise(&self) -> bool {
        //         false
        //     }

        //     #[dbus(property)]
        //     fn has_track_list(&self) -> bool {
        //         true
        //     }

        //     #[dbus(property)]
        //     fn identity(&self) -> String {
        //         gettext("Ripples")
        //     }

        //     #[dbus(property)]
        //     fn desktop_entry(&self) -> &'static str {
        //         &config::APP_ID
        //     }

        //     #[dbus(property)]
        //     fn supported_uri_schemes(&self) -> &'static [&'static str] {
        //         &[]
        //     }

        //     #[dbus(property)]
        //     fn supported_mime_types(&self) -> &'static [&'static str] {
        //         &[]
        //     }
        // }
    }
}
