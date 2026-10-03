mod expanded;
mod navigation;
mod outfit;
mod sortie_accessory;
mod native_accessory_scene;
mod sortie_backdrop;
mod sortie_transition;
mod obody_trace;
use std::sync::OnceLock;

// The standalone diagnostic NRO owns the post-load model hooks.
pub(crate) const DIAGNOSTICS_ENABLED: bool = false;
pub(crate) const MAP_HEAD_DIAGNOSTICS: bool = false;
pub(crate) const SHEZ_WEAPON_DIAGNOSTICS: bool = false;

pub(crate) fn debug_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::fs::read_to_string("sd:/engage/fee-outfit-menu-debug.flag")
            .map(|value| value.trim().eq_ignore_ascii_case("on"))
            .unwrap_or(false)
    })
}
#[skyline::main(name = "engage_outfit_enhancements")]
pub fn main() {
    if !expanded::supported() || !outfit::supported() || !navigation::supported() || !native_accessory_scene::supported() || !sortie_backdrop::supported() || !sortie_transition::supported() {
        if debug_enabled() { let _ = horizon_svc::output_debug_string("[FEE Outfit Menu v0.3] unsupported executable; no hooks installed\n"); }
        return;
    }
    expanded::install();
    outfit::install();
    navigation::install();
    sortie_accessory::install();
    native_accessory_scene::install();
    sortie_backdrop::install();
}
