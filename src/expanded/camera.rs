//! Camera changes only. The upstream eight slots and save format are retained.
use unity_legacy::prelude::OptionalMethod;
type P = *mut u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Vec3 { x: f32, y: f32, z: f32 }
fn midpoint(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { x: (a.x+b.x)*0.5, y: (a.y+b.y)*0.5, z: (a.z+b.z)*0.5 }
}
// Target is Nullable<AccessoryData.Kinds>: kind at +0xe0, hasValue at +0xe4.
#[skyline::hook(offset = 0x2d71d00)]
unsafe fn fee_accessory_camera_target(camera: P, target: u64, method: OptionalMethod) {
    let target = if target & 0xff00000000 != 0 && target as u32 == 7 {
        target & 0xffffffff00000000 // Style uses exactly the Body target.
    } else { target };
    call_original!(camera, target, method);
}
// Evaluate the two native positions for the current character, including zoom.
// Only the target kind is temporarily replaced; no setter, transition or GC change.
#[skyline::hook(offset = 0x2d73d30)]
unsafe fn fee_accessory_camera_nearest(camera: P, method: OptionalMethod) -> Vec3 {
    let head = call_original!(camera, method);
    let target = camera.add(0xe0) as *mut u64;
    let old = *target;
    if old & 0xff00000000 == 0 || old as u32 != 1 || *camera.add(0xec) == 0 { return head; }
    *target = (old & 0xffffffff00000000) | 2;
    let face = call_original!(camera, method);
    *target = old;
    midpoint(head, face)
}
#[skyline::hook(offset = 0x2d73f70)]
unsafe fn fee_accessory_camera_farthest(camera: P, method: OptionalMethod) -> Vec3 {
    let head = call_original!(camera, method);
    let target = camera.add(0xe0) as *mut u64;
    let old = *target;
    if old & 0xff00000000 == 0 || old as u32 != 1 || *camera.add(0xec) == 0 { return head; }
    *target = (old & 0xffffffff00000000) | 2;
    let face = call_original!(camera, method);
    *target = old;
    midpoint(head, face)
}
pub fn valid() -> bool {
    let base = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize };
    super::guards::WORDS.iter().all(|(offset, word)| unsafe { *((base+offset) as *const u32) == *word })
}
pub fn install() {
    skyline::install_hooks!(fee_accessory_camera_target, fee_accessory_camera_nearest, fee_accessory_camera_farthest);
}
