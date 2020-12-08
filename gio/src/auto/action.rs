// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use std::ptr;

glib::glib_wrapper! {
    pub struct Action(Interface<ffi::GAction>);

    match fn {
        get_type => || ffi::g_action_get_type(),
    }
}

impl Action {
    #[doc(alias = "g_action_name_is_valid")]
    pub fn name_is_valid(action_name: &str) -> bool {
        unsafe { from_glib(ffi::g_action_name_is_valid(action_name.to_glib_none().0)) }
    }

    #[doc(alias = "g_action_parse_detailed_name")]
    pub fn parse_detailed_name(
        detailed_name: &str,
    ) -> Result<(glib::GString, glib::Variant), glib::Error> {
        unsafe {
            let mut action_name = ptr::null_mut();
            let mut target_value = ptr::null_mut();
            let mut error = ptr::null_mut();
            let _ = ffi::g_action_parse_detailed_name(
                detailed_name.to_glib_none().0,
                &mut action_name,
                &mut target_value,
                &mut error,
            );
            if error.is_null() {
                Ok((from_glib_full(action_name), from_glib_full(target_value)))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_action_print_detailed_name")]
    pub fn print_detailed_name(
        action_name: &str,
        target_value: Option<&glib::Variant>,
    ) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_action_print_detailed_name(
                action_name.to_glib_none().0,
                target_value.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_ACTION: Option<&Action> = None;

pub trait ActionExt: 'static {
    #[doc(alias = "g_action_activate")]
    fn activate(&self, parameter: Option<&glib::Variant>);

    #[doc(alias = "g_action_change_state")]
    fn change_state(&self, value: &glib::Variant);

    #[doc(alias = "g_action_get_enabled")]
    fn get_enabled(&self) -> bool;

    #[doc(alias = "g_action_get_name")]
    fn get_name(&self) -> Option<glib::GString>;

    #[doc(alias = "g_action_get_parameter_type")]
    fn get_parameter_type(&self) -> Option<glib::VariantType>;

    #[doc(alias = "g_action_get_state")]
    fn get_state(&self) -> Option<glib::Variant>;

    #[doc(alias = "g_action_get_state_hint")]
    fn get_state_hint(&self) -> Option<glib::Variant>;

    #[doc(alias = "g_action_get_state_type")]
    fn get_state_type(&self) -> Option<glib::VariantType>;

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Action>> ActionExt for O {
    fn activate(&self, parameter: Option<&glib::Variant>) {
        unsafe {
            ffi::g_action_activate(self.as_ref().to_glib_none().0, parameter.to_glib_none().0);
        }
    }

    fn change_state(&self, value: &glib::Variant) {
        unsafe {
            ffi::g_action_change_state(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_enabled(&self) -> bool {
        unsafe { from_glib(ffi::g_action_get_enabled(self.as_ref().to_glib_none().0)) }
    }

    fn get_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_action_get_name(self.as_ref().to_glib_none().0)) }
    }

    fn get_parameter_type(&self) -> Option<glib::VariantType> {
        unsafe {
            from_glib_none(ffi::g_action_get_parameter_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_state(&self) -> Option<glib::Variant> {
        unsafe { from_glib_full(ffi::g_action_get_state(self.as_ref().to_glib_none().0)) }
    }

    fn get_state_hint(&self) -> Option<glib::Variant> {
        unsafe { from_glib_full(ffi::g_action_get_state_hint(self.as_ref().to_glib_none().0)) }
    }

    fn get_state_type(&self) -> Option<glib::VariantType> {
        unsafe { from_glib_none(ffi::g_action_get_state_type(self.as_ref().to_glib_none().0)) }
    }

    fn connect_property_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_enabled_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GAction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_enabled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GAction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_parameter_type_notify<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn notify_parameter_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GAction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::parameter-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_parameter_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GAction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_property_state_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_state_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::GAction,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Action>,
        {
            let f: &F = &*(f as *const F);
            f(&Action::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::state-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_state_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Action")
    }
}
