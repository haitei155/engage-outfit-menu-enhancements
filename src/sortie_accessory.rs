//! Add the native accessory shop to Cobalt's sortie shop submenu.
use engage::app::{
    accessorydata::{AccessoryData, IAccessoryDataMethods},
    backgroundmanager::BackgroundManager,
    basicmenu::{BasicMenu, IBasicMenu},
    basicmenuitem::{BasicMenuItem, IBasicMenuItemMethods},
    force::{Force, Force_Type, IForceMethods},
    hubaccessoryshopsequence::{HubAccessoryShopSequence, IHubAccessoryShopSequenceMethods},
    language::Language,
    procinst::{IProcInstMethods, ProcInst},
    pad::Pad,
    sortietopmenu::SortieTopMenu_ShopMenuItem,
    unit::{Unit, IUnit, IUnitMethods},
    unitaccessorylist::IUnitAccessoryListMethods,
    assettable::{AssetTable_Result, IAssetTable_ResultMethods},
};
use engage::combat::{
    character::{Character, ICharacterMethods},
    characterappearance::CharacterAppearance,
    characterfactoryasync::CharacterFactoryAsync,
};
use engage::root_motion::final_ik::{fullbodybipedik::FullBodyBipedIK, grounderfbbik::GrounderFBBIK};
use engage::{BasicMenuExt, BasicMenuItemAttribute, BasicMenuItemExt, BasicMenuItemMethods, BasicMenuResult, List_1Ext};
use engage_legacy::titlebar::TitleBar;
use engage::system::collections::generic::list_1::IList_1Methods;
use engage::system::{action::Action, object::Object};
use engage::app::hubaccessoryroomcamera::IHubAccessoryRoomCameraMethods;
use engage::app::unitactor::IUnitActorMethods;
use engage::unity_engine::{
    component::IComponentMethods, behaviour::{Behaviour, IBehaviourMethods},
    time::Time, animator::IAnimatorMethods, gameobject::{GameObject, IGameObjectMethods},
    object_2::Object_2, transform::ITransformMethods, vector3::Vector3,
};
use std::io::Write;
use std::sync::{atomic::{AtomicBool, AtomicUsize, Ordering}, Mutex};
use unity::{Cast, FromIlInstance, Il2CppString, IlNull, OptionalMethod, SystemObject, SystemType};

struct AccessoryShopItem;
static SORTIE_ROOM_OPEN: AtomicBool = AtomicBool::new(false);
static PREVIEW_STAGE: AtomicUsize = AtomicUsize::new(0);
static PREVIEW_CHARACTER: AtomicUsize = AtomicUsize::new(0);
// Unit and AccessoryData belong to the game's force/data tables. A request
// keeps no temporary managed objects alive across frames.
#[derive(Clone, Copy)]
struct PreviewRequest {
    unit: usize,
    accessory: usize,
    slots: [i32; 8],
    frame: i32,
    serial: usize,
}
#[derive(Clone, Copy)]
struct LoadingPreview { character: usize, unit: usize, hash: i32, serial: usize }
static PREVIEW_REQUEST: Mutex<Option<PreviewRequest>> = Mutex::new(None);
static PREVIEW_LOADING: Mutex<Option<LoadingPreview>> = Mutex::new(None);
static PREVIEW_SELECTION: Mutex<(usize, i32)> = Mutex::new((0, 0));
static PREVIEW_SERIAL: AtomicUsize = AtomicUsize::new(0);
static PREVIEW_STARTED: AtomicUsize = AtomicUsize::new(0);
static PREVIEW_READY: AtomicUsize = AtomicUsize::new(0);
static PREVIEW_TICK: AtomicUsize = AtomicUsize::new(usize::MAX);
static BACKGROUND_CAPTURE_OWNED: AtomicBool = AtomicBool::new(false);
static PREVIEW_YAW: Mutex<f32> = Mutex::new(0.0);
static PREVIEW_ZOOM: Mutex<(f32, f32)> = Mutex::new((2.4, 0.0));
static PREVIEW_ROTATION_SPEED: Mutex<f32> = Mutex::new(0.0);
static RIGHT_X: AtomicUsize = AtomicUsize::new(0);
static RIGHT_Y: AtomicUsize = AtomicUsize::new(0);
static MAP_BODY_BEFORE: Mutex<Vec<(usize, [i32; 7])>> = Mutex::new(Vec::new());

