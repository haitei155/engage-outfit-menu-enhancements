// Native CustomCall entry points from the game 2.0.0 metadata/code image.
pub fn custom_allowed(namespace: &str, name: &str, offset: usize) -> bool {
    if offset == 0x2461e10 { return true; }
    match (namespace, name) {
        ("App", "RingSelectMenu") => offset == 0x1eba880,
        ("App", "WeaponShopBuyMenu") => offset == 0x21cd090,
        ("App", "MyRoomRelianceSubSelect") => offset == 0x23986b0,
        _ => false,
    }
}
use super::*;

// Audited IL2CPP singleton generic methods require their actual MethodInfo.
unsafe fn singleton(offset: usize, method_global: usize) -> P {
    let slot = *( (unity::module_base() + method_global) as *const P );
    if slot.is_null() { return std::ptr::null_mut(); }
    let method = read::<P>(slot, 0);
    if method.is_null() { return std::ptr::null_mut(); }
    let f: extern "C" fn(P) -> P = std::mem::transmute(unity::module_base()+offset);
    f(method)
}

pub unsafe fn context_allowed(menu: BasicMenu) -> bool {
    let class = menu.get_class();
    if class.name() != "SkillEditPoolSkillMenu" { return true; }
    // In detailed help mode ZR opens the native skill help. Normal selection's
    // ZL/ZR are unused, and both pool skill and the "none" entry use this gate.
    let help = singleton(0x1c6cd0, 0x6286a40);
    !help.is_null() && read::<i32>(help, 0x1c) != 3
}

pub unsafe fn inventory_paging_allowed(menu: BasicMenu) -> bool {
    let class = menu.get_class();
    if class.namespace() != "App" { return false; }
    let name = class.name();
    let expected = match name.as_str() {
        "InventoryUnitItemMenu" => 0x279e1a0,
        "InventoryPoolItemMenu" => 0x2799130,
        _ => return false,
    };
    if (class.raw().get_vtable()[58].method_ptr as usize).wrapping_sub(unity::module_base()) != expected { return false; }
    let manager = singleton(0x2194e0, 0x62a0078);
    if manager.is_null() { return false; }
    let selection: P = read(manager, 0x20);
    if selection.is_null() { return false; }
    // Both native pane switches now read StickR. Only the audited item callbacks
    // may page; shop, well and other inventory subclasses remain excluded.
    let p = menu.as_instance().as_ptr() as P;
    let item = native!(0x2454ae0, P; P => p, i32 => read::<i32>(p,0xa0));
    if item.is_null() { return false; }
    let item_class = unity::Class::from_raw(&*(read::<P>(item,0) as *const unity::il2cpp::Il2CppClass));
    // BasicMenuItem.CustomCall is slot 26; slot 18 is ACall.
    let callback = (item_class.raw().get_vtable()[26].method_ptr as usize).wrapping_sub(unity::module_base());
    matches!(callback, 0x279fa50 | 0x279ae50 | 0x2466940)
}

// Nested classes all report name "Menu". Match their actual class identity,
// never enable unrelated windows merely because they share that name.
unsafe fn exact(menu: BasicMenu, name: &str, custom: usize) -> bool {
    let class = menu.get_class();
    let Ok(expected) = unity::Class::try_lookup("App", name) else { return false; };
    std::ptr::eq(class.raw(), expected.raw())
        && (class.raw().get_vtable()[58].method_ptr as usize)
            .wrapping_sub(unity::module_base()) == custom
}

pub unsafe fn ring_ability_allowed(menu: BasicMenu) -> bool {
    exact(menu, "RingListSkillMenu.Menu", 0x1b19990)
}

pub unsafe fn ring_catalog_allowed(menu: BasicMenu) -> bool {
    exact(menu, "RingListSequence.GodAndRingListWindow.Menu", 0x2461e10)
}

pub unsafe fn sell_allowed(menu: BasicMenu) -> bool {
    if !exact(menu, "ShopSellMenu", 0x2461e10) { return false; }
    let p = menu.as_instance().as_ptr() as P;
    let item = native!(0x2454ae0, P; P => p, i32 => read::<i32>(p, 0xa0));
    if item.is_null() { return false; }
    let class = unity::Class::from_raw(&*(read::<P>(item, 0) as *const unity::il2cpp::Il2CppClass));
    let callback = (class.raw().get_vtable()[26].method_ptr as usize)
        .wrapping_sub(unity::module_base());
    matches!(callback, 0x21b6eb0 | 0x21b2480)
}

pub unsafe fn inventory_switch_allowed() -> bool {
    let manager = singleton(0x2194e0, 0x62a0078);
    if manager.is_null() { return false; }
    let selection: P = read(manager, 0x20);
    !selection.is_null() && !native!(0x1d75380, bool; P => selection)
}
