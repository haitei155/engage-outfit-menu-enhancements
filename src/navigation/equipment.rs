//! Both outfit screens share AccessoryEquipmentInfo. Keep native item formatting,
//! private-dress fallback, cursor binding and the prefab's vertical layout.
use super::*;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering};
// Identity only; never dereference or retain an unrooted managed object.
static CURRENT_INFO: AtomicUsize = AtomicUsize::new(0);
static CURRENT_KIND: AtomicI32 = AtomicI32::new(0);

unsafe fn component(transform: P, name: &str) -> P {
    let Ok(class) = unity::Class::try_lookup("App", name) else { return std::ptr::null_mut(); };
    let Some(ty) = unity::SystemType::from_il2cpp_type(class.raw().get_type()) else { return std::ptr::null_mut(); };
    native!(0x2c464e0, P; P => transform, P => ty.as_instance().as_ptr() as P)
}
unsafe fn rows(info: P) -> P {
    let go: P = read(info, 0x18);
    if go.is_null() { return go; }
    native!(0x2c4e880, P; P => go)
}
unsafe fn bind(content: P, item: P) {
    let class = unity::Class::from_raw(&*(read::<P>(content, 0) as *const unity::il2cpp::Il2CppClass));
    // Native Build uses class+0x1b8: virtual slot 8, not Object slot 0.
    // A cloned component may still reference the Face template's item. Detach
    // that copied reference so Build cannot clear another row's reverse link.
    reference(content, 0x18, std::ptr::null_mut());
    let vi = class.raw().get_vtable()[8];
    let f: extern "C" fn(P, P, OptionalMethod) = std::mem::transmute(vi.method_ptr);
    f(content, item, Some(&*(vi.method_info as *const unity::MethodInfo as *const ())));
}
unsafe fn refresh(info: P, bind_rows: bool) {
    let list: P = read(info, 0x28);
    if list.is_null() || read::<i32>(list, 0x18) != 7 { return; }
    let parent = rows(info);
    if parent.is_null() || native!(0x37925a0, i32; P => parent) < 7 { return; }
    let items: P = read(list, 0x10);
    for i in 0..7 {
        let item: P = read(items, 0x20 + i*8);
        let tr = native!(0x3792d40, P; P => parent, i32 => i as i32);
        let go = native!(0x2c46490, P; P => tr);
        let selected = if CURRENT_INFO.load(Ordering::Relaxed) == info as usize {
            CURRENT_KIND.load(Ordering::Relaxed)
        } else { -1 };
        let visible = logic::summary_visible(logic::KINDS[i], !read::<P>(item, 0x68).is_null(), selected);
        if bind_rows {
            let content = component(tr, "AccessoryMenuItemContent");
            if !content.is_null() { bind(content, item); }
        }
        native!(0x2c4ea10, (); P => go, bool => visible);
        // Head/back/battle/dye/style sprites are fresh readable PNG copies. The
        // native Face sprite uses RGB 127/241/234; retain its alpha in new icons.
        if bind_rows && !matches!(logic::KINDS[i], 0 | 2) {
            let content = component(tr, "AccessoryMenuItemContent");
            if !content.is_null() && read::<P>(content, 0x18) == item {
                let image: P = read(content, 0x68);
                if !image.is_null() {
                    let sprite = native!(0x3becfe0, P; P => image);
                    white_sprite(sprite, logic::KINDS[i]);
                    let class = unity::Class::from_raw(&*(read::<P>(image, 0) as *const unity::il2cpp::Il2CppClass));
                    let vi = class.raw().get_vtable()[23];
                    let set: extern "C" fn(P, Color, OptionalMethod) = std::mem::transmute(vi.method_ptr);
                    set(image, Color { r:127.0/255.0, g:241.0/255.0, b:234.0/255.0, a:1.0 },
                        Some(&*(vi.method_info as *const unity::MethodInfo as *const ())));
                }
            }
        }
    }
    native!(0x3c01600, (); P => parent);
}