fn map_slots(unit: engage::app::unit::Unit) -> [i32; 7] {
    let list = unit.get_accessory_list();
    if list.is_null() { return [0; 7]; }
    [0, 1, 2, 3, 5, 6, 7].map(|kind| {
        let item = list.get_item(kind);
        if item.is_null() { 0 } else { unity::field_get_value_at_offset(item, 16) }
    })
}

fn snapshot_map_equipment() {
    let mut before = MAP_BODY_BEFORE.lock().unwrap();
    before.clear();
    let force = Force::get(Force_Type::player());
    if force.is_null() { return; }
    let mut unit = force.get_first();
    for _ in 0..64 {
        if unit.is_null() { break; }
        if !unit.get_actor().is_null() {
            before.push((unit.as_instance().as_ptr() as usize, map_slots(unit)));
        }
        unit = unit.get_next();
    }
}

pub(crate) fn trace_menu(message: &str) {
    if !crate::debug_enabled() { return; }
    let line = format!("[FEE Sortie Outfit] {message}\n");
    let _ = horizon_svc::output_debug_string(&line);
    if let Ok(mut file) = std::fs::OpenOptions::new().create(true).append(true).open("sd:/engage/fee-outfit-menu-debug.log") {
        let _ = file.write_all(line.as_bytes());
    }
}

impl BasicMenuItemMethods for AccessoryShopItem {
    extern "C" fn get_name(_: BasicMenuItem, _: OptionalMethod) -> Il2CppString {
        match Language::get_lang().value {
            10 => "装饰品店",
            9 => "裝飾品店",
            0 => "アクセサリー屋",
            _ => "Accessory Shop",
        }.into()
    }

    extern "C" fn a_call(this: BasicMenuItem, _: OptionalMethod) -> BasicMenuResult {
        let menu = this.get_menu();
        menu.close_anime_all();
        let menu_proc: ProcInst = menu.into();
        snapshot_map_equipment();
        SORTIE_ROOM_OPEN.store(true, Ordering::Release);
        trace_menu("BEGIN build=native-fade-v8 sortie accessory shop");
        let seq = HubAccessoryShopSequence::new();
        let descs = crate::sortie_transition::descriptors(seq.create_desc());
        let seq_proc: ProcInst = seq.into();
        seq_proc.create_bind(menu_proc, descs, "HubAccessoryShopSequence");
        seq.override_class().override_virtual_method("OnDispose", unity::method_info!(shop_on_dispose, 0));
        BasicMenuResult::se_decide()
    }

    extern "C" fn build_attribute(_: BasicMenuItem, _: OptionalMethod) -> BasicMenuItemAttribute {
        BasicMenuItemAttribute::enable()
    }
}

