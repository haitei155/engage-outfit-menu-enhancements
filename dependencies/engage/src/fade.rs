use unity::prelude::*;

use crate::proc::desc::ProcDesc;

/// Utility class to facilitate fadings
#[unity::class("App", "Fade")]
pub struct Fade { }

impl Fade {
    /// Blacken the layer over the provided duration
    pub fn black_in(duration: f32, layer: i32) -> &'static mut ProcDesc {
        unsafe { fade_black_in(duration, layer, None) }
    }

    /// Blacken the layer over the provided duration
    pub fn black_out(duration: f32, layer: i32) -> &'static mut ProcDesc {
        unsafe { fade_black_out(duration, layer, None) }
    }
}

#[unity::from_offset("App", "Fade", "BlackOut")]
fn fade_black_out(
    duration: f32,
    layer: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;

#[unity::from_offset("App", "Fade", "BlackIn")]
fn fade_black_in(
    duration: f32,
    layer: i32,
    method_info: OptionalMethod,
) -> &'static mut ProcDesc;