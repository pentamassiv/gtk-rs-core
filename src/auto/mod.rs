// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ cf55bdc)
// DO NOT EDIT

mod checksum;
pub use self::checksum::Checksum;

mod date_time;
pub use self::date_time::DateTime;

mod key_file;
pub use self::key_file::KeyFile;

mod main_context;
pub use self::main_context::MainContext;

mod main_loop;
pub use self::main_loop::MainLoop;

mod source;
pub use self::source::Source;

mod time_zone;
pub use self::time_zone::TimeZone;

mod enums;
pub use self::enums::ChecksumType;
pub use self::enums::DateMonth;
pub use self::enums::DateWeekday;
pub use self::enums::KeyFileError;
pub use self::enums::SeekType;
pub use self::enums::TimeType;

mod flags;
pub use self::flags::FileTest;
pub use self::flags::FormatSizeFlags;
pub use self::flags::IOCondition;
pub use self::flags::KeyFileFlags;

mod alias;
pub use self::alias::DateDay;
pub use self::alias::DateYear;
pub use self::alias::Time;
pub use self::alias::TimeSpan;

pub mod functions;

mod constants;
pub use self::constants::CSET_A_2_Z;
pub use self::constants::CSET_DIGITS;
pub use self::constants::CSET_a_2_z;
pub use self::constants::KEY_FILE_DESKTOP_ACTION_GROUP_PREFIX;
pub use self::constants::KEY_FILE_DESKTOP_GROUP;
#[cfg(any(feature = "v2_38", feature = "dox"))]
pub use self::constants::KEY_FILE_DESKTOP_KEY_ACTIONS;
pub use self::constants::KEY_FILE_DESKTOP_KEY_CATEGORIES;
pub use self::constants::KEY_FILE_DESKTOP_KEY_COMMENT;
#[cfg(any(feature = "v2_38", feature = "dox"))]
pub use self::constants::KEY_FILE_DESKTOP_KEY_DBUS_ACTIVATABLE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_EXEC;
pub use self::constants::KEY_FILE_DESKTOP_KEY_FULLNAME;
pub use self::constants::KEY_FILE_DESKTOP_KEY_GENERIC_NAME;
pub use self::constants::KEY_FILE_DESKTOP_KEY_GETTEXT_DOMAIN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_HIDDEN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_ICON;
pub use self::constants::KEY_FILE_DESKTOP_KEY_KEYWORDS;
pub use self::constants::KEY_FILE_DESKTOP_KEY_MIME_TYPE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NAME;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NOT_SHOW_IN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_NO_DISPLAY;
pub use self::constants::KEY_FILE_DESKTOP_KEY_ONLY_SHOW_IN;
pub use self::constants::KEY_FILE_DESKTOP_KEY_PATH;
pub use self::constants::KEY_FILE_DESKTOP_KEY_STARTUP_NOTIFY;
pub use self::constants::KEY_FILE_DESKTOP_KEY_STARTUP_WM_CLASS;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TERMINAL;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TRY_EXEC;
pub use self::constants::KEY_FILE_DESKTOP_KEY_TYPE;
pub use self::constants::KEY_FILE_DESKTOP_KEY_URL;
pub use self::constants::KEY_FILE_DESKTOP_KEY_VERSION;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_APPLICATION;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_DIRECTORY;
pub use self::constants::KEY_FILE_DESKTOP_TYPE_LINK;
pub use self::constants::OPTION_REMAINING;
pub use self::constants::STR_DELIMITERS;
pub use self::constants::URI_RESERVED_CHARS_GENERIC_DELIMITERS;
pub use self::constants::URI_RESERVED_CHARS_SUBCOMPONENT_DELIMITERS;

#[doc(hidden)]
pub mod traits {
}