// Compare all seven visible categories, including face and InfoAnim.
pub(crate) fn refresh_changed_map_actors() {
    let force = Force::get(Force_Type::player());
    let mut refreshed = 0;
    if !force.is_null() {
        let mut before = MAP_BODY_BEFORE.lock().unwrap();
        let mut unit = force.get_first();
        for _ in 0..64 {
            if unit.is_null() { break; }
            let next = unit.get_next();
            let key = unit.as_instance().as_ptr() as usize;
            if !unit.get_actor().is_null() {
                if let Some((_, old_slots)) = before.iter().find(|(ptr, _)| *ptr == key) {
                    let slots = map_slots(unit);
                    if *old_slots != slots {
                        trace_menu(&format!("REFRESH unit={key:#x} slots_before={old_slots:?} slots_after={slots:?}"));
                        unit.reload_actor();
                        // ReloadActor queues a status flag; process it under black
                        // before waiting so the outer map Proc need not resume.
                        unit.get_actor().update_status();
                        crate::sortie_transition::track_changed_unit(unit);
                        refreshed += 1;
                    }
                }
            }
            unit = next;
        }
        before.clear();
    }
    trace_menu(&format!("reloaded {refreshed} sortie actors after accessory shop (all 7 categories)"));
}
fn shop_on_dispose(this: ProcInst, _: OptionalMethod) {
    clear_preview();
    crate::native_accessory_scene::restore();
    if !SORTIE_ROOM_OPEN.swap(false, Ordering::AcqRel) { return; }
    // Normally already refreshed under black in EndSequence; fallback only.
    let needs_refresh = !MAP_BODY_BEFORE.lock().unwrap().is_empty();
    if needs_refresh { refresh_changed_map_actors(); }
    TitleBar::close_header();
    let parent = this.get_super();
    if parent.is_null() { return; }
    if let Some(slot) = parent.get_class().raw().get_virtual_method("OpenAnimeAll") {
        let open: extern "C" fn(ProcInst, &'static unity::MethodInfo) = unsafe { std::mem::transmute(slot.method_ptr) };
        open(parent, slot.method_info);
    }
}

// The Somniel shop uses GetForAccessory -> CreateFromResult(1) ->
// CharacterFactoryAsync.CreateForTalk. Keep that full-body character in a
// original additive accessory scene, rendered by its full-screen native camera.
pub(crate) fn is_open() -> bool { SORTIE_ROOM_OPEN.load(Ordering::Acquire) }
pub(crate) fn right_stick() -> (f32, f32) {
    (f32::from_bits(RIGHT_X.load(Ordering::Acquire) as u32), f32::from_bits(RIGHT_Y.load(Ordering::Acquire) as u32))
}
pub(crate) fn capture_transition() {
    if !BackgroundManager::is_captured() {
        BackgroundManager::set_capture_2(true);
        BACKGROUND_CAPTURE_OWNED.store(true, Ordering::Release);
    }
}
pub(crate) fn end_capture_transition() {
    if BACKGROUND_CAPTURE_OWNED.swap(false, Ordering::AcqRel) { BackgroundManager::set_capture_2(false); }
}
pub(crate) fn release_native_preview() { clear_preview(); }
fn clear_preview() {
    if let Some(c) = crate::native_accessory_scene::controller() { c.set_character(Character::null()); }
    *PREVIEW_REQUEST.lock().unwrap() = None;
    *PREVIEW_LOADING.lock().unwrap() = None;
    PREVIEW_STARTED.store(0, Ordering::Release);
    PREVIEW_READY.store(0, Ordering::Release);
    PREVIEW_TICK.store(usize::MAX, Ordering::Release);
    PREVIEW_CHARACTER.store(0, Ordering::Release);
    *PREVIEW_SELECTION.lock().unwrap() = (0, 0);
    *PREVIEW_YAW.lock().unwrap() = 0.0;
    *PREVIEW_ZOOM.lock().unwrap() = (2.4, 0.0);
    *PREVIEW_ROTATION_SPEED.lock().unwrap() = 0.0;
    let stage = PREVIEW_STAGE.swap(0, Ordering::AcqRel);
    if stage != 0 {
        Object_2::destroy_2(<GameObject as FromIlInstance>::from_il_instance(
            unity::IlInstance::from_raw(stage as *mut ())));
    }
    if BACKGROUND_CAPTURE_OWNED.swap(false, Ordering::AcqRel) {
        BackgroundManager::set_capture_2(false);
    }
}

fn ensure_preview_stage() -> bool {
    if !crate::native_accessory_scene::activate() { return false; }
    end_capture_transition();
    if PREVIEW_STAGE.load(Ordering::Acquire) == 0 {
        let stage = GameObject::new("FEE Native Shop Characters".into());
        PREVIEW_STAGE.store(stage.as_instance().as_ptr() as usize, Ordering::Release);
    }
    true
}

fn request_preview(unit: Unit, accessory: AccessoryData, delay: bool) {
    if unit.is_null() { clear_preview(); return; }
    crate::native_accessory_scene::target(accessory);
    trace_menu(&format!("REQUEST unit={:p} accessory={:p} kind={} delay={delay}", unit.as_instance().as_ptr(), accessory.as_instance().as_ptr(), if accessory.is_null() { -1 } else { accessory.get_kind().value }));
    let list = unit.get_accessory_list();
    let slots = std::array::from_fn(|kind| {
        if list.is_null() || kind >= list.get_count() as usize { return 0; }
        let item = list.get_item(kind as i32);
        if item.is_null() { 0 } else { unity::field_get_value_at_offset(item, 16) }
    });
    let unit = unit.as_instance().as_ptr() as usize;
    let accessory = accessory.as_instance().as_ptr() as usize;
    let mut request = PREVIEW_REQUEST.lock().unwrap();
    if request.as_ref().is_some_and(|r| (r.unit, r.accessory, r.slots) == (unit, accessory, slots)) { return; }
    *request = Some(PreviewRequest { unit, accessory, slots,
        frame: Time::get_frame_count() - if delay { 0 } else { 5 },
        serial: PREVIEW_SERIAL.fetch_add(1, Ordering::AcqRel) + 1 });
}

fn tick_preview_loading() {
    let request = *PREVIEW_REQUEST.lock().unwrap();
    let loading = *PREVIEW_LOADING.lock().unwrap();
    if let Some(loading) = loading {
        let character = <Character as FromIlInstance>::from_il_instance(
            unity::IlInstance::from_raw(loading.character as *mut ()));
        if PREVIEW_READY.load(Ordering::Acquire) != loading.character { return; }
        PREVIEW_READY.store(0, Ordering::Release);
        *PREVIEW_LOADING.lock().unwrap() = None;
        if request.is_some_and(|r| r.serial == loading.serial) {
            // HubAccessoryRoom.<LoadCharacter>b__0 does SetupForHub, resets
            // the completed model's position/rotation, then makes it visible.
            // Its two GetComponentsInChildren specializations are GrounderFBBIK
            // and FullBodyBipedIK. Leaving them enabled makes a shop model seek
            // ground beneath the model and visibly shift its pose.
            for kind in [SystemType::of::<GrounderFBBIK>(), SystemType::of::<FullBodyBipedIK>()].into_iter().flatten() {
                let components = character.get_game_object().get_components_in_children(kind);
                if !components.is_null() {
                    for index in 0..components.len() {
                        let component = components.get(index);
                        if !component.is_null() {
                            <Behaviour as FromIlInstance>::from_il_instance(component.as_instance()).set_enabled(false);
                        }
                    }
                }
            }
            character.setup_for_hub();
            let face = character.get_face_animator();
            if !face.is_null() {
                let layer = face.get_layer_index("Base Layer");
                if layer >= 0 { face.play_in_fixed_time_3("Normal", layer, 1.0); }
            }
            character.get_transform().set_local_position(Vector3 { x: 0.0, y: 0.0, z: 0.0 });
            character.get_transform().set_local_euler_angles_2(Vector3 { x: 0.0, y: 0.0, z: 0.0 });
            if let Some(c) = crate::native_accessory_scene::controller() {
                let unit = <Unit as FromIlInstance>::from_il_instance(unity::IlInstance::from_raw(loading.unit as *mut ()));
                crate::native_accessory_scene::with_camera(|| c.set_character_2(character, unit.get_pid()));
            }
            let previous = PREVIEW_CHARACTER.swap(loading.character, Ordering::AcqRel);
            character.set_is_visible(true);
            if previous != 0 {
                let old = <Character as FromIlInstance>::from_il_instance(
                    unity::IlInstance::from_raw(previous as *mut ()));
                old.set_is_visible(false);
                Object_2::destroy_2(old.get_game_object());
            }
            *PREVIEW_SELECTION.lock().unwrap() = (loading.unit, loading.hash);
            crate::sortie_transition::model_ready();
            trace_menu(&format!("DISPLAY character={:#x} serial={} hash={} previous={previous:#x}", loading.character, loading.serial, loading.hash));
        } else {
            Object_2::destroy_2(character.get_game_object());
            trace_menu("discarded obsolete hidden model");
        }
    }
    let Some(request) = request else { return; };
    if PREVIEW_STARTED.load(Ordering::Acquire) == request.serial
        || Time::get_frame_count() - request.frame < 5 { return; }
    if !ensure_preview_stage() { return; }
    let unit = <Unit as FromIlInstance>::from_il_instance(unity::IlInstance::from_raw(request.unit as *mut ()));
    // Exact non-amiibo SetUnit path: Unit(true), CopyFrom, retain force, then
    // add the hovered accessory only on the temporary copy (never the save).
    let Some(copy) = Unit::instantiate() else { return; };
    copy.ctor(true);
    copy.copy_from(unit);
    copy.set_m_force(unit.m_force());
    let accessory = <AccessoryData as FromIlInstance>::from_il_instance(
        unity::IlInstance::from_raw(request.accessory as *mut ()));
    if !accessory.is_null() && !copy.get_accessory_list().is_exist(accessory) {
        copy.get_accessory_list().add(accessory, accessory.get_kind());
    }
    let result = AssetTable_Result::get_for_accessory(copy);
    if result.is_null() { return; }
    let hash = result.get_hash_code();
    if *PREVIEW_SELECTION.lock().unwrap() == (request.unit, hash) {
        PREVIEW_STARTED.store(request.serial, Ordering::Release);
        return;
    }
    let appearance = CharacterAppearance::create_from_result(result, 1);
    if appearance.is_null() { return; }
    let stage = <GameObject as FromIlInstance>::from_il_instance(
        unity::IlInstance::from_raw(PREVIEW_STAGE.load(Ordering::Acquire) as *mut ()));
    // true is the native room's invisible flag, not an animation setting.
    let character = CharacterFactoryAsync::create_for_talk(appearance, stage.get_transform(), true);
    if character.is_null() { return; }
    *PREVIEW_LOADING.lock().unwrap() = Some(LoadingPreview {
        character: character.as_instance().as_ptr() as usize, unit: request.unit, hash, serial: request.serial });
    PREVIEW_STARTED.store(request.serial, Ordering::Release);
    // IsSetupDone is written by SetupForHub/SetupForTalk, not by the builder.
    // Subscribe to the actual builder observable just like HubAccessoryRoom.
    let action = Action::new(
        <Object as FromIlInstance>::from_il_instance(character.as_instance()),
        unity::method_info!(outfit_preview_setup_done, 0).into());
    character.call_on_setup_done(action);
    trace_menu(&format!("LOADING character={:p} serial={} hash={hash}; previous model retained", character.as_instance().as_ptr(), request.serial));
}

extern "C" fn outfit_preview_setup_done(character: Character, _: OptionalMethod) {
    if !SORTIE_ROOM_OPEN.load(Ordering::Acquire) { return; }
    let ptr = character.as_instance().as_ptr() as usize;
    trace_menu(&format!("COMPLETE builder character={ptr:#x}"));
    if PREVIEW_LOADING.lock().unwrap().is_some_and(|p| p.character == ptr) {
        PREVIEW_READY.store(ptr, Ordering::Release);
    }
}

pub fn tick_preview_input(menu: BasicMenu) {
    if !SORTIE_ROOM_OPEN.load(Ordering::Acquire) { return; }
    crate::native_accessory_scene::input(!menu.is_null() && matches!(menu.get_class().name().as_str(), "AccessoryShopBuyMenu" | "AccessoryShopChangeMenu"));
    let frame = Time::get_frame_count() as usize;
    if PREVIEW_TICK.swap(frame, Ordering::AcqRel) == frame { return; }
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire)
        && PREVIEW_REQUEST.lock().unwrap().is_none()
        && !menu.is_null()
        && menu.get_class().name() == "AccessoryShopTopMenu" {
        let force = Force::get(Force_Type::player());
        if !force.is_null() {
            let unit = force.get_first();
            if !unit.is_null() { request_preview(unit, AccessoryData::null(), false); }
        }
    }
    tick_preview_loading();
    if !SORTIE_ROOM_OPEN.load(Ordering::Acquire) || PREVIEW_CHARACTER.load(Ordering::Acquire) == 0 { return; }
    // The native room shows its portrait on the action and unit pages, but
    // accepts camera input only while the actual clothing list is active.
    if menu.is_null() || !matches!(menu.get_class().name().as_str(),
        "AccessoryShopBuyMenu" | "AccessoryShopChangeMenu") {
        *PREVIEW_ROTATION_SPEED.lock().unwrap() = 0.0;
        PREVIEW_ZOOM.lock().unwrap().1 = 0.0;
        return;
    }
    crate::native_accessory_scene::input(true);
    if frame % 60 == 0 { crate::native_accessory_scene::trace_camera(); }
}

