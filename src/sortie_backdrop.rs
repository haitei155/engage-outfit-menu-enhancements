//! Accepted battle HUD suppression, restored independently of scene teardown.
use std::sync::{Mutex, atomic::{AtomicBool, Ordering}};
use engage::unity_engine::{canvas::Canvas, behaviour::IBehaviourMethods,
    component::IComponentMethods, gameobject::{GameObject, IGameObjectMethods},
    object_2::Object_2, transform::ITransformMethods};
use engage::root::mapinfoiconlocatorroot::MapInfoIconLocatorRoot;
use unity::{Cast, FromIlInstance, OptionalMethod, SystemObject};
static HIDDEN: Mutex<Vec<(usize, bool)>> = Mutex::new(Vec::new());
static ICONS: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static SUPPRESS: AtomicBool = AtomicBool::new(false);
fn wrap<T: FromIlInstance>(p: usize) -> T {
    T::from_il_instance(unity::IlInstance::from_raw(p as *mut ()))
}
fn suppressing() -> bool { SUPPRESS.load(Ordering::Acquire) && super::sortie_accessory::is_open() }
fn hide(canvas: Canvas) {
    if canvas.is_null() { return; }
    let mut hidden = HIDDEN.lock().unwrap();
    let ptr = canvas.as_instance().as_ptr() as usize;
    if !hidden.iter().any(|x| x.0 == ptr) { hidden.push((ptr, canvas.get_enabled())); }
    canvas.set_enabled(false);
}
pub fn hide_map_ui() {
    SUPPRESS.store(true, Ordering::Release);
    for canvas in Object_2::find_objects_of_type_3::<Canvas>().iter() {
        let mut t = canvas.get_transform();
        for _ in 0..16 {
            if t.is_null() { break; }
            let go = t.get_game_object();
            if !go.get_component_by_name("MapInfoRoot").is_null()
                || !go.get_component_by_name("MapInfoIconLocatorRoot").is_null() {
                hide(canvas); break;
            }
            t = t.get_parent();
        }
    }
    let mut icons = ICONS.lock().unwrap();
    for icon in Object_2::find_objects_of_type_3::<MapInfoIconLocatorRoot>().iter() {
        let go = icon.get_game_object();
        if go.get_active_self() {
            icons.push(go.as_instance().as_ptr() as usize);
            go.set_active(false);
        }
    }
    super::sortie_accessory::trace_menu(&format!("HUD hidden map canvases={}", HIDDEN.lock().unwrap().len()));
    super::sortie_accessory::trace_menu(&format!("HUD hidden terrain icon roots={}", icons.len()));
}
// Field names are resolved from the actual class hierarchy, without assuming
// the generated CustomForwardRenderer layout matches this executable.
pub(crate) fn field<T: Copy>(obj: unity::IlInstance, name: &str) -> Option<T> {
    if obj.is_null() { return None; }
    for class in obj.get_class().hierarchy() {
        for f in class.declared_fields() {
            if f.get_name().as_deref() == Some(name) {
                return Some(unity::field_get_value(obj, f));
            }
        }
    }
    None
}
pub fn restore() {
    SUPPRESS.store(false, Ordering::Release);
    let mut hidden = HIDDEN.lock().unwrap();
    for &(p, enabled) in hidden.iter() { wrap::<Canvas>(p).set_enabled(enabled); }
    if !hidden.is_empty() { super::sortie_accessory::trace_menu(&format!("HUD restored {} map canvases", hidden.len())); }
    hidden.clear();
    for p in ICONS.lock().unwrap().drain(..) { wrap::<GameObject>(p).set_active(true); }
}
#[skyline::hook(offset = 0x2085ae0)]
fn fee_sortie_map_info_tick(this: unity::IlInstance, unit: engage::app::unit::Unit, method: OptionalMethod) {
    if suppressing() {
        if let Some(canvas) = field::<Canvas>(this, "m_Canvas") { hide(canvas); }
        return;
    }
    call_original!(this, unit, method);
}
pub fn supported() -> bool {
    [(0x2085ae0usize, 0xa9bd7bfdu32), (0x202a820usize, 0xa9be7bfdu32)]
        .into_iter().all(|(offset, word)| unsafe { *((unity::module_base() + offset) as *const u32) == word })
}
#[skyline::hook(offset = 0x202a820)]
fn fee_sortie_map_icon_update(this: unity::IlInstance, method: OptionalMethod) {
    if !suppressing() { call_original!(this, method); }
}
pub fn install() { skyline::install_hooks!(fee_sortie_map_info_tick, fee_sortie_map_icon_update); }
