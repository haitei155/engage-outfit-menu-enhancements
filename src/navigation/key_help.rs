//! Add the native paging glyph to exactly the menus accepted by TickInput.
use super::*;
use engage::app::language::Language;
use unity::Il2CppString;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static OWNED: AtomicBool = AtomicBool::new(false);
static GENERATION: AtomicUsize = AtomicUsize::new(0);
static LAST_GENERATION: AtomicUsize = AtomicUsize::new(usize::MAX);
static LAST_MENU: AtomicUsize = AtomicUsize::new(0);
static LAST_CONTROLLER: AtomicUsize = AtomicUsize::new(0);
static LAST_STYLE: AtomicUsize = AtomicUsize::new(0);

fn caption() -> Il2CppString {
    match Language::get_lang().value {
        10 => "翻页", 9 => "翻頁", 0 => "ページ切替", _ => "Page",
    }.into()
}

unsafe fn object(controller: P, index: usize) -> P {
    let list: P = read(controller, 0x18);
    if list.is_null() || read::<i32>(list, 0x18) <= index as i32 { return std::ptr::null_mut(); }
    let items: P = read(list, 0x10);
    if items.is_null() { return std::ptr::null_mut(); }
    read(items, 0x20 + index * 8)
}

unsafe fn show(controller: P, obj: P, text: Il2CppString) {
    if obj.is_null() { return; }
    native!(0x1bda920, (); P => controller, P => obj, Il2CppString => text);
    native!(0x2c4ea10, (); P => obj, bool => true);
    let tr = native!(0x2c4e880, P; P => obj);
    native!(0x3792690, (); P => tr);
}

#[skyline::hook(offset = 0x1bda9f0)]
unsafe fn fee_key_help_clear(controller: P, method: OptionalMethod) {
    call_original!(controller, method);
    OWNED.store(false, Ordering::Relaxed);
    GENERATION.fetch_add(1, Ordering::Relaxed);
}

#[skyline::hook(offset = 0x1bda5c0)]
unsafe fn fee_key_help(controller: P, id: Il2CppString, method: OptionalMethod) {
    call_original!(controller, id, method);
    OWNED.store(false, Ordering::Relaxed);
    GENERATION.fetch_add(1, Ordering::Relaxed);
    if controller.is_null() || id.is_null() { return; }
    match id.to_rust_string().as_str() {
        "KHID_クラスチェンジ" => {
            let sort = object(controller, 8);
            if !sort.is_null() { native!(0x2c4ea10, (); P => sort, bool => false); }
        }
        "KHID_紋章士_紋章士指輪" => {
            let old = object(controller, 8);
            if !old.is_null() { native!(0x2c4ea10, (); P => old, bool => false); }
            // Native Type.StickR = 19, NpadButton.StickR = bit 5.
            let text = native!(0x25c9240, Il2CppString;
                Il2CppString => "MID_KEYHELP_MENU_ABILITY".into());
            show(controller, object(controller, 19), text);
        }
        _ => {}
    }
}

pub unsafe fn update(menu: BasicMenu, eligible: bool) {
    let active = eligible && !menu.is_null() && !menu.is_suspend()
        && !menu.is_closing() && !menu.is_opening()
        && !menu.is_input_disable() && !menu.is_input_disable_now_frame();
    let style = if active && menu_scope::ring_ability_allowed(menu)
        && read::<bool>(menu.as_instance().as_ptr() as P, 0xd0) { 1 }
        else if active && menu_scope::inventory_paging_allowed(menu) {
            if menu_scope::inventory_switch_allowed() { 2 } else { 4 }
        }
        else if active && menu_scope::sell_allowed(menu) { 3 }
        else { 0 };
    update_hints(menu.as_instance().as_ptr() as usize, active, style);
}

unsafe fn update_hints(identity: usize, active: bool, style: u8) {
    if !active && !OWNED.load(Ordering::Relaxed) { return; }
    // Resolve the currently owned title/controller every time; never cache a Unity pointer.
    let title = native!(0x21ecaf0, P;);
    if title.is_null() { return; }
    let current: P = read(title, 0x50);
    if current.is_null() { return; }
    let controller: P = read(current, 0x48);
    if controller.is_null() { return; }
    if !active {
        if OWNED.swap(false, Ordering::Relaxed) {
            let id: Il2CppString = read(title, 0x70);
            if !id.is_null() { fee_key_help(controller, id, None); }
            else { native!(0x1bda9f0, (); P => controller); }
        }
        LAST_MENU.store(0, Ordering::Relaxed);
        return;
    }
    let generation = GENERATION.load(Ordering::Relaxed);
    if OWNED.load(Ordering::Relaxed)
        && LAST_MENU.load(Ordering::Relaxed) == identity
        && LAST_CONTROLLER.load(Ordering::Relaxed) == controller as usize
        && LAST_STYLE.load(Ordering::Relaxed) == style as usize
        && LAST_GENERATION.load(Ordering::Relaxed) == generation { return; }
    let pages = object(controller, 9);
    if pages.is_null() { return; }
    // A modal replaces the parent's captions on the same native controller.
    // On return, appending Page alone leaves its StickR Close caption behind.
    // Rebuild the current page's complete native hint set before its overlay,
    // including after controller changes or external SetMessage/Clear calls.
    if style != 1 {
        let id: Il2CppString = read(title, 0x70);
        if !id.is_null() { fee_key_help(controller, id, None); }
        else { native!(0x1bda9f0, (); P => controller); }
    }
    match style {
        1 => {
            // Native HideHeaderKeyHelp clears child glyphs in the alternate title,
            // rather than disabling its container. Populate that current controller
            // with modal controls only; ShowHeaderKeyHelp would swap titles again.
            native!(0x1bda9f0, (); P => controller);
            let text: Il2CppString = match Language::get_lang().value {
                10 => "关闭", 9 => "關閉", 0 => "閉じる", _ => "Close",
            }.into();
            show(controller, object(controller, 19), text);
        }
        2 | 4 => {
            let stick = object(controller, 19);
            if style == 2 {
                let text = native!(0x25c9240, Il2CppString;
                    Il2CppString => "MID_KEYHELP_MENU_ACTION".into());
                show(controller, stick, text);
            } else if !stick.is_null() {
                native!(0x2c4ea10, (); P => stick, bool => false);
            }
        }
        3 => {
            let old = object(controller, 8);
            if !old.is_null() { native!(0x2c4ea10, (); P => old, bool => false); }
            let text = native!(0x25c9240, Il2CppString;
                Il2CppString => "MID_KEYHELP_MENU_SELECT".into());
            show(controller, object(controller, 19), text);
        }
        _ => {}
    }
    // The page glyph is always placed after all other active controls.
    show(controller, pages, caption());
    OWNED.store(true, Ordering::Relaxed);
    LAST_MENU.store(identity, Ordering::Relaxed);
    LAST_CONTROLLER.store(controller as usize, Ordering::Relaxed);
    LAST_STYLE.store(style as usize, Ordering::Relaxed);
    LAST_GENERATION.store(GENERATION.load(Ordering::Relaxed), Ordering::Relaxed);
}

#[skyline::hook(offset = 0x1eba880)]
unsafe fn fee_ring_custom(menu: BasicMenu, method: OptionalMethod) -> u32 {
    // TickInput has already moved by the visible row count. Consume both page
    // inputs here so native ring actions cannot open another window or rebuild
    // the selection. The native TickInput tail still runs OnDeselect/OnSelect.
    let pages = (1u64 << 8) | (1u64 << 9);
    if native!(0x1f23010, bool; u64 => pages)
        || native!(0x1f23100, bool; u64 => pages) {
        return native!(0x2461e10, u32; BasicMenu => menu);
    }
    call_original!(menu, method)
}

// Bond-ring catalog pages have no skill menu to receive BasicMenu.TickInput.
// Their native visible content is one catalog entry; page through that content
// via SetPageData so its rank variants, text and owned-ring state stay native.
#[skyline::hook(offset = 0x242e120)]
unsafe fn fee_ring_catalog_tick(sequence: P, method: OptionalMethod) {
    call_original!(sequence, method);
    if sequence.is_null() { return; }
    let root: P = read(sequence, 0xc0);
    if root.is_null() || !read::<P>(root, 0x10).is_null() { return; }
    let list: P = read(sequence, 0x78);
    if list.is_null() { return; }
    let count: i32 = read(list, 0x18);
    let up = native!(0x1f23010, bool; u64 => 1 << 8)
        || native!(0x1f23100, bool; u64 => 1 << 8);
    let down = native!(0x1f23010, bool; u64 => 1 << 9)
        || native!(0x1f23100, bool; u64 => 1 << 9);
    if count > 0 && up != down {
        let index: i32 = read(sequence, 0x80);
        let next = (index + if down { 1 } else { -1 }).clamp(0, count - 1);
        let items: P = read(list, 0x10);
        if next != index && !items.is_null() {
            let page: P = read(items, 0x20 + next as usize * 8);
            if !page.is_null() {
                write(sequence, 0x80, next);
                native!(0x242d770, (); P => sequence, P => page);
            }
        }
    }
    update_hints(sequence as usize, count > 0, 0);
}

// Each entry replaces just the audited Pad static-field load. Keep native
// fresh-edge checks, focus callbacks, multi-select accounting and cancel events.
pub const INPUT_PATCHES: &[(usize, u32, u32)] = &[
    (0x1ebbbc0, 0xf9402514, 0xf9401514), // ring: open ability list
    (0x1b199f8, 0xf9402514, 0xf9401514), // ability list: close
    (0x279e274, 0xf940211b, 0xf940151b), // unit inventory: former ZL
    (0x279e348, 0xf9402519, 0xf9401519), // unit inventory: former ZR
    (0x279e370, 0xf9402519, 0xf9401519), // same class-init path
    (0x2799204, 0xf940211b, 0xf940151b), // convoy: former ZL
    (0x27992d8, 0xf9402519, 0xf9401519), // convoy: former ZR
    (0x2799300, 0xf9402519, 0xf9401519), // same class-init path
    (0x21b6f78, 0xf9402519, 0xf9401519), // sell item: multi-select
    (0x21b2548, 0xf9402519, 0xf9401519), // sell empty item: same input
];

pub fn valid() -> bool {
    INPUT_PATCHES.iter().all(|(offset, old, _)| unsafe {
        *((unity::module_base() + offset) as *const u32) == *old
    })
}

pub fn install() {
    if !valid() { log("menu input mask mismatch; paging hints disabled"); return; }
    for &(offset, _, new) in INPUT_PATCHES {
        if skyline::patching::Patch::in_text(offset).data(new).is_err() {
            log("menu input mask patch failed; paging hints disabled");
            return;
        }
    }
    skyline::install_hooks!(fee_key_help, fee_key_help_clear, fee_ring_custom, fee_ring_catalog_tick);
}
