// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Icon;
use crate::LoadableIcon;
use glib::translate::*;
use std::fmt;

glib::glib_wrapper! {
    pub struct BytesIcon(Object<ffi::GBytesIcon>) @implements Icon, LoadableIcon;

    match fn {
        get_type => || ffi::g_bytes_icon_get_type(),
    }
}

impl BytesIcon {
    #[doc(alias = "g_bytes_icon_new")]
    pub fn new(bytes: &glib::Bytes) -> BytesIcon {
        unsafe { from_glib_full(ffi::g_bytes_icon_new(bytes.to_glib_none().0)) }
    }

    #[doc(alias = "g_bytes_icon_get_bytes")]
    pub fn get_bytes(&self) -> Option<glib::Bytes> {
        unsafe { from_glib_none(ffi::g_bytes_icon_get_bytes(self.to_glib_none().0)) }
    }
}

impl fmt::Display for BytesIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("BytesIcon")
    }
}
