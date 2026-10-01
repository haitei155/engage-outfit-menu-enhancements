//! Seven purchase categories, using native stock checks and sold-out entries.
use super::*;

#[skyline::hook(offset = 0x27b9aa0)]
unsafe fn fee_shop_ctor(menu: P, list: P, content: P, stock: P,
    select: P, decide: P, close: P, change: P, method: OptionalMethod) {
    let expanded = expand_header(content, 0xf0);
    call_original!(menu, list, content, stock, select, decide, close, change, method);
    if expanded {
        expand_cursor_records(menu, 0xd8);
        header_colors(menu);
    }
}

#[skyline::hook(offset = 0x27b9cb0)]
unsafe fn fee_shop_filter(stock: P, kind: i32, method: OptionalMethod) -> P {
    // Native owns stock/availability; filter only its freshly allocated result.
    // CreateMenuItem retains its native empty-list -> sold-out menu item branch.
    let list = call_original!(stock, kind, method);
    if list.is_null() { return list; }
    let items: P = read(list, 0x10);
    let count: i32 = read(list, 0x18);
    if items.is_null() || count < 0 { return list; }
    // Actual generic AccessoryData.Get MethodInfo used by the native Filter.
    let slot = *((unity::module_base() + 0x629ee98) as *const P);
    if slot.is_null() { return list; }
    let method: P = read(slot, 0);
    if method.is_null() { return list; }
    let get: extern "C" fn(P, P) -> P = std::mem::transmute(unity::module_base() + 0x217f20);
    let mut retained = 0usize;
    for i in 0..count as usize {
        let entry: P = read(items, 0x20 + i*8);
        if entry.is_null() { continue; }
        let aid: P = read(entry, 0x10);
        if aid.is_null() { continue; }
        let data = get(aid, method);
        if data.is_null() { continue; }
        let mask = native!(0x27b4da0, i32; P => data);
        if logic::category(mask) == Some(kind) {
            reference(items, 0x20 + retained*8, entry);
            retained += 1;
        }
    }
    for i in retained..count as usize {
        reference(items, 0x20 + i*8, std::ptr::null_mut());
    }
    write(list, 0x18, retained as i32);
    write(list, 0x1c, read::<i32>(list, 0x1c).wrapping_add(1));
    list
}

pub fn install() { skyline::install_hooks!(fee_shop_ctor, fee_shop_filter); }
