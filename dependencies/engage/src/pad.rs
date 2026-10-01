//! Types and methods to read the state of the inputs.


use modular_bitfield::{bitfield, specifiers::B4};
use unity::prelude::*;	

#[repr(C)]
#[unity::class("App", "Pad")]
pub struct Pad {
    base: [u8; 0x10],
    npad_id: i32,
    npad_style: i32,
    pub npad_state: NpadState,
    pub old_buttons: NpadButton,
}

#[repr(C)]
pub struct NpadState {
    sampling_number: i64,
    pub buttons: NpadButton,
    base: [u8; 0x20],
}

#[repr(C)]
#[bitfield]
pub struct NpadButton {
    pub a: bool,
    pub b: bool,
    pub x: bool,
    pub y: bool,
    pub stick_l: bool,
    pub stick_r: bool,
    pub l: bool,
    pub r: bool,
    pub zl: bool,
    pub zr: bool,
    pub plus: bool,
    pub minus: bool,
    pub left: bool,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub stick_l_left: bool,
    pub stick_l_up: bool,
    pub stick_l_right: bool,
    pub stick_l_down: bool,
    pub stick_r_left: bool,
    pub stick_r_up: bool,
    pub stick_r_right: bool,
    pub stick_r_down: bool,
    pub left_sl: bool,
    pub left_sr: bool,
    pub right_sl: bool,
    pub right_sr: bool,
    #[skip]
    __: B4,
}

#[unity::from_offset("App", "Pad", "Vibration")]	
pub fn app_pad_vibration(method_info: OptionalMethod) -> *const u8;