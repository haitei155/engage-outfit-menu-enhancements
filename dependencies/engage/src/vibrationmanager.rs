//! Convenience methods to perform vibrations.

use crate::pad::app_pad_vibration;
use unity::prelude::*;

#[skyline::from_offset(0x21c02c0)]
pub fn vibration_manager_one_shot(
    this: *const u8,
    time: f32,
    amplitude_magnitude: f32,
    amp_low: f32,
    amp_high: f32,
    freq_low: f32,
    freq_high: f32,
    method_info: OptionalMethod,
);

// App.VibrationManager$$PlayByVibrationFileName	71021c0a70	void App.VibrationManager$$PlayByVibrationFileName(App_VibrationManager_o * __this, System_String_o * vibFileName, float amplitudeMagnitude, float time, bool isLoop, MethodInfo * method)	508
#[skyline::from_offset(0x21c0a70)]
pub fn play_by_vibration_file_name(
    this: *const u8,
    vib_file_name: &Il2CppString,
    amplitude_magnitude: f32,
    time: f32,
    is_loop: bool,
    method_info: OptionalMethod,
);

/// Utility function to play a vibration.
pub fn vibrate(
    time: f32,
    amplitude_magnitude: f32,
    amp_low: f32,
    amp_high: f32,
    freq_low: f32,
    freq_high: f32,
) {
    unsafe {
        let vibration_manager = app_pad_vibration(None);
        vibration_manager_one_shot(
            vibration_manager,
            time,
            amplitude_magnitude,
            amp_low,
            amp_high,
            freq_low,
            freq_high,
            None,
        );
    }
}

// These were the original values as set by one of the overloads of vibration_manager_one_shot
// TODO: Explore the effects of changing them.
pub const FREQ_LOW: f32 = 160.0;
pub const FREQ_HIGH: f32 = 300.0;
