//! Types and methods to query the state of [`Unit`](crate::gamedata::unit::Unit)s in battle.

use unity::prelude::*;

#[unity::class("Combat", "Character")]
pub struct Character {
    monobehaviour_fields: [u8; 0x8],
    side: i32,
    chain_id: i32,
    prefetch: *const u8,
    effect: *const u8,
    observable: *const u8,
    idle_smb: *const u8,
    fsm: *const u8,
    brain: *const u8,
    game_status: *const u8,
    is_done_setup: bool,
    head_look_at_ik: *const u8,
    body_look_at_ik: *const u8,
    enemy_side: i32,
    ground_level: f32,
    world_hit_dir: [f32; 3],
    rush_dir: *const u8,
    combat_start_fade_disposable: *const u8,
    body_animator: *const u8,
    ride_animator: *const u8,
    face_animator: *const u8,
    play__idle__________: i32,
    ________________: f32,
    constant_speed_playback: bool,
    playing_hash: i32,
    playing_store: *const u8,
    pub playing_event: *const u8,
    play_end_world_pos: *const u8,
    dither_fade: *const u8,
    cached_dither_fade: bool,
    material_engage: *const u8,
    cached_material_engage: bool,
    signal: *const u8,
    lying: *const u8,
    joint: *const u8,
    cached_joint: bool,
    proportion: *const u8,
    cached_proportion: bool,
    config: *const u8,
    // too lazy to do the rest for now
}

// Combat.Character$$get_Side	7102afc5a0	int32_t Combat.Character$$get_Side(Combat_Character_o * __this, MethodInfo * method)	8
#[unity::from_offset("Combat", "Character", "get_Side")]
pub fn character_get_side(this: &Character, method_info: OptionalMethod) -> i32;

#[unity::class("Combat", "CharacterSound")]
pub struct CharacterSound { }

#[unity::class("Combat", "Phase")]
pub struct Phase {
    i_dont_care: [u8; 0x10],
    pub kind: i32,
    pub hit_type: HitType,
    pub detail: Detail,
    pub attack_side: i32,
    pub attack_hash: i32,
    pub damage_hash: i32,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for the type of hit. The combo flags (which are non-power of two) are provided by the game and included here for completeness.
    pub struct HitType: i32 {
        const Critical = 1;
        const Miss = 2;
        const Guard = 4;
        const Hit = 8;
        const Parry = 16;
        const Knockoff = 64;
        const Heal = 128;
        const ChainGuard = 256;
        const DualGuard = 512;
        const HitStop = 268;
        const GuardType = 260;
        const MissType = 82;
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    /// Bitflags for the detail of the hit. The combo flags (which are non-power of two) are provided by the game and included here for completeness.
    pub struct Detail: i32 {
        const FirstAttack = 1;
        const LastAttack = 2;
        const Rush = 4;
        const Efficacy = 8;
        const EngageAttack = 16;
        const Break = 32;
        const Smash = 64;
        const StandingDie = 128;
        const DamageDisplayed = 256;
        const ChainAtk = 4096;
        const ChainAtk2 = 8192;
        const ChainGrd1 = 16384;
        const ChainGrd2 = 32768;
        const ChainGrd3 = 65536;
        const ChainGrd4 = 131072;
        const ChainGrd = 245760;
    }
}

#[repr(C)]
#[derive(Debug)]
/// Used by the game to determine the sound effects to play during damage for zoomed-in combat.
pub enum DamageEffectLevel {
    Low,
    Medium,
    High,
}

#[repr(C)]
#[unity::class("Combat", "MagicSignalProcessor")]
pub struct MagicSignalProcessor {
    monobehaviour_fields: [u8; 0x8],
    pub character: &'static Character,
}

#[repr(C)]
#[derive(Debug)]
/// Describes how the magic projectile will arrive at the target.
pub enum ArrivalType {
    /// The magic projectile will fly to the target, such as fireballs and wind attacks.
    Flying,
    /// The magic projectile will arrive at the target immediately, such as lightning.
    ConstantTime,
}

#[unity::class("Combat", "MagicBulletSettings")]
pub struct MagicBulletSettings {
    home_node_name: &'static Il2CppString,
    target_node_name: &'static Il2CppString,
    float: f32,
    pub arrival_type: ArrivalType,
    move_speed: f32,
}

#[unity::class("Combat", "Magic")]
pub struct Magic<'a> {
    base: [u8; 0x28],
    pub magic_bullet_settings: &'a MagicBulletSettings,
}

