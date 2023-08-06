// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use glib::{prelude::*,translate::*};
use std::{fmt,ptr};

glib::wrapper! {
    #[doc(alias = "GIcon")]
    pub struct Icon(Interface<ffi::GIcon, ffi::GIconIface>);

    match fn {
        type_ => || ffi::g_icon_get_type(),
    }
}

impl Icon {
        pub const NONE: Option<&'static Icon> = None;
    

    #[doc(alias = "g_icon_deserialize")]
    pub fn deserialize(value: &glib::Variant) -> Option<Icon> {
        unsafe {
            from_glib_full(ffi::g_icon_deserialize(value.to_glib_none().0))
        }
    }

    #[doc(alias = "g_icon_new_for_string")]
    #[doc(alias = "new_for_string")]
    pub fn for_string(str: &str) -> Result<Icon, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_icon_new_for_string(str.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Icon>> Sealed for T {}
}

pub trait IconExt: IsA<Icon> + sealed::Sealed + 'static {
    #[doc(alias = "g_icon_equal")]
    fn equal(&self, icon2: Option<&impl IsA<Icon>>) -> bool {
        unsafe {
            from_glib(ffi::g_icon_equal(self.as_ref().to_glib_none().0, icon2.map(|p| p.as_ref()).to_glib_none().0))
        }
    }

    #[doc(alias = "g_icon_hash")]
    fn hash(&self) -> u32 {
        unsafe {
            ffi::g_icon_hash(ToGlibPtr::<*mut ffi::GIcon>::to_glib_none(self.as_ref()).0 as glib::ffi::gconstpointer)
        }
    }

    #[doc(alias = "g_icon_serialize")]
    fn serialize(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_icon_serialize(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "g_icon_to_string")]
    fn to_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_icon_to_string(self.as_ref().to_glib_none().0))
        }
    }
}

impl<O: IsA<Icon>> IconExt for O {}

impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Icon")
    }
}
