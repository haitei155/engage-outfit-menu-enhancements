//! Battle-only adapter for the original accessory scene and camera component.
//! No HubLocatorGroup/HubPlayerController save/restore operations are invoked.
use std::sync::{Mutex, atomic::{AtomicBool, AtomicUsize, Ordering}};
use engage::app::{procinst::ProcInst, procscenemanager::ProcSceneManager,
    hubaccessoryshopsequence::HubAccessoryShopSequence,
    hubaccessoryroom::HubAccessoryRoom_ViewMode,
    hubaccessoryroomcamera::{HubAccessoryRoomCamera, IHubAccessoryRoomCameraMethods},
    accessorydata::{AccessoryData, IAccessoryDataMethods}, pad::Pad};
use engage::unity_engine::{camera::{Camera, ICameraMethods},
    component::IComponentMethods, behaviour::{Behaviour, IBehaviourMethods},
    renderer::{Renderer, IRendererMethods}, light::Light,
    object_2::Object_2, gameobject::IGameObjectMethods, transform::ITransformMethods,
    vector3::Vector3, rect::Rect, cameraclearflags::CameraClearFlags,
    scene_management::{scenemanager::SceneManager, scene::Scene, loadscenemode::LoadSceneMode}};
use unity::{Cast, FromIlInstance, IlNull, OptionalMethod, SystemObject, SystemType};

static CONTROLLER: AtomicUsize = AtomicUsize::new(0);
static READY: AtomicBool = AtomicBool::new(false);
static INPUT: AtomicBool = AtomicBool::new(false);
static INPUT_FRAME: AtomicUsize = AtomicUsize::new(usize::MAX);
static VIEW: AtomicUsize = AtomicUsize::new(0);
static TARGET: AtomicUsize = AtomicUsize::new(0);
static OWNED: AtomicBool = AtomicBool::new(false);
static NATIVE_CAMERA: AtomicUsize = AtomicUsize::new(0);
static CAMERA_CONTEXT: AtomicBool = AtomicBool::new(false);
static AWAKE_DONE: AtomicBool = AtomicBool::new(false);
static SAVED: Mutex<Option<Saved>> = Mutex::new(None);