#[skyline::hook(offset = 0x1f23540)]
fn preview_stick_rx(method: OptionalMethod) -> f32 {
    let value = call_original!(method);
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire) {
        RIGHT_X.store(value.to_bits() as usize, Ordering::Release);
        0.0
    } else { value }
}

#[skyline::hook(offset = 0x1f23610)]
fn preview_stick_ry(method: OptionalMethod) -> f32 {
    let value = call_original!(method);
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire) {
        RIGHT_Y.store(value.to_bits() as usize, Ordering::Release);
        0.0
    } else { value }
}

#[skyline::hook(offset = 0x1f239c0)]
fn preview_stick_rx_allowance(allowance: f32, method: OptionalMethod) -> f32 {
    let value = call_original!(allowance, method);
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire) {
        RIGHT_X.store(value.to_bits() as usize, Ordering::Release);
        0.0
    } else { value }
}

#[skyline::hook(offset = 0x1f23b30)]
fn preview_stick_ry_allowance(allowance: f32, method: OptionalMethod) -> f32 {
    let value = call_original!(allowance, method);
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire) {
        RIGHT_Y.store(value.to_bits() as usize, Ordering::Release);
        0.0
    } else { value }
}

// All native selection, hover, confirm, cancel and unit-switch paths already
// converge here. Intercept only the sortie session; Somniel stays native.
#[skyline::hook(offset = 0x21737a0)]
fn outfit_sortie_set_unit(unit: Unit, accessory: AccessoryData, delay: bool, amiibo: bool, method: OptionalMethod) {
    if SORTIE_ROOM_OPEN.load(Ordering::Acquire) {
        request_preview(unit, accessory, delay);
    } else {
        call_original!(unit, accessory, delay, amiibo, method);
    }
}

