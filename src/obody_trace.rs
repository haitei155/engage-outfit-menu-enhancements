//! Read-only oBody resolution and post-load material diagnostics, Engage 2.0.0.
use std::{collections::HashSet, hash::{Hash, Hasher}, io::Write, sync::{Mutex, OnceLock, atomic::{AtomicBool, Ordering}}};
use unity::{OptionalMethod, SystemObject};
type P = *mut u8;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Color { r: f32, g: f32, b: f32, a: f32 }
macro_rules! native {
    ($offset:expr, $ret:ty; $($ty:ty => $arg:expr),* $(,)?) => {{
        let f: extern "C" fn($($ty,)* OptionalMethod) -> $ret = std::mem::transmute(unity::module_base() + $offset);
        f($($arg,)* None)
    }};
}
static ENABLED: AtomicBool = AtomicBool::new(false);
static SEEN: OnceLock<Mutex<HashSet<u64>>> = OnceLock::new();
static PROPERTIES: OnceLock<[i32; 5]> = OnceLock::new();
unsafe fn read<T: Copy>(p: P, off: usize) -> T { std::ptr::read_unaligned(p.add(off) as *const T) }
unsafe fn string(p: P) -> String {
    if p.is_null() { return String::new(); }
    let len = read::<i32>(p, 0x10);
    if !(0..=1024).contains(&len) { return String::from("<invalid string>"); }
    String::from_utf16_lossy(std::slice::from_raw_parts(p.add(0x14) as *const u16, len as usize))
}
unsafe fn name(p: P) -> String {
    if p.is_null() || read::<P>(p, 0x10).is_null() { return String::from("<none>"); }
    string(native!(0x32e8e90, P; P => p))
}
unsafe fn texture(p: P) -> String {
    if p.is_null() || read::<P>(p,0x10).is_null() { return String::from("<none>"); }
    format!("{}[{}x{};mips={}]",name(p),native!(0x3789400,i32; P=>p),native!(0x3789450,i32; P=>p),native!(0x37893a0,i32; P=>p))
}
fn write(line: &str) {
    let line = format!("[FEE oBody Trace v1] {line}\n");
    let _ = horizon_svc::output_debug_string(&line);
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open("sd:/engage/fee-obody-trace.log") {
        let _ = f.write_all(line.as_bytes());
    }
}
fn unique(line: String) {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    line.hash(&mut hasher);
    let emit = if let Ok(mut seen) = SEEN.get_or_init(|| Mutex::new(HashSet::new())).lock() {
        seen.len() < 4096 && seen.insert(hasher.finish())
    } else { false };
    if emit { write(&line); }
}
unsafe fn pid(unit: P) -> String {
    if unit.is_null() { return String::new(); }
    let person=read::<P>(unit,0x40);
    if person.is_null() { return String::new(); }
    string(read(person,0x20))
}
unsafe fn flags() -> String {
    let base=unity::module_base() as P;
    let slot=read::<P>(base,0x6286840);
    if slot.is_null() { return String::new(); }
    let klass=read::<P>(slot,0);
    if klass.is_null() { return String::new(); }
    let statics=read::<P>(klass,0xb8);
    if statics.is_null() { return String::new(); }
    let flags=read::<P>(statics,0x18);
    if flags.is_null() { return String::new(); }
    let list=read::<P>(flags,0x18);
    if list.is_null() { return String::new(); }
    let n=read::<i32>(list,0x18);
    let items=read::<P>(list,0x10);
    if items.is_null() || !(0..=1024).contains(&n) || read::<usize>(items,0x18)<n as usize { return String::new(); }
    (0..n as usize).map(|i| string(read(items,0x20+i*8))).collect::<Vec<_>>().join(";")
}
unsafe fn property_ids() -> &'static [i32; 5] {
    PROPERTIES.get_or_init(|| ["_Preset", "_Mask_ON", "_Color", "_BaseMap", "_MultiMap"].map(|s| {
        let text: unity::Il2CppString = s.into();
        native!(0x2f8f0a0, i32; P => text.as_instance().as_ptr() as P)
    }))
}
// This reports the final selected address. It does not claim the prefab loaded.
pub unsafe fn resolved(result: P, mode: i32, unit: P, state: i32, probe: bool) -> P {
    if !ENABLED.load(Ordering::Relaxed) || result.is_null() || mode != 1 || probe { return result; }
    let body = string(read(result, 0x20));
    if !body.to_ascii_lowercase().starts_with("obody_") { return result; }
    let dress = string(read(result, 0x28));
    let colors = read::<[f32; 16]>(result, 0xc0);
    unique(format!("RESOLVED unit={unit:p} state={state} mode={mode} body={body} dress={dress} pid={} conditions={} mask_colors={colors:?}",pid(unit),flags()));
    result
}
// CommitColor runs after materials are bound. Log after calling the original;
// never change fields, shaders, colors, keyword state, or resource loading.
#[skyline::hook(offset = 0x1fbd360)]
unsafe fn fee_obody_commit_color(model: P, flags: i32, method: OptionalMethod) {
    call_original!(model, flags, method);
    if model.is_null() { return; }
    let root = read::<P>(model, 0xb0);
    let body=read::<P>(model,0x170);
    let body_name=name(body);
    if !body_name.to_ascii_lowercase().starts_with("obody_") { return; }
    let root_name=format!("{} body={body_name}", name(root));
    let unit = read::<P>(model, 0x18);
    let list = read::<P>(model, 0x40);
    if list.is_null() { return; }
    let count = read::<i32>(list, 0x18);
    let items = read::<P>(list, 0x10);
    if items.is_null() || !(0..=256).contains(&count) { return; }
    let length = read::<usize>(items, 0x18);
    if length < count as usize || length > 4096 { return; }
    unique(format!("LOADED_COLOR unit={unit:p} root={root_name} flags={flags} materials={count} load_mode={} pid={} skin={:?} mask100={:?} mask075={:?} mask050={:?} mask025={:?}",
        read::<i32>(model,0xd0), pid(unit), read::<Color>(model,0x60), read::<Color>(model,0x70), read::<Color>(model,0x80), read::<Color>(model,0x90), read::<Color>(model,0xa0)));
    let ids = property_ids();
    for i in 0..count as usize {
        let material = read::<P>(items, 0x20 + i * 8);
        if material.is_null() || read::<P>(material, 0x10).is_null() { continue; }
        let mut data = format!("MATERIAL unit={unit:p} root={root_name} slot={i} name={}",name(material));
        for (label, id) in ["preset", "mask"].iter().zip(ids.iter()) {
            if native!(0x32da270,bool; P=>material, i32=>*id) {
                data += &format!(" {label}={}",native!(0x32db3e0,f32; P=>material,i32=>*id));
            }
        }
        if native!(0x32da270,bool; P=>material,i32=>ids[2]) {
            data += &format!(" color={:?}",native!(0x32db430,Color; P=>material,i32=>ids[2]));
        }
        let shader=native!(0x32d92c0,P; P=>material);
        data += &format!(" shader={}", name(shader));
        let keyword: unity::Il2CppString = "_S_KEY_COLOR_CHANGE_MASK".into();
        data += &format!(" tint_keyword={}",native!(0x32da4b0,bool; P=>material,P=>keyword.as_instance().as_ptr() as P));
        for (label,id) in ["base", "multi"].iter().zip(ids[3..].iter()) {
            if native!(0x32da270,bool; P=>material,i32=>*id) {
                data += &format!(" {label}={}",texture(native!(0x32db600,P; P=>material,i32=>*id)));
            }
        }
        unique(data);
    }
}
pub fn install() {
    let base=unity::module_base();
    let guards: &[(usize,u32)] = &[
        (0x3789400, 0xa9be7bfd),
        (0x3789450, 0xa9be7bfd),
        (0x37893a0, 0xa9be7bfd),
        (0x1fbb740, 0xf940b800),
        (0x1f25c20, 0xf9401000),
        (0x1fbd360, 0x6db83bef),
        (0x1fbd364, 0x6d0133ed),
        (0x1fbd368, 0x6d022beb),
        (0x1fbd36c, 0x6d0323e9),
        (0x32e8e90, 0xa9bd7bfd),
        (0x32d92c0, 0xa9be7bfd),
        (0x32da270, 0xa9bd7bfd),
        (0x32db3e0, 0xa9bd7bfd),
        (0x32db430, 0xd10103ff),
        (0x32db600, 0xa9bd7bfd),
        (0x32da4b0, 0xa9bd7bfd),
        (0x2f8f0a0, 0xa9be7bfd),
    ];
    if !guards.iter().all(|&(o,w)| unsafe { std::ptr::read_unaligned((base+o) as *const u32)==w }) {
        write("unsupported executable: post-load tracing disabled"); return;
    }
    skyline::install_hooks!(fee_obody_commit_color);
    ENABLED.store(true,Ordering::Relaxed);
    write("BEGIN build=20261001-fullmip-diagnostics; RESOLVED=selected address; LOADED_COLOR=post-load color commit; MATERIAL=live bindings; unique events capped at 4096 per launch");
}