struct Saved {
    camera: usize, scene: i32, position: Vector3, rotation: Vector3,
    fov: f32, mask: i32, hdr: bool, msaa: bool, rect: Rect,
    clear: CameraClearFlags, enabled: bool, pipeline: usize, flags: [i32; 5],
    renderers: Vec<usize>, lights: Vec<usize>, scripts: Vec<usize>,
}
fn wrap<T: FromIlInstance>(p: usize) -> T {
    T::from_il_instance(unity::IlInstance::from_raw(p as *mut ()))
}
pub fn controller() -> Option<HubAccessoryRoomCamera> {
    let p = CONTROLLER.load(Ordering::Acquire);
    (p != 0).then(|| wrap(p))
}
pub fn ready() -> bool { READY.load(Ordering::Acquire) }
pub fn trace_camera() {
    let native = NATIVE_CAMERA.load(Ordering::Acquire);
    let Some(c) = controller() else { return; };
    if native == 0 || !ready() { return; }
    let camera: Camera = wrap(native);
    let p = c.as_instance().as_ptr() as *const u8;
    let (view, target, distance, fov) = unsafe {
        (std::ptr::read_unaligned(p.add(232).cast::<i32>()),
         std::ptr::read_unaligned(p.add(224).cast::<u64>()),
         std::ptr::read_unaligned(p.add(248).cast::<f32>()),
         std::ptr::read_unaligned(p.add(252).cast::<f32>()))
    };
    let t = camera.get_transform();
    let pos = t.get_position(); let angles = t.get_euler_angles();
    super::sortie_accessory::trace_menu(&format!(
        "CAMERA native view={view} target={target:#x} distance={distance} param_fov={fov} actual_fov={} aspect={} hdr={} mask={:#x} pos=({},{},{}) angles=({},{},{}) character={:p}",
        camera.get_field_of_view(), camera.get_aspect(), camera.get_allow_hdr(), camera.get_culling_mask(),
        pos.x, pos.y, pos.z, angles.x, angles.y, angles.z, c.get_character().as_instance().as_ptr()));
}
pub fn with_camera<T>(f: impl FnOnce() -> T) -> T {
    let previous = CAMERA_CONTEXT.swap(true, Ordering::AcqRel);
    let result = f();
    CAMERA_CONTEXT.store(previous, Ordering::Release);
    result
}
pub fn input(enable: bool) {
    let frame = engage::unity_engine::time::Time::get_frame_count() as usize;
    if enable {
        INPUT_FRAME.store(frame, Ordering::Release);
        INPUT.store(true, Ordering::Release);
    } else if INPUT_FRAME.load(Ordering::Acquire) != frame {
        INPUT.store(false, Ordering::Release);
    }
}
fn locate_camera() -> usize {
    let Some(t) = SystemType::of::<Camera>() else { return 0; };
    // Awake order is unspecified; include inactive cameras as well.
    let cameras = Object_2::find_objects_of_type_2(t, true);
    for i in 0..cameras.len() {
        let camera: Camera = wrap(cameras.get(i).as_instance().as_ptr() as usize);
        let mut scene = camera.get_game_object().get_scene();
        if scene.get_name().to_string() == "Hub_AccessoryRoom"
            && camera.get_game_object().get_tag().to_string() == "MainCamera" {
            return camera.as_instance().as_ptr() as usize;
        }
    }
    0
}
pub fn target(accessory: AccessoryData) {
    let value = if accessory.is_null() { 0 } else {
        (1usize << 32) | (accessory.get_kind().value as u32 as usize)
    };
    TARGET.store(value, Ordering::Release);
    if ready() { apply_target(); }
}
fn apply_target() {
    if let Some(c) = controller() {
        // Nullable<Kinds> is an eight-byte value; this method is omitted by bindgen.
        // The existing expanded-camera hook also maps Style to Body here.
        let set: extern "C" fn(HubAccessoryRoomCamera, u64, OptionalMethod) = unsafe {
            std::mem::transmute(unity::module_base() + 0x2d71d00)
        };
        with_camera(|| set(c, TARGET.load(Ordering::Acquire) as u64, None));
    }
}
pub fn activate() -> bool {
    if ready() { return true; }
    let mut scene = SceneManager::get_scene_by_name("Hub_AccessoryRoom");
    if !scene.is_valid() || !scene.get_is_loaded() { return false; }
    let Some(c) = controller() else { return false; };
    if NATIVE_CAMERA.load(Ordering::Acquire) == 0 {
        NATIVE_CAMERA.store(locate_camera(), Ordering::Release);
    }
    if NATIVE_CAMERA.load(Ordering::Acquire) == 0 { return false; }
    if !AWAKE_DONE.load(Ordering::Acquire) { c.awake(); }
    let saved = SAVED.lock().unwrap();
    let Some(s) = saved.as_ref() else { return false; };
    for &p in &s.renderers { wrap::<Renderer>(p).set_enabled(false); }
    for &p in &s.lights { wrap::<Behaviour>(p).set_enabled(false); }
    for &p in &s.scripts { wrap::<Behaviour>(p).set_enabled(false); }
    SceneManager::set_active_scene(scene);
    let native = NATIVE_CAMERA.load(Ordering::Acquire);
    if native == 0 { return false; }
    // The original scene includes its own MainCamera and complete URP data.
    // Only suspend the old battlefield camera; do not transplant its light stack.
    wrap::<Camera>(s.camera).set_enabled(false);
    let camera: Camera = wrap(native);
    camera.set_culling_mask(0x5007c711);
    camera.set_allow_hdr(true);
    camera.set_allow_msaa(false);
    camera.set_rect(Rect { m_x_min: 0.0, m_y_min: 0.0, m_width: 1.0, m_height: 1.0 });
    camera.set_clear_flags(CameraClearFlags::skybox());
    camera.set_enabled(true);
    READY.store(true, Ordering::Release);
    with_camera(|| c.set_view_mode(HubAccessoryRoom_ViewMode { value: VIEW.load(Ordering::Acquire) as i32 }));
    apply_target();
    with_camera(|| c.init_pos(true));
    super::sortie_accessory::trace_menu(&format!("SCENE ready native controller={:p} camera={:p} scene={} hidden_renderers={} lights={} scripts={}",
        c.as_instance().as_ptr(), camera.as_instance().as_ptr(), scene.m_handle,
        s.renderers.len(), s.lights.len(), s.scripts.len()));
    true
}
fn snapshot() {
    let camera = Camera::get_main();
    if camera.is_null() { super::sortie_accessory::trace_menu("SCENE ERROR main camera missing"); return; }
    let mut renderers = Vec::new();
    for r in Object_2::find_objects_of_type_3::<Renderer>().iter() {
        if !r.is_null() && r.get_enabled() { renderers.push(r.as_instance().as_ptr() as usize); }
    }
    let mut lights = Vec::new();
    for l in Object_2::find_objects_of_type_3::<Light>().iter() {
        if !l.is_null() && l.get_enabled() { lights.push(l.as_instance().as_ptr() as usize); }
    }
    let mut scripts = Vec::new();
    if let Some(t) = SystemType::of::<Behaviour>() {
        let list = camera.get_game_object().get_components(t);
        for i in 0..list.len() {
            let b = wrap::<Behaviour>(list.get(i).as_instance().as_ptr() as usize);
            if b.is_null() || b.as_instance().as_ptr() == camera.as_instance().as_ptr() { continue; }
            let name = b.get_class().name();
            if name != "UniversalAdditionalCameraData" && name != "AudioListener" && b.get_enabled() {
                scripts.push(b.as_instance().as_ptr() as usize);
            }
        }
    }
    let data = camera.get_game_object().get_component_by_name("UniversalAdditionalCameraData");
    let pipeline = data.as_instance().as_ptr() as usize;
    let flags = if pipeline == 0 { [0; 5] } else { unsafe {
        let p = pipeline as *const u8;
        [std::ptr::read_unaligned(p.add(52).cast::<i32>()), *p.add(64) as i32,
         std::ptr::read_unaligned(p.add(68).cast::<i32>()),
         std::ptr::read_unaligned(p.add(72).cast::<i32>()),
         std::ptr::read_unaligned(p.add(88).cast::<i32>())]
    }};
    *SAVED.lock().unwrap() = Some(Saved { camera: camera.as_instance().as_ptr() as usize,
        scene: SceneManager::get_active_scene().m_handle,
        position: camera.get_transform().get_position(), rotation: camera.get_transform().get_euler_angles(),
        fov: camera.get_field_of_view(), mask: camera.get_culling_mask(), hdr: camera.get_allow_hdr(),
        msaa: camera.get_allow_msaa(), rect: camera.get_rect(), clear: camera.get_clear_flags(),
        enabled: camera.get_enabled(), pipeline, flags, renderers, lights, scripts });
}
pub fn restore() {
    crate::sortie_backdrop::restore();
    READY.store(false, Ordering::Release);
    INPUT.store(false, Ordering::Release);
    INPUT_FRAME.store(usize::MAX, Ordering::Release);
    CONTROLLER.store(0, Ordering::Release);
    AWAKE_DONE.store(false, Ordering::Release);
    let native = NATIVE_CAMERA.swap(0, Ordering::AcqRel);
    if native != 0 { wrap::<Camera>(native).set_enabled(false); }
    if let Some(s) = SAVED.lock().unwrap().take() {
        for p in s.renderers { wrap::<Renderer>(p).set_enabled(true); }
        for p in s.lights { wrap::<Behaviour>(p).set_enabled(true); }
        for p in s.scripts { wrap::<Behaviour>(p).set_enabled(true); }
        let mut scene = Scene { m_handle: s.scene };
        if scene.is_valid() && scene.get_is_loaded() { SceneManager::set_active_scene(scene); }
        let camera: Camera = wrap(s.camera);
        camera.get_transform().set_position(s.position);
        camera.get_transform().set_euler_angles(s.rotation);
        camera.set_field_of_view(s.fov); camera.set_culling_mask(s.mask);
        camera.set_allow_hdr(s.hdr); camera.set_allow_msaa(s.msaa);
        camera.set_rect(s.rect); camera.set_clear_flags(s.clear); camera.set_enabled(s.enabled);
        if s.pipeline != 0 { unsafe {
            let p = s.pipeline as *mut u8;
            for (offset, value) in [52, 68, 72, 88].into_iter().zip([s.flags[0], s.flags[2], s.flags[3], s.flags[4]]) {
                std::ptr::write_unaligned(p.add(offset).cast::<i32>(), value);
            }
            *p.add(64) = s.flags[1] as u8;
        }}
        super::sortie_accessory::trace_menu("SCENE restored battlefield renderers, lights, camera and active scene");
    }
}