#[unity::class("Combat", "MagicSignal")]
pub struct MagicSignal {
    pub level: i32,
    pub frame: f32,
    pub command: i32,
    pub prefab: *const u8,
    pub parent_name: Option<&'static Il2CppString>,
    pub connect: i32,
    pub int_parameter: i32,
    pub float_parameter: f32,
    pub string_parameter: Option<&'static Il2CppString>,
}

// Combat.MagicSignalProcessor$$get_Magic	7101bf31a0	Combat_Magic_o * Combat.MagicSignalProcessor$$get_Magic(Combat_MagicSignalProcessor_o * __this, MethodInfo * method)	8
#[unity::from_offset("Combat", "MagicSignalProcessor", "get_Magic")]
pub fn magicsignalprocessor_get_magic(
    this: &MagicSignalProcessor,
    method_info: OptionalMethod,
) -> &Magic;

#[unity::class("UnityEngine", "AnimationEvent")]
pub struct AnimationEvent { }

#[unity::from_offset("Combat", "Phase", "get_DamageEffectLevel")]
pub fn phase_get_damage_effect_level(
    this: &Phase,
    method_info: OptionalMethod,
) -> DamageEffectLevel;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsEvasion")]
pub fn runtimeanimutil_is_evasion(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsParry")]
pub fn runtimeanimutil_is_parry(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "RuntimeAnimUtil", "IsGuard")]
pub fn runtimeanimutil_is_guard(hash: i32, method_info: OptionalMethod) -> bool;

#[unity::from_offset("Combat", "Phase", "get_IsCritical")]
pub fn phase_get_is_critical(this: &Phase, method_info: OptionalMethod) -> bool;

// Combat.Character$$get_Phase	7102afcb70	Combat_Phase_o * Combat.Character$$get_Phase(Combat_Character_o * __this, MethodInfo * method)	336
#[unity::from_offset("Combat", "Character", "get_Phase")]
pub fn character_get_phase(
    this: &Character,
    method_info: OptionalMethod,
) -> &Phase;

// Combat.CharacterSound$$get_CP	71025efef0	Combat_Character_o * Combat.CharacterSound$$get_CP(Combat_CharacterSound_o * __this, MethodInfo * method)	180
#[unity::from_offset("Combat", "CharacterSound", "get_CP")]
pub fn charactersound_get_cp(
    this: &CharacterSound,
    method_info: OptionalMethod,
) -> &Character;

// Combat.Phase$$get_IsPlayerSideAttack	7101f2b2d0	bool Combat.Phase$$get_IsPlayerSideAttack(Combat_Phase_o * __this, MethodInfo * method)	12
#[unity::from_offset("Combat", "Phase", "get_IsPlayerSideAttack")]
pub fn phase_get_is_player_side_attack(
    this: &Phase,
    method_info: OptionalMethod,
) -> bool;

// Combat.Phase$$get_IsEnemySideAttack	7101f2b2e0	bool Combat.Phase$$get_IsEnemySideAttack(Combat_Phase_o * __this, MethodInfo * method)	12
#[unity::from_offset("Combat", "Phase", "get_IsEnemySideAttack")]
pub fn phase_get_is_enemy_side_attack(
    this: &Phase,
    method_info: OptionalMethod,
) -> bool;

// Combat.Side$$IsMaster	710247cad0	bool Combat.Side$$IsMaster(int32_t i, MethodInfo * method)	12
#[unity::from_offset("Combat", "Side", "IsMaster")]
pub fn side_is_master(i: i32, method_info: OptionalMethod) -> bool;

// Combat.Side$$IsChainAtk	710247cb00	bool Combat.Side$$IsChainAtk(int32_t i, MethodInfo * method)	16
#[unity::from_offset("Combat", "Side", "IsChainAtk")]
pub fn side_is_chain_atk(i: i32, method_info: OptionalMethod) -> bool;
