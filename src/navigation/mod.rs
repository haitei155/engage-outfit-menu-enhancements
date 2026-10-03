//! UI companion to SierraSak's Expanded Accessory Slots. Engage 2.0.0.
//! Existing equipment slots, serialization, and GameIcon hooks remain authoritative.
mod logic;
use std::io::Write;
use engage::app::basicmenu::{BasicMenu, IBasicMenuMethods};
use unity::{Cast, OptionalMethod, SystemObject};
type P = *mut u8;
#[repr(C)]
#[derive(Clone, Copy)]
struct Vec3 { x: f32, y: f32, z: f32 }
#[repr(C)]
#[derive(Clone, Copy)]
struct Color { r: f32, g: f32, b: f32, a: f32 }
macro_rules! native {
    ($offset:expr, $ret:ty; $($ty:ty => $arg:expr),* $(,)?) => {{
        let f: extern "C" fn($($ty,)* OptionalMethod) -> $ret =
            std::mem::transmute(unity::module_base() + $offset);
        f($($arg,)* None)
    }};
}
unsafe fn read<T: Copy>(p: P, offset: usize) -> T { *(p.add(offset) as *const T) }
unsafe fn write<T>(p: P, offset: usize, value: T) { *(p.add(offset) as *mut T) = value; }
unsafe fn reference(p: P, offset: usize, value: P) {
    write(p, offset, value);
    native!(0x491fb0, (); P => p.add(offset), P => value);
}
fn log(s: &str) {
    if !crate::debug_enabled() { return; }
    let line = format!("[FEE Outfit Enhancements] {s}\n");
    let _ = horizon_svc::output_debug_string(&line);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true)
        .open("sd:/engage/fee-outfit-menu-debug.log") {
        let _ = f.write_all(line.as_bytes());
    }
}
unsafe fn array_like(old: P, count: usize) -> P {
    native!(0x4920a0, P; P => read::<P>(old, 0), usize => count)
}
unsafe fn object_like(old: P) -> P {
    native!(0x4921f0, P; P => read::<P>(old, 0))
}
// ExpandedAccessorySlots creates a fresh readable PNG texture for each of these five
// kinds. Whiten only this header copy, preserving alpha and all original list sprites.
unsafe fn white_sprite(sprite: P, kind: i32) {
    if sprite.is_null() { return; }
    let texture = native!(0x2f97670, P; P => sprite);
    let pixels = native!(0x378b3d0, P; P => texture, i32 => 0);
    if pixels.is_null() { return; }
    let count: usize = read(pixels, 0x18);
    for i in 0..count {
        let offset = 0x20 + i * 4;
        write(pixels, offset, logic::header_pixel(read::<u32>(pixels, offset), kind));
    }
    native!(0x378cf40, (); P => texture, P => pixels, i32 => 0);
    native!(0x378c830, (); P => texture, bool => true, bool => false);
}
unsafe fn header_colors(menu: P) {
    let content: P = read(menu, 0x70);
    if content.is_null() { return; }
    let class = unity::Class::from_raw(&*(read::<P>(menu, 0) as *const unity::il2cpp::Il2CppClass));
    let buy = class.name() == "AccessoryShopBuyMenu";
    let icons: P = read(content, if buy { 0xf0 } else { 0xe8 });
    if icons.is_null() || read::<usize>(icons, 0x18) != 7 { return; }
    let kind: i32 = read(menu, if buy { 0xd0 } else { 0xc4 });
    for i in 0..7 {
        let entry: P = read(icons, 0x20+i*8);
        let image: P = read(entry, 0x10);
        let class = unity::Class::from_raw(&*(read::<P>(image, 0) as *const unity::il2cpp::Il2CppClass));
        let vi = class.raw().get_vtable()[23]; // Graphic.set_color virtual override.
        let set: extern "C" fn(P, Color, OptionalMethod) = std::mem::transmute(vi.method_ptr);
        let level = if read::<i32>(entry, 0x18) == kind { 1.0 } else { 0.50 };
        set(image, Color { r: level, g: level, b: level, a: 1.0 },
            Some(&*(vi.method_info as *const unity::MethodInfo as *const ())));
    }
}
// Each Image retains the sprite through Unity's managed fields. No unrooted sprite cache.
unsafe fn expand_header(content: P, icons_offset: usize) -> bool {
    if content.is_null() { return false; }
    let old: P = read(content, icons_offset);
    if old.is_null() { return false; }
    let length: usize = read(old, 0x18);
    if length == 7 { return true; }
    if length != 2 { log("unexpected header layout; keeping native menu"); return false; }
    let first: P = read(old, 0x20);
    let second: P = read(old, 0x28);
    if first.is_null() || second.is_null() { return false; }
    let template: P = read(second, 0x10);
    let transform = native!(0x2c46440, P; P => template);
    let parent = native!(0x37909f0, P; P => transform);
    // Verified prefab children: ArrowL, Category1, Category2, ArrowR. The layout
    // group controls positions; cloned images must go before ArrowR in sibling order.
    let children = native!(0x37925a0, i32; P => parent);
    if children != if icons_offset == 0xf0 { 5 } else { 4 } { return false; }
    let right = native!(0x3792d40, P; P => parent, i32 => children - 1);
    if icons_offset == 0xf0 {
        // Purchase prefab has an additional All placeholder before Clothes.
        let all = native!(0x3792d40, P; P => parent, i32 => 1);
        let go = native!(0x2c46490, P; P => all);
        native!(0x2c4ea10, (); P => go, bool => false);
    }
    let clothes = native!(0x3becfe0, P; P => read::<P>(first, 0x10));
    let face = native!(0x3becfe0, P; P => template);
    let result = array_like(old, logic::KINDS.len());
    if result.is_null() { return false; }
    for (i, kind) in logic::KINDS.iter().copied().enumerate() {
        let entry = if i == 0 { first } else if i == 1 { second } else {
            let entry = object_like(second);
            native!(0x2904b40, (); P => entry);
            let image = native!(0x32ef280, P; P => template, P => parent, bool => false);
            reference(entry, 0x10, image);
            entry
        };
        write(entry, 0x18, kind);
        let image: P = read(entry, 0x10);
        let sprite = if kind == 0 { clothes } else if kind == 2 { face } else {
            let sprite = native!(0x227d5b0, P; i32 => kind);
            white_sprite(sprite, kind);
            sprite
        };
        native!(0x3becff0, (); P => image, P => sprite);
        let tr = native!(0x2c46440, P; P => image);
        let scale = if kind == 1 { 0.50 } else { 0.60 };
        native!(0x3790940, (); P => tr, Vec3 => Vec3 { x: scale, y: scale, z: 1.0 });
        native!(0x37926e0, (); P => tr, i32 => i as i32 + 1);
        reference(result, 0x20 + i * 8, entry);
    }
    reference(content, icons_offset, result);
    native!(0x3792690, (); P => right);
    if let Ok(class) = unity::Class::try_lookup("UnityEngine.UI", "HorizontalLayoutGroup") {
        if let Some(ty) = unity::SystemType::from_il2cpp_type(class.raw().get_type()) {
            let layout = native!(0x2c464e0, P; P => parent, P => ty.as_instance().as_ptr() as P);
            if !layout.is_null() { native!(0x3193600, (); P => layout, f32 => 10.0); }
        }
    }
    native!(0x3c01600, (); P => parent);
    log("seven white silhouettes; hat 0.50; spacing 10; ArrowR last");
    true
}
// Native constructor sizes cursor memory by visible icon count (7), but style's Kind is 7.
// Allocate eight cursor records so all actual Kind indexes have an independent record.
#[skyline::hook(offset = 0x27bfc30)]
unsafe fn fee_accessory_ctor(menu: P, list: P, content: P, unit: P,
    select: P, decide: P, close: P, change: P, method: OptionalMethod) {
    let expanded = expand_header(content, 0xe8);
    call_original!(menu, list, content, unit, select, decide, close, change, method);
    if !expanded { return; }
    header_colors(menu);
    let old: P = read(menu, 0xd0);
    if old.is_null() { return; }
    let length: usize = read(old, 0x18);
    if length >= 8 { return; }
    expand_cursor_records(menu, 0xd0);
    log("menu cursor records ready for kinds 0,1,2,3,5,6,7");
}
unsafe fn expand_cursor_records(menu: P, offset: usize) {
    let old: P = read(menu, offset);
    if old.is_null() { return; }
    let length: usize = read(old, 0x18);
    if length == 0 || length >= 8 { return; }
    let result = array_like(old, 8);
    let template: P = read(old, 0x20);
    for i in 0..8 {
        let item = if i < length { read::<P>(old, 0x20 + i*8) } else {
            let item = object_like(template);
            native!(0x2467960, (); P => item);
            item
        };
        reference(result, 0x20+i*8, item);
    }
    reference(menu, offset, result);
}
// Start with the native eligibility/ownership/person rules and stably compact its result.
// Passing nonzero Kind to native Filter still means its original combined accessories page.
#[skyline::hook(offset = 0x27bfe40)]
unsafe fn fee_accessory_filter(kind: i32, unit: P, method: OptionalMethod) -> P {
    let list = call_original!(kind, unit, method);
    if list.is_null() || kind == 0 { return list; }
    let items: P = read(list, 0x10);
    let count: i32 = read(list, 0x18);
    if items.is_null() || count < 0 { return list; }
    let mut retained = 0usize;
    for i in 0..count as usize {
        let item: P = read(items, 0x20+i*8);
        if item.is_null() { continue; }
        let mask = native!(0x27b4da0, i32; P => item);
        if logic::category(mask) == Some(kind) {
            reference(items, 0x20+retained*8, item);
            retained += 1;
        }
    }
    for i in retained..count as usize { reference(items, 0x20+i*8, std::ptr::null_mut()); }
    write(list, 0x18, retained as i32);
    write(list, 0x1c, read::<i32>(list, 0x1c).wrapping_add(1));
    list
}
// TickInput is entered only for the active menu. The native tail performs OnDeselect,
// OnSelect, help/preview events, cursor sound and result handling after our selection move.
#[skyline::hook(offset = 0x245ed80)]
unsafe fn fee_accessory_input(menu: BasicMenu, method: OptionalMethod) -> bool {
    if crate::sortie_accessory::is_open() && crate::sortie_transition::input_blocked() {
        crate::sortie_accessory::tick_preview_input(menu);
        return false;
    }
    let (eligible, accessory) = if menu.is_null() { (false, false) } else {
        let class = menu.get_class();
        let name = class.name();
        let ns = class.namespace();
        let accessory = ns == "App" && name == "AccessoryShopChangeMenu";
        let extra = logic::character_menu(&ns, &name);
        // Audited native overrides retain their existing actions; unknown replacements skip.
        let custom_unused = extra && menu_scope::custom_allowed(&ns, &name,
            (class.raw().get_vtable()[58].method_ptr as usize).wrapping_sub(unity::module_base()));
        (accessory || (extra && custom_unused && menu_scope::context_allowed(menu))
            || menu_scope::inventory_trade_allowed(menu), accessory)
    };
    if eligible
        && !menu.is_input_disable() && !menu.is_input_disable_now_frame()
        && !menu.is_suspend() && !menu.is_opening() && !menu.is_closing() {
        let up = native!(0x1f23010, bool; u64 => 1 << 8)
            || native!(0x1f23100, bool; u64 => 1 << 8);
        let down = native!(0x1f23010, bool; u64 => 1 << 9)
            || native!(0x1f23100, bool; u64 => 1 << 9);
        if up != down {
            let p = menu.as_instance().as_ptr() as P;
            let list: P = read(p, 0x78);
            if !list.is_null() {
                let count: i32 = read(list, 0x18);
                let index: i32 = read(p, 0xa0);
                let scroll: i32 = read(p, 0xa8);
                let visible: i32 = read(p, 0x9c);
                let class = menu.get_class();
                let grid = class.namespace() == "App" && class.name() == "UnitSelectSortieMenu";
                let (next, start) = if grid {
                    let content: P = read(p, 0xc8);
                    let layout: P = if content.is_null() { std::ptr::null_mut() } else { read(content, 0xe8) };
                    let columns = if layout.is_null() { 0 } else { read::<i32>(layout, 0x74) };
                    logic::grid_page(index, scroll, count, visible, columns, down)
                } else { logic::page(index, scroll, count, visible, down) };
                if next != index {
                    menu.set_select_index(next);
                    menu.set_scroll_index(start);
                    menu.scroll_instant();
                    log(&format!("page {}.{}: {} -> {}; rows={visible}; scroll={start}", class.namespace(), class.name(), index, next));
                }
            }
        }
    }
    let result = call_original!(menu, method);
    crate::sortie_accessory::tick_preview_input(menu);
    if accessory || (!menu.is_null() && menu.get_class().namespace() == "App"
        && menu.get_class().name() == "AccessoryShopBuyMenu") {
        header_colors(menu.as_instance().as_ptr() as P);
    }
    result
}
pub fn install() {
    log("install begin; Engage 2.0.0; Expanded Accessory Slots required");
    // Generated from the locally extracted 2.0.0 code image; validate before any mutation.
    if !guards::valid() { log("native instruction mismatch; plugin disabled"); return; }
    skyline::install_hooks!(fee_accessory_ctor, fee_accessory_filter, fee_accessory_input);
    equipment::install();
    shop::install();
    log("installed navigation and seven-row equipment summary hooks");
}
mod guards;
mod menu_scope;

pub fn supported() -> bool { guards::valid() }

mod equipment;

mod shop;
