// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::TextBuffer;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib::glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct TargetList(Shared<ffi::GtkTargetList>);

    match fn {
        ref => |ptr| ffi::gtk_target_list_ref(ptr),
        unref => |ptr| ffi::gtk_target_list_unref(ptr),
        get_type => || ffi::gtk_target_list_get_type(),
    }
}

impl TargetList {
    #[doc(alias = "gtk_target_list_add")]
    pub fn add(&self, target: &gdk::Atom, flags: u32, info: u32) {
        unsafe {
            ffi::gtk_target_list_add(self.to_glib_none().0, target.to_glib_none().0, flags, info);
        }
    }

    #[doc(alias = "gtk_target_list_add_image_targets")]
    pub fn add_image_targets(&self, info: u32, writable: bool) {
        unsafe {
            ffi::gtk_target_list_add_image_targets(self.to_glib_none().0, info, writable.to_glib());
        }
    }

    #[doc(alias = "gtk_target_list_add_rich_text_targets")]
    pub fn add_rich_text_targets<P: IsA<TextBuffer>>(
        &self,
        info: u32,
        deserializable: bool,
        buffer: &P,
    ) {
        unsafe {
            ffi::gtk_target_list_add_rich_text_targets(
                self.to_glib_none().0,
                info,
                deserializable.to_glib(),
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_target_list_add_text_targets")]
    pub fn add_text_targets(&self, info: u32) {
        unsafe {
            ffi::gtk_target_list_add_text_targets(self.to_glib_none().0, info);
        }
    }

    #[doc(alias = "gtk_target_list_add_uri_targets")]
    pub fn add_uri_targets(&self, info: u32) {
        unsafe {
            ffi::gtk_target_list_add_uri_targets(self.to_glib_none().0, info);
        }
    }

    #[doc(alias = "gtk_target_list_find")]
    pub fn find(&self, target: &gdk::Atom) -> Option<u32> {
        unsafe {
            let mut info = mem::MaybeUninit::uninit();
            let ret = from_glib(ffi::gtk_target_list_find(
                self.to_glib_none().0,
                target.to_glib_none().0,
                info.as_mut_ptr(),
            ));
            let info = info.assume_init();
            if ret {
                Some(info)
            } else {
                None
            }
        }
    }

    #[doc(alias = "gtk_target_list_remove")]
    pub fn remove(&self, target: &gdk::Atom) {
        unsafe {
            ffi::gtk_target_list_remove(self.to_glib_none().0, target.to_glib_none().0);
        }
    }
}
