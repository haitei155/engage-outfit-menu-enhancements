//! Standalone environment option. No XML replacement and no character/outfit allowlist.
mod text;
use std::{cell::Cell, io::Write, sync::atomic::{AtomicBool, AtomicUsize, Ordering}};
use engage::app::{basicmenu::BasicMenu, language::Language};
use engage::root::configbasicmenuitem::{ConfigBasicMenuItem, IConfigBasicMenuItem, IConfigBasicMenuItemMethods};
use engage::{BasicMenuExt, BasicMenuResult, ConfigBasicMenuItemExt, ConfigBasicMenuItemSwitchMethods};
use unity::{Cast, OptionalMethod};

type P = *mut u8;
const SETTING: &str = "sd:/engage/fee-engage-outfit.cfg";
static KEEP_CURRENT: AtomicBool = AtomicBool::new(false);
static LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
static ATTACK_LOG_COUNT: AtomicUsize = AtomicUsize::new(0);
static ATTACK_TRACE_COUNT: AtomicUsize = AtomicUsize::new(0);
thread_local! {
    static ORDINARY_PASS: Cell<bool> = const { Cell::new(false) };
    static EMBLEM_PASS: Cell<bool> = const { Cell::new(false) };
}
fn log(s: &str) {
    let line = format!("[FEE Engage Outfit v4] {s}\n");
    let _ = horizon_svc::output_debug_string(&line);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("sd:/engage/fee-engage-outfit-v4.log") {
        let _ = f.write_all(line.as_bytes());
    }
}
fn save_setting(value: bool) -> std::io::Result<()> {
    // Flush before publishing the new in-memory choice; a failed write keeps the old setting.
    let mut file = std::fs::File::create(SETTING)?;
    file.write_all(if value { b"current\n" } else { b"emblem\n" })?;
    file.sync_all()
}
struct OutfitSetting;
impl ConfigBasicMenuItemSwitchMethods for OutfitSetting {
    extern "C" fn custom_call(this: ConfigBasicMenuItem, _: OptionalMethod) -> BasicMenuResult {
        let old = KEEP_CURRENT.load(Ordering::Relaxed);
        let next = ConfigBasicMenuItem::change_key_value_b(old);
        if next == old { return BasicMenuResult::new(); }
        if let Err(error) = save_setting(next) { log(&format!("setting write failed: {error}")); return BasicMenuResult::new(); }
        KEEP_CURRENT.store(next, Ordering::Relaxed);
        Self::set_command_text(this, None);
        Self::set_help_text(this, None);
        this.update_text();
        log(if next { "selected current outfit" } else { "selected emblem outfit" });
        BasicMenuResult::se_cursor()
    }
    extern "C" fn set_command_text(this: ConfigBasicMenuItem, _: OptionalMethod) {
        log("menu: setting command text");
        let t = text::text(Language::get_lang().value);
        this.set_m_command_text(if KEEP_CURRENT.load(Ordering::Relaxed) { t.current } else { t.emblem }.into());
    }
    extern "C" fn set_help_text(this: ConfigBasicMenuItem, _: OptionalMethod) {
        log("menu: setting help text");
        this.set_m_help_text(text::text(Language::get_lang().value).help.into());
    }
}
// ConfigMenu's tiny constructor tail-calls this full BasicMenu constructor.
// Filter by the allocated object's class. Cobalt keeps its CreateBind hook;
// our row enters the completed list before Cobalt appends its own settings row.
#[skyline::hook(offset = 0x24533f0)]
fn fee_outfit_v2_menu(menu: BasicMenu, list: P, content: P, method: OptionalMethod) {
    let environment = !menu.is_null() && menu.get_class().name() == "ConfigMenu";
    if environment { log("menu: entering native BasicMenu constructor"); }
    call_original!(menu, list, content, method);
    if environment {
        log("menu: native constructor completed; reading language");
        let language = Language::get_lang().value;
        log(&format!("menu: language={language}; creating switch item"));
        let item = ConfigBasicMenuItem::new_switch::<OutfitSetting>(text::text(language).title);
        log("menu: switch item created; appending to menu list");
        menu.add_item(item);
        log("menu: environment option added");
    }
}
unsafe fn read<T: Copy>(p: P, off: usize) -> T { std::ptr::read_unaligned(p.add(off) as *const T) }
unsafe fn string(p: P) -> String {
    if p.is_null() { return String::new(); }
    let len = read::<i32>(p, 0x10);
    if !(0..=1024).contains(&len) { return String::new(); }
    String::from_utf16_lossy(std::slice::from_raw_parts(p.add(0x14) as *const u16, len as usize))
}
unsafe fn flags_have_transform() -> bool {
    let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as P;
    let slot = read::<P>(base, 0x6286840);
    if slot.is_null() { return true; }
    let klass = read::<P>(slot, 0);
    if klass.is_null() { return true; }
    let statics = read::<P>(klass, 0xb8);
    if statics.is_null() { return true; }
    let flags = read::<P>(statics, 0x18);
    if flags.is_null() { return true; }
    let list = read::<P>(flags, 0x18);
    if list.is_null() { return true; }
    let len = read::<i32>(list, 0x18);
    let array = read::<P>(list, 0x10);
    if array.is_null() || !(0..=1024).contains(&len) { return true; }
    (0..len as usize).any(|i| {
        let token = string(read::<P>(array, 0x20 + i * 8));
        token == "竜化" || token == "チキ"
    })
}
// Engine-owned strings stay on the hook's stack during the second pass. Boehm GC
// scans this registered game thread conservatively. Never retain them across frames.
struct Clothes { body: P, dress: P, colors: [u8; 64] }
impl Clothes {
    unsafe fn take(result: P) -> Self {
        Self { body: read(result, 0x20), dress: read(result, 0x28), colors: read(result, 0xc0) }
    }
    unsafe fn apply(self, result: P, mode: i32, kind: &str) {
        if result.is_null() || flags_have_transform() { return; }
        let usable = if mode == 1 { !string(self.body).is_empty() } else { !string(self.dress).is_empty() };
        if !usable { return; }
        let base = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize;
        // Native reference setters include the IL2CPP write barrier.
        let set_body: unsafe extern "C" fn(P, P, P) = std::mem::transmute(base + 0x1bb1cf0);
        let set_dress: unsafe extern "C" fn(P, P, P) = std::mem::transmute(base + 0x1bb1d10);
        set_body(result, self.body, std::ptr::null_mut());
        if mode == 2 { set_dress(result, self.dress, std::ptr::null_mut()); }
        std::ptr::copy_nonoverlapping(self.colors.as_ptr(), result.add(0xc0), 64);
        let counter = if kind == "current" { &LOG_COUNT } else { &ATTACK_LOG_COUNT };
        if counter.fetch_add(1, Ordering::Relaxed) < 128 {
            log(&format!("restored {kind} mode={mode} body={} dress={}", string(self.body), string(self.dress)));
        }
    }
}
// During the ordinary model pass alone, generate non-engaged flags. The second
// pass receives every original argument and rebuilds the actual engaged result.
#[skyline::hook(offset = 0x1bb0440)]
unsafe fn fee_outfit_v1_flags(flags: P, state: i32, god: P, dark: bool, method: P) {
    if ORDINARY_PASS.with(Cell::get) { call_original!(flags, 0, std::ptr::null_mut(), false, method); }
    else if EMBLEM_PASS.with(Cell::get) { call_original!(flags, 1, god, dark, method); }
    else { call_original!(flags, state, god, dark, method); }
}
#[skyline::hook(offset = 0x1bb2430)]
unsafe fn fee_outfit_v1_unit(result: P, mode: i32, unit: P, equipped: P, conditions: P, method: P) -> P {
    if !matches!(mode, 1 | 2) || result.is_null() || unit.is_null() || ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get) {
        return crate::obody_trace::resolved(call_original!(result, mode, unit, equipped, conditions, method), mode, unit, -1, ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get));
    }
    let get_state: unsafe extern "C" fn(P, P) -> i32 = std::mem::transmute(skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize + 0x1bb0100);
    let state = get_state(unit, std::ptr::null_mut());
    if !KEEP_CURRENT.load(Ordering::Relaxed) {
        if !matches!(state, 2..=4) { return crate::obody_trace::resolved(call_original!(result, mode, unit, equipped, conditions, method), mode, unit, -1, ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get)); }
        // Combat/cut-in GetFromUnit tail-calls this overload, not the full
        // Person/Job/God overload. Normalize only flags in the clothing probe;
        // never change the Unit's Engage state or equipment.
        EMBLEM_PASS.with(|f| f.set(true));
        let engaged = call_original!(result, mode, unit, equipped, conditions, method);
        let clothes = Clothes::take(engaged);
        EMBLEM_PASS.with(|f| f.set(false));
        let actual = call_original!(result, mode, unit, equipped, conditions, method);
        if ATTACK_TRACE_COUNT.fetch_add(1, Ordering::Relaxed) < 128 {
            log(&format!("Unit attack state={state} mode={mode} engaged dress={} attack dress={}", string(clothes.dress), string(read(actual, 0x28))));
        }
        clothes.apply(actual, mode, "emblem Unit attack");
        return crate::obody_trace::resolved(actual, mode, unit, state, false);
    }
    if !matches!(state, 1..=4) {
        return crate::obody_trace::resolved(call_original!(result, mode, unit, equipped, conditions, method), mode, unit, -1, ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get));
    }
    ORDINARY_PASS.with(|f| f.set(true));
    let normal = call_original!(result, mode, unit, equipped, conditions, method);
    let clothes = Clothes::take(normal);
    ORDINARY_PASS.with(|f| f.set(false));
    let actual = call_original!(result, mode, unit, equipped, conditions, method);
    clothes.apply(actual, mode, "current");
    crate::obody_trace::resolved(actual, mode, unit, state, false)
}
// Covers combat previews, Engage attacks and link attacks which pass explicit state
// rather than a Unit. No PID, JID, EID or AID list is baked into this plugin.
#[skyline::hook(offset = 0x1bb43a0)]
unsafe fn fee_outfit_v1_full(result: P, mode: i32, person: P, job: P, god: P, equipped: P, force: i32, state: i32, dark: bool, conditions: P, method: P) -> P {
    // Attack states 2/3/4 have their own flags, and many normal Engage outfit
    // rules exclude them. Resolve clothing with state 1 from the live table,
    // then rebuild the true attack and replace clothing only. No stored model
    // pointers or character/emblem lists; transformation checks use the final pass.
    if !KEEP_CURRENT.load(Ordering::Relaxed) && matches!(mode, 1 | 2) && matches!(state, 2..=4) && !result.is_null() && !god.is_null() && !ORDINARY_PASS.with(Cell::get) && !EMBLEM_PASS.with(Cell::get) {
        EMBLEM_PASS.with(|f| f.set(true));
        let engaged = call_original!(result, mode, person, job, god, equipped, force, 1, dark, conditions, method);
        let clothes = Clothes::take(engaged);
        EMBLEM_PASS.with(|f| f.set(false));
        let actual = call_original!(result, mode, person, job, god, equipped, force, state, dark, conditions, method);
        clothes.apply(actual, mode, "emblem attack");
        return crate::obody_trace::resolved(actual, mode, std::ptr::null_mut(), state, false);
    }
    if !KEEP_CURRENT.load(Ordering::Relaxed) || !matches!(mode, 1 | 2) || !matches!(state, 1..=4) || result.is_null() || ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get) {
        return crate::obody_trace::resolved(call_original!(result, mode, person, job, god, equipped, force, state, dark, conditions, method), mode, std::ptr::null_mut(), state, ORDINARY_PASS.with(Cell::get) || EMBLEM_PASS.with(Cell::get));
    }
    ORDINARY_PASS.with(|f| f.set(true));
    let normal = call_original!(result, mode, person, job, god, equipped, force, state, dark, conditions, method);
    let clothes = Clothes::take(normal);
    ORDINARY_PASS.with(|f| f.set(false));
    let actual = call_original!(result, mode, person, job, god, equipped, force, state, dark, conditions, method);
    clothes.apply(actual, mode, "current");
    crate::obody_trace::resolved(actual, mode, std::ptr::null_mut(), state, false)
}
pub fn install() {
    KEEP_CURRENT.store(std::fs::read_to_string(SETTING).map(|s| text::decode_setting(&s)).unwrap_or(false), Ordering::Relaxed);
    if !supported() { log("unsupported executable: outfit hooks disabled"); return; }
    skyline::install_hooks!(fee_outfit_v2_menu, fee_outfit_v1_flags, fee_outfit_v1_unit, fee_outfit_v1_full);
    log(if KEEP_CURRENT.load(Ordering::Relaxed) { "installed constructor and three asset hooks; loaded current outfit" } else { "installed constructor and three asset hooks; loaded emblem outfit (default)" });
}

