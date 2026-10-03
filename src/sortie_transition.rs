//! Native fade descriptors fence scene loading and teardown on the Proc timeline.
use std::sync::{Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use engage::app::{fade::{Fade, Fade_Layer}, procdesc::ProcDesc,
    procdescwaittrue::ProcDescWaitTrue, procboolfunction::ProcBoolFunction, procinst::ProcInst,
    unit::{Unit, IUnitMethods}, unitactor::IUnitActorMethods};
use engage::unity_engine::time::Time;
use unity::{Cast, FromIlInstance, IlNull, OptionalMethod, SystemObject};

static ENTRY: AtomicBool = AtomicBool::new(false);
static CHANGED: Mutex<Vec<usize>> = Mutex::new(Vec::new());
static RELOAD_FRAME: AtomicUsize = AtomicUsize::new(0);
const DURATION: f32 = 0.25;

fn callback_is(desc: ProcDesc, offset: usize) -> bool {
    for name in ["m_Method", "m_Function"] {
        let Some(delegate) = super::sortie_backdrop::field::<unity::IlInstance>(desc.as_instance(), name) else { continue; };
        if super::sortie_backdrop::field::<usize>(delegate, "method_ptr") == Some(unity::module_base() + offset) {
            return true;
        }
    }
    false
}
pub fn descriptors(original: unity::Array<ProcDesc>) -> unity::Array<ProcDesc> {
    CHANGED.lock().unwrap().clear();
    let native: Vec<ProcDesc> = original.iter().collect();
    let Some(end) = native.iter().position(|d| callback_is(*d, 0x2d76340)) else {
        super::sortie_accessory::trace_menu("TRANSITION ERROR EndSequence descriptor missing; native sequence retained");
        ENTRY.store(false, Ordering::Release);
        return original;
    };
    let layer = Fade_Layer::system();
    let mut result = Vec::with_capacity(native.len() + 7);
    result.push(Fade::black_out(DURATION, layer));
    result.push(Fade::fade_wait(layer));
    ENTRY.store(true, Ordering::Release);
    for (i, desc) in native.into_iter().enumerate() {
        if i == end {
            result.push(Fade::black_out(DURATION, layer));
            result.push(Fade::fade_wait(layer));
        }
        result.push(desc);
        if i == end {
            // EndSequence restores the battle and starts only changed actors.
            // Its child scene unload completes before this wait executes.
            // Use the concrete descriptor; its delegate is bypassed by our IsWait override.
            let wait = ProcDescWaitTrue::new(ProcBoolFunction::null());
            wait.override_class().override_virtual_method("IsWait", unity::method_info!(wait_changed_actors, 1));
            result.push(wait.into());
            result.push(Fade::black_in(DURATION, layer));
            result.push(Fade::fade_wait(layer));
        }
    }
    super::sortie_accessory::trace_menu(&format!("TRANSITION native 0.25s fades inserted; EndSequence index={end}; entry reveal waits first model"));
    result.into()
}
pub fn model_ready() {
    if ENTRY.swap(false, Ordering::AcqRel) {
        Fade::fade_in_2(Fade_Layer::system(), DURATION);
        super::sortie_accessory::trace_menu("TRANSITION entry model complete; native fade in 0.25s");
    }
}
pub fn input_blocked() -> bool {
    ENTRY.load(Ordering::Acquire) || Fade::is_active_2(Fade_Layer::system())
}
pub fn track_changed_unit(unit: Unit) {
    CHANGED.lock().unwrap().push(unit.as_instance().as_ptr() as usize);
    RELOAD_FRAME.store(Time::get_frame_count() as usize, Ordering::Release);
}
extern "C" fn wait_changed_actors(_: ProcDescWaitTrue, _: ProcInst, _: OptionalMethod) -> bool {
    let mut units = CHANGED.lock().unwrap();
    if units.is_empty() { return false; }
    if (Time::get_frame_count() as usize).saturating_sub(RELOAD_FRAME.load(Ordering::Acquire)) < 2 { return true; }
    let mut waiting = false;
    for &p in units.iter() {
        let unit = Unit::from_il_instance(unity::IlInstance::from_raw(p as *mut ()));
        let actor = unit.get_actor();
        if !actor.is_null() {
            actor.update_loading();
            waiting |= actor.is_loading();
        }
    }
    if waiting { return true; }
    super::sortie_accessory::trace_menu(&format!("TRANSITION exit {} changed actors ready; native fade in", units.len()));
    units.clear();
    false
}
pub fn supported() -> bool {
    // Filled from the extracted 2.0.0 executable, including the actor readiness
    // query and wait constructor used while the black curtain is held.
    GUARDS.into_iter().all(|(offset, word)| unsafe { *((unity::module_base() + offset) as *const u32) == word })
}
const GUARDS: [(usize, u32); 9] = [
    (0x2d53a60, 0xfc1b0fec), (0x2d539a0, 0xfc1b0fec), (0x2d53e00, 0xa9bd7bfd),
    (0x2d52d90, 0xd10103ff), (0x2d53310, 0xa9bd7bfd), (0x281b8f0, 0xa9be7bfd), (0x1f679f0, 0xa9bd7bfd),
    (0x1f67ba0, 0xa9bd7bfd),
    (0x1f67af0, 0xa9be7bfd),
];
