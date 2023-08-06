// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{ActionGroup,DBusConnection,RemoteActionGroup};
use glib::{translate::*};
use std::{fmt};

glib::wrapper! {
    #[doc(alias = "GDBusActionGroup")]
    pub struct DBusActionGroup(Object<ffi::GDBusActionGroup>) @implements ActionGroup, RemoteActionGroup;

    match fn {
        type_ => || ffi::g_dbus_action_group_get_type(),
    }
}

impl DBusActionGroup {
    #[doc(alias = "g_dbus_action_group_get")]
    pub fn get(connection: &DBusConnection, bus_name: Option<&str>, object_path: &str) -> DBusActionGroup {
        unsafe {
            from_glib_full(ffi::g_dbus_action_group_get(connection.to_glib_none().0, bus_name.to_glib_none().0, object_path.to_glib_none().0))
        }
    }
}

impl fmt::Display for DBusActionGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("DBusActionGroup")
    }
}
