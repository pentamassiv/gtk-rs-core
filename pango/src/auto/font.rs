// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::object::IsA;
use glib::translate::*;
use pango_sys;
use std::fmt;
use Coverage;
use EngineShape;
use FontDescription;
#[cfg(any(feature = "v1_46", feature = "dox"))]
use FontFace;
use FontMap;
use FontMetrics;
use Glyph;
use Language;
use Rectangle;

glib_wrapper! {
    pub struct Font(Object<pango_sys::PangoFont, pango_sys::PangoFontClass, FontClass>);

    match fn {
        get_type => || pango_sys::pango_font_get_type(),
    }
}

pub const NONE_FONT: Option<&Font> = None;

pub trait FontExt: 'static {
    fn describe(&self) -> Option<FontDescription>;

    fn describe_with_absolute_size(&self) -> Option<FontDescription>;

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape>;

    fn get_coverage(&self, language: &Language) -> Option<Coverage>;

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    fn get_face(&self) -> Option<FontFace>;

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //fn get_features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32;

    fn get_font_map(&self) -> Option<FontMap>;

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle);

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //fn get_hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t>;

    fn get_metrics(&self, language: Option<&Language>) -> Option<FontMetrics>;

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    fn has_char(&self, wc: char) -> bool;
}

impl<O: IsA<Font>> FontExt for O {
    fn describe(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(pango_sys::pango_font_describe(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn describe_with_absolute_size(&self) -> Option<FontDescription> {
        unsafe {
            from_glib_full(pango_sys::pango_font_describe_with_absolute_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn find_shaper(&self, language: &Language, ch: u32) -> Option<EngineShape> {
        unsafe {
            from_glib_none(pango_sys::pango_font_find_shaper(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
                ch,
            ))
        }
    }

    fn get_coverage(&self, language: &Language) -> Option<Coverage> {
        unsafe {
            from_glib_full(pango_sys::pango_font_get_coverage(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_46", feature = "dox"))]
    fn get_face(&self) -> Option<FontFace> {
        unsafe {
            from_glib_none(pango_sys::pango_font_get_face(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //fn get_features(&self, features: /*Unimplemented*/&mut Fundamental: Pointer, num_features: &mut u32) -> u32 {
    //    unsafe { TODO: call pango_sys:pango_font_get_features() }
    //}

    fn get_font_map(&self) -> Option<FontMap> {
        unsafe {
            from_glib_none(pango_sys::pango_font_get_font_map(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn get_glyph_extents(&self, glyph: Glyph) -> (Rectangle, Rectangle) {
        unsafe {
            let mut ink_rect = Rectangle::uninitialized();
            let mut logical_rect = Rectangle::uninitialized();
            pango_sys::pango_font_get_glyph_extents(
                self.as_ref().to_glib_none().0,
                glyph,
                ink_rect.to_glib_none_mut().0,
                logical_rect.to_glib_none_mut().0,
            );
            (ink_rect, logical_rect)
        }
    }

    //#[cfg(any(feature = "v1_44", feature = "dox"))]
    //fn get_hb_font(&self) -> /*Ignored*/Option<harf_buzz::font_t> {
    //    unsafe { TODO: call pango_sys:pango_font_get_hb_font() }
    //}

    fn get_metrics(&self, language: Option<&Language>) -> Option<FontMetrics> {
        unsafe {
            from_glib_full(pango_sys::pango_font_get_metrics(
                self.as_ref().to_glib_none().0,
                mut_override(language.to_glib_none().0),
            ))
        }
    }

    #[cfg(any(feature = "v1_44", feature = "dox"))]
    fn has_char(&self, wc: char) -> bool {
        unsafe {
            from_glib(pango_sys::pango_font_has_char(
                self.as_ref().to_glib_none().0,
                wc.to_glib(),
            ))
        }
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Font")
    }
}