pub fn supported() -> bool {
    let base = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as usize };
    // Game 2.0.0 code and field access guards, including the unshared constructor.
    let guards: &[(usize, u32)] = &[
        (0x1bb0440, 0xa9bc7bfd),
        (0x1bb0444, 0xa9015ff8),
        (0x1bb0448, 0x910003fd),
        (0x1bb044c, 0xa90257f6),
        (0x1bb2430, 0xa9ba7bfd),
        (0x1bb2434, 0xf9000bfb),
        (0x1bb2438, 0x910003fd),
        (0x1bb243c, 0xa90267fa),
        (0x1bb43a0, 0xd101c3ff),
        (0x1bb43a4, 0xa9017bfd),
        (0x1bb43a8, 0x910043fd),
        (0x1bb43ac, 0xa9026ffc),
        (0x24533f0, 0xd10203ff),
        (0x24533f4, 0xa9027bfd),
        (0x24533f8, 0x910083fd),
        (0x24533fc, 0xa9036ffc),
        (0x1bb1ce0, 0xf9401000),
        (0x1bb1cf0, 0xf8020c01),
        (0x1bb1cf4, 0x17a380af),
        (0x1bb1d00, 0xf9401400),
        (0x1bb1d10, 0xf8028c01),
        (0x1bb1d14, 0x17a380a7),
        (0x1bb1f20, 0x2d580400),
        (0x1bb1f24, 0x2d590c02),
        (0x1bb1f50, 0x2d5a0400),
        (0x1bb1f54, 0x2d5b0c02),
        (0x1bb1f80, 0x2d5c0400),
        (0x1bb1f84, 0x2d5d0c02),
        (0x1bb1fb0, 0x2d5e0400),
        (0x1bb1fb4, 0x2d5f0c02),
    ];
    guards.iter().all(|&(off, word)| unsafe { std::ptr::read_unaligned((base + off) as *const u32) == word })
}
