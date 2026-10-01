//! Utilities to display popup dialogues. Tied to the [`ProcInst`](crate::proc::ProcInst) system.

use unity::prelude::*;

use crate::proc::Bindable;

pub struct GameMessage;

impl GameMessage {
    pub fn create_key_wait(parent: &impl Bindable, message: impl AsRef<str>) {
        unsafe {
            gamemessage_create_key_wait(parent, message.as_ref().into(), None);
        }
    }
}

#[skyline::from_offset(0x2270d00)]
fn gamemessage_create_key_wait<P: Bindable>(proc: &P, mess: &'static Il2CppString, method_info: OptionalMethod) -> *const u8;