#[skyline::hook(offset = 0x2d757e0)]
fn fee_sortie_native_scene_load(this: HubAccessoryShopSequence, method: OptionalMethod) {
    call_original!(this, method);
    if !super::sortie_accessory::is_open() { return; }
    snapshot();
    crate::sortie_backdrop::hide_map_ui();
    super::sortie_accessory::capture_transition();
    OWNED.store(true, Ordering::Release);
    VIEW.store(0, Ordering::Release); TARGET.store(0, Ordering::Release);
    super::sortie_accessory::trace_menu("SCENE load Hub_AccessoryRoom additive via ProcSceneManager");
    ProcSceneManager::load_bind(<ProcInst as FromIlInstance>::from_il_instance(this.as_instance()),
        "Hub_AccessoryRoom", LoadSceneMode::additive());
}
#[skyline::hook(offset = 0x2d76340)]
fn fee_sortie_native_scene_end(this: HubAccessoryShopSequence, method: OptionalMethod) {
    if super::sortie_accessory::is_open() {
        super::sortie_accessory::release_native_preview();
        restore();
        super::sortie_accessory::refresh_changed_map_actors();
        if OWNED.swap(false, Ordering::AcqRel) {
            super::sortie_accessory::trace_menu("SCENE unload Hub_AccessoryRoom via ProcSceneManager");
            ProcSceneManager::unload_bind_2(<ProcInst as FromIlInstance>::from_il_instance(this.as_instance()), "Hub_AccessoryRoom");
        }
    }
    call_original!(this, method);
}
#[skyline::hook(offset = 0x2d71ad0)]
fn fee_sortie_native_scene_awake(this: HubAccessoryRoomCamera, method: OptionalMethod) {
    if super::sortie_accessory::is_open() {
        // During additive load Camera.main could otherwise resolve the old map.
        let native = locate_camera();
        if native != 0 {
            NATIVE_CAMERA.store(native, Ordering::Release);
            wrap::<Camera>(native).set_enabled(false);
            with_camera(|| call_original!(this, method));
            AWAKE_DONE.store(true, Ordering::Release);
        }
        CONTROLLER.store(this.as_instance().as_ptr() as usize, Ordering::Release);
        super::sortie_accessory::trace_menu(&format!("SCENE native camera controller Awake camera={:#x}", NATIVE_CAMERA.load(Ordering::Acquire)));
    } else { call_original!(this, method); }
}
#[skyline::hook(offset = 0x2d72060)]
fn fee_sortie_native_scene_late(this: HubAccessoryRoomCamera, method: OptionalMethod) {
    if super::sortie_accessory::is_open() {
        if !ready() || CONTROLLER.load(Ordering::Acquire) != this.as_instance().as_ptr() as usize { return; }
        // The controller has no character until the first asynchronous model completes.
        if this.get_character().is_null() { return; }
    }
    if super::sortie_accessory::is_open() { with_camera(|| call_original!(this, method)); }
    else { call_original!(this, method); }
}
#[repr(C)]
#[derive(Clone, Copy)]
struct Stick { x: f32, y: f32 }
#[skyline::hook(offset = 0x2d74190)]
fn fee_sortie_native_scene_stick(this: HubAccessoryRoomCamera, method: OptionalMethod) -> Stick {
    if super::sortie_accessory::is_open() {
        if !INPUT.load(Ordering::Acquire)
            || INPUT_FRAME.load(Ordering::Acquire) != engage::unity_engine::time::Time::get_frame_count() as usize {
            return Stick { x: 0.0, y: 0.0 };
        }
        let _ = Pad::get_stick_rx(); let _ = Pad::get_stick_ry();
        let (x, y) = super::sortie_accessory::right_stick();
        return Stick { x, y };
    }
    call_original!(this, method)
}
#[skyline::hook(offset = 0x2173c70)]
fn fee_sortie_native_scene_view(view: HubAccessoryRoom_ViewMode, method: OptionalMethod) {
    if super::sortie_accessory::is_open() {
        VIEW.store(view.value as usize, Ordering::Release);
        if ready() { if let Some(c) = controller() { with_camera(|| c.set_view_mode(view)); } }
        super::sortie_accessory::trace_menu(&format!("SCENE view={}", view.value));
    } else { call_original!(view, method); }
}
pub fn supported() -> bool {
    let base = unity::module_base();
    [(0x2d757e0usize, 0xa9bf7bfdu32), (0x2d76340usize, 0xa9bd7bfdu32), (0x2d71ad0usize, 0xa9bc7bfdu32), (0x2d72060usize, 0xa9be7bfdu32), (0x2d74190usize, 0xd10143ffu32), (0x2173c70usize, 0xa9bd7bfdu32), (0x2c3d2b0usize, 0xa9be7bfdu32), (0x281fe60usize, 0xd101c3ffu32), (0x2821130usize, 0xa9bd7bfdu32)].into_iter().all(|(offset, word)| unsafe { *((base + offset) as *const u32) == word })
}

pub fn install() {
    skyline::install_hooks!(fee_sortie_native_scene_load, fee_sortie_native_scene_end,
        fee_sortie_native_scene_awake, fee_sortie_native_scene_late,
        fee_sortie_native_scene_stick, fee_sortie_native_scene_view, fee_sortie_native_main_camera);
}

#[skyline::hook(offset = 0x2c3d2b0)]
fn fee_sortie_native_main_camera(method: OptionalMethod) -> Camera {
    let p = NATIVE_CAMERA.load(Ordering::Acquire);
    if CAMERA_CONTEXT.load(Ordering::Acquire) && p != 0 { wrap(p) }
    else { call_original!(method) }
}
