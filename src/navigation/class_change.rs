//! Class-change paging replaces the native ZR sorting gesture, Engage 2.0.0.
use super::*;

pub fn paging_allowed(menu: BasicMenu) -> bool {
    let class = menu.get_class();
    class.namespace() == "App" && class.name() == "ClassChangeJobMenu"
        && (class.raw().get_vtable()[58].method_ptr as usize)
            .wrapping_sub(unity::module_base()) == 0x1ea3890
}

#[skyline::hook(offset = 0x1ea3890)]
unsafe fn fee_class_change_custom(menu: BasicMenu, method: OptionalMethod) -> u32 {
    // TickInput performs the page move before entering CustomCall. Let its native
    // tail run OnDeselect/OnSelect and update class details, without sorting again.
    // L/R switching remains the game's original CustomCall on ordinary L/R input.
    let triggers = (1u64 << 8) | (1u64 << 9);
    if native!(0x1f23010, bool; u64 => triggers)
        || native!(0x1f23100, bool; u64 => triggers) {
        return native!(0x2461e10, u32; BasicMenu => menu);
    }
    call_original!(menu, method)
}

pub fn install() {
    skyline::install_hooks!(fee_class_change_custom);
}
