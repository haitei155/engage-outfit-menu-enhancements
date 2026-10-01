//! Wrappers to open and close a TitleBar when entering a menu.

use unity::prelude::*;

#[repr(C)]
#[unity::class("App", "TitleBar")]
pub struct TitleBar { }

impl TitleBar {
    fn get_instance() -> &'static TitleBar {
        TitleBar::class().get_static_fields::<&TitleBar>()
    }

    pub fn open_header(title: impl AsRef<str>, help: impl AsRef<str>, key_help_id: impl AsRef<str>) -> bool {
        unsafe {
            titlebar_open_header(
                Self::get_instance(),
                title.as_ref().into(),
                help.as_ref().into(),
                key_help_id.as_ref().into(),
                None,
            )
        }
    }

    pub fn close_header() -> bool {
        unsafe { titlebar_close_header(Self::get_instance(), None) }
    }
}

#[unity::from_offset("App", "TitleBar", "OpenHeader")]
fn titlebar_open_header(
    this: &'static TitleBar,
    title: &'static Il2CppString,
    help: &'static Il2CppString,
    key_help_id: &'static Il2CppString,
    method_info: OptionalMethod,
) -> bool;

#[unity::from_offset("App", "TitleBar", "CloseHeader")]
fn titlebar_close_header(this: &'static TitleBar, method_info: OptionalMethod) -> bool;