#[skyline::hook(offset = 0x27b6530)]
unsafe fn fee_equipment_build(info: P, unit: P, method: OptionalMethod) {
    CURRENT_INFO.store(info as usize, Ordering::Relaxed);
    CURRENT_KIND.store(0, Ordering::Relaxed);
    call_original!(info, unit, method);
    let list: P = read(info, 0x28);
    if list.is_null() || read::<i32>(list, 0x18) != 2 { return; }
    let parent = rows(info);
    if parent.is_null() { return; }
    let count = native!(0x37925a0, i32; P => parent);
    if !(2..=7).contains(&count) { return; }
    let template = native!(0x3792d40, P; P => parent, i32 => 1);
    for _ in count..7 { native!(0x32ef280, P; P => template, P => parent, bool => false); }
    let old: P = read(list, 0x10);
    let first: P = read(old, 0x20);
    let face: P = read(old, 0x28);
    let result = array_like(old, 7);
    reference(list, 0x10, result);
    for (i, kind) in logic::KINDS.iter().copied().enumerate() {
        let item = if kind == 0 { first } else if kind == 2 { face } else {
            let item = object_like(first);
            native!(0x2455fc0, (); P => item);
            write(item, 0x7c, true); // AlwaysActive, as in native summary constructor.
            item
        };
        write(item, 0x70, kind);
        reference(result, 0x20+i*8, item);
    }
    write(list, 0x18, 7i32);
    write(list, 0x1c, read::<i32>(list, 0x1c).wrapping_add(1));
    native!(0x27b6ab0, (); P => info, P => unit);
}

#[skyline::hook(offset = 0x27b6ab0)]
unsafe fn fee_equipment_data(info: P, unit: P, method: OptionalMethod) {
    call_original!(info, unit, method);
    // Rebind all seven rows: native Build initially binds only Body and Face,
    // while clones otherwise keep prefab text and the Face item's reverse link.
    refresh(info, true);
    if CURRENT_INFO.load(Ordering::Relaxed) == info as usize {
        native!(0x27b7810, (); P => info, i32 => CURRENT_KIND.load(Ordering::Relaxed));
    }
}

#[skyline::hook(offset = 0x27b7810)]
unsafe fn fee_equipment_cursor(info: P, kind: i32, method: OptionalMethod) {
    let list: P = read(info, 0x28);
    if !list.is_null() && read::<i32>(list, 0x18) == 7 {
        if logic::KINDS.contains(&kind) {
            CURRENT_INFO.store(info as usize, Ordering::Relaxed);
            CURRENT_KIND.store(kind, Ordering::Relaxed);
            // Reflow before native cursor reads the selected item's RectTransform.
            refresh(info, false);
            let items: P = read(list, 0x10);
            let index = logic::KINDS.iter().position(|k| *k == kind).unwrap();
            let item: P = read(items, 0x20 + index * 8);
            if read::<P>(item, 0x68).is_null() {
                native!(0x27b7ae0, (); P => info);
                return;
            }
        } else {
            CURRENT_INFO.store(info as usize, Ordering::Relaxed);
            CURRENT_KIND.store(-1, Ordering::Relaxed);
            native!(0x27b7ae0, (); P => info);
            return;
        }
    }
    call_original!(info, kind, method);
}

// Native callers also use AccessoryData and parameterless overloads. Route all
// three entry points through the same equipped-row visibility check.
#[skyline::hook(offset = 0x27b7450)]
unsafe fn fee_equipment_cursor_data(info: P, data: P, method: OptionalMethod) {
    let list: P = read(info, 0x28);
    if !list.is_null() && read::<i32>(list, 0x18) == 7 {
        let kind = if data.is_null() { -1 } else { read::<i32>(data, 0x9c) };
        native!(0x27b7810, (); P => info, i32 => kind);
        return;
    }
    call_original!(info, data, method);
}
#[skyline::hook(offset = 0x27b73a0)]
unsafe fn fee_equipment_cursor_show(info: P, method: OptionalMethod) {
    let list: P = read(info, 0x28);
    if !list.is_null() && read::<i32>(list, 0x18) == 7 {
        let kind = if CURRENT_INFO.load(Ordering::Relaxed) == info as usize {
            CURRENT_KIND.load(Ordering::Relaxed)
        } else { -1 };
        native!(0x27b7810, (); P => info, i32 => kind);
        return;
    }
    call_original!(info, method);
}
pub fn install() { skyline::install_hooks!(fee_equipment_build, fee_equipment_data,
    fee_equipment_cursor, fee_equipment_cursor_data, fee_equipment_cursor_show); }