extern "C" fn help_text(_: BasicMenuItem, _: OptionalMethod) -> Il2CppString {
    match Language::get_lang().value {
        10 => "打开装饰品店，为同伴更换服饰。",
        9 => "開啟裝飾品店，為同伴更換服飾。",
        0 => "アクセサリー屋で仲間の衣装を変更します。",
        _ => "Open the accessory shop to change your allies' outfits.",
    }.into()
}

// The top-level shop button creates Cobalt's expanded submenu in ACall. Hook
// that button, rather than Cobalt's submenu CreateBind, and inspect its child
// after the original ACall returns. This places the new row after forge.
fn add_accessory_shop(menu: BasicMenu) {
    if menu.get_class().name() != "SortieTopMenuShopSubMenu" { return; }
    let full = menu.m_full_menu_item_list();
    if full.is_null() { return; }
    let count = full.count();
    if crate::debug_enabled() {
        trace_menu(&format!("shop after ACall rows={count} classes={}",
            (0..count).map(|i| full.get(i).get_class().name()).collect::<Vec<_>>().join(" | ")));
    }
    if count == 4 {
        let item = BasicMenuItem::new_impl::<AccessoryShopItem>();
        let class = item.get_class();
        class.override_virtual_method("GetName", unity::method_info!(AccessoryShopItem::get_name, 0));
        class.override_virtual_method("GetHelpText", unity::method_info!(help_text, 0));
        class.override_virtual_method("ACall", unity::method_info!(AccessoryShopItem::a_call, 0));
        class.override_virtual_method("BuildAttribute", unity::method_info!(AccessoryShopItem::build_attribute, 0));
        full.insert(3, item);
        menu.set_m_reserved_show_row_num(full.count());
        trace_menu("inserted below forge row");
    } else if count != 5 {
        trace_menu(&format!("shop submenu has {count} rows; outfit item skipped"));
    }
}

#[unity::hook("App", "SortieTopMenu.ShopMenuItem", "ACall", 0)]
fn sortie_shop_a_call(this: SortieTopMenu_ShopMenuItem, method: OptionalMethod) -> BasicMenuResult {
    let result = call_original!(this, method);
    let parent: ProcInst = this.get_menu().into();
    let child = parent.get_child();
    if !child.is_null() {
        let menu = <BasicMenu as FromIlInstance>::from_il_instance(child.as_instance());
        add_accessory_shop(menu);
    } else {
        trace_menu("shop ACall returned no child menu");
    }
    result
}

pub fn install() {
    skyline::install_hooks!(sortie_shop_a_call, outfit_sortie_set_unit,
        preview_stick_rx, preview_stick_ry,
        preview_stick_rx_allowance, preview_stick_ry_allowance);
}
