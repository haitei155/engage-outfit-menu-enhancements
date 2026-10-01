//! Utility to force the game to reload the language MSBTs.

use unity::prelude::*;

/// Holder for the language and voice chosen by the player.
#[unity::class("App", "Language")]
pub struct Language { }

impl Language {
    /// Force the game to unload and reload every language MSBT file.
    /// 
    /// Avoid calling this while the player is already in the game, as it will most likely call a crash.
    pub fn reflect_setting() {
        unsafe { language_reflectsetting(None) };
    }
}

#[unity::from_offset("App", "Language", "ReflectSetting")]
fn language_reflectsetting(method_info: OptionalMethod);