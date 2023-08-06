// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::{Initable};
use glib::{prelude::*,signal::{connect_raw, SignalHandlerId},translate::*};
use std::{boxed::Box as Box_,fmt,mem::transmute};

glib::wrapper! {
    #[doc(alias = "GPowerProfileMonitor")]
    pub struct PowerProfileMonitor(Interface<ffi::GPowerProfileMonitor, ffi::GPowerProfileMonitorInterface>) @requires Initable;

    match fn {
        type_ => || ffi::g_power_profile_monitor_get_type(),
    }
}

impl PowerProfileMonitor {
        pub const NONE: Option<&'static PowerProfileMonitor> = None;
    

    #[doc(alias = "g_power_profile_monitor_dup_default")]
    #[doc(alias = "dup_default")]
    pub fn get_default() -> PowerProfileMonitor {
        unsafe {
            from_glib_full(ffi::g_power_profile_monitor_dup_default())
        }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::PowerProfileMonitor>> Sealed for T {}
}

pub trait PowerProfileMonitorExt: IsA<PowerProfileMonitor> + sealed::Sealed + 'static {
    #[doc(alias = "g_power_profile_monitor_get_power_saver_enabled")]
    #[doc(alias = "get_power_saver_enabled")]
    fn is_power_saver_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::g_power_profile_monitor_get_power_saver_enabled(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "power-saver-enabled")]
    fn connect_power_saver_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_power_saver_enabled_trampoline<P: IsA<PowerProfileMonitor>, F: Fn(&P) + 'static>(this: *mut ffi::GPowerProfileMonitor, _param_spec: glib::ffi::gpointer, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(PowerProfileMonitor::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::power-saver-enabled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(notify_power_saver_enabled_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl<O: IsA<PowerProfileMonitor>> PowerProfileMonitorExt for O {}

impl fmt::Display for PowerProfileMonitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PowerProfileMonitor")
    }
}
