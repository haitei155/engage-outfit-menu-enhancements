//! Methods and wrappers to manipulate the settings of the player.

use crate::{singleton::SingletonClass, gamevariable::GameVariable};
use crate::gamedata::dispos::ChapterData;
use unity::{il2cpp::class::Il2CppRGCTXData, prelude::*};

#[repr(C)]
#[unity::class("App", "GameUserData")]
pub struct GameUserData {}

#[unity::from_offset("App", "GameUserData", "get_Variable")]
fn get_variable(this: &GameUserData, method_info: OptionalMethod) -> &'static mut GameVariable;

#[unity::from_offset("App", "GameUserData", "SetGameMode")]
fn set_game_mode(this: &GameUserData, game_mode: GameMode, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "GetGameMode")]
fn get_game_mode(this: &GameUserData, method_info: OptionalMethod) -> GameMode;



#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Casual,
    Classic,
    Phoenix
}

impl From<i32> for GameMode {
    fn from(value: i32) -> Self {
        match value {
            0 => GameMode::Casual,
            1 => GameMode::Classic,
            2 => GameMode::Phoenix,
            _ => GameMode::Casual,
        }
    }
}

impl GameUserData {
    pub fn get_instance() -> &'static mut GameUserData {
        let idk = get_generic_class!(SingletonClass<GameUserData>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut GameUserData>(pointer[5].method_ptr) };
            
        get_instance(Some(&pointer[5]))
    }
    pub fn get_variable() -> &'static mut GameVariable {
        let instance = Self::get_instance();
        unsafe { get_variable(instance, None) }
    }

    pub fn get_game_mode() -> GameMode {
        let instance = Self::get_instance();
        unsafe { get_game_mode(instance, None) }
    }

    pub fn set_game_mode(game_mode: GameMode) {
        let instance = Self::get_instance();
        unsafe { set_game_mode(instance, game_mode, None) }
    }
    pub fn get_difficulty(is_dynamic: bool) -> i32 {
        let instance = Self::get_instance();
        unsafe { get_difficulty(instance, is_dynamic, None) }
    }
    pub fn get_grow_mode() -> i32 {
        let instance = Self::get_instance();
        unsafe { getgrowmode(instance, None)}
    }

    pub fn get_sequence() -> i32 {
        let instance = Self::get_instance();
        unsafe { game_user_data_get_sequence(instance, None) }
    }

    pub fn add_iron(amount: i32) -> i32 {
        let instance = Self::get_instance();
        unsafe {
            let new_amount = get_iron(instance, None) + amount;
            game_user_data_set_iron(instance, new_amount, None);
            new_amount
        }
    }

    pub fn add_steel(amount: i32) -> i32 {
        let instance = Self::get_instance();
        unsafe {
            let new_amount = get_steel(instance, None) + amount;
            game_user_data_set_steel(instance, new_amount, None);
            new_amount
        }
    }

    pub fn add_silver(amount: i32) -> i32 {
        let instance = Self::get_instance();
        unsafe {
            let new_amount = get_steel(instance, None) + amount;
            game_user_data_set_silver(instance, new_amount, None);
            new_amount
        }
    }

    pub fn add_bond(amount: i32) -> i32 {
        let instance = Self::get_instance();
        unsafe {
            let new_amount = get_piece_of_bond(instance, None) + amount;
           set_piece_of_bond(instance, new_amount, None);
            new_amount
        }
    }
    pub fn get_chapter() -> &'static ChapterData { unsafe { get_chapter_data(Self::get_instance(), None)}}
    pub fn get_status() -> &'static mut GameUserDataStatus { unsafe { get_game_user_data_status(Self::get_instance(), None) } }

    pub fn set_gold(monies: i32) { unsafe { setgold(Self::get_instance(), monies, None );} }
    pub fn set_iron(amount: i32){ unsafe { game_user_data_set_iron(Self::get_instance(), amount, None); } }
    pub fn set_steel(amount: i32){ unsafe { game_user_data_set_steel(Self::get_instance(), amount, None); } }
    pub fn set_silver(amount: i32){ unsafe { game_user_data_set_silver(Self::get_instance(), amount, None); } }
    pub fn set_bond(amount: i32){ unsafe {set_piece_of_bond(Self::get_instance(), amount, None); } }

    pub fn is_encount_map() -> bool { unsafe { is_encounter_map(Self::get_instance(), None)}}
    pub fn is_cid_completed(cid: &Il2CppString) -> bool { unsafe{ is_completed(Self::get_instance(), cid, None) }}
    pub fn is_chapter_completed(chapter: &ChapterData) -> bool { unsafe {is_completed_chapterdata(Self::get_instance(), chapter, None) }}
    pub fn is_evil_map() -> bool { unsafe { is_evil_map(Self::get_instance(), None) }}
}


// GameUserData Status Flags
#[unity::class("App", "GameUserDataStatus")]
pub struct GameUserDataStatus {
     pub value : i32, 
}

#[unity::from_offset("App", "GameUserData", "GetDifficulty")]
fn get_difficulty(this: &GameUserData, is_dynamic: bool, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "GetGrowMode")]
fn getgrowmode(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "get_RefineIron")]
fn get_iron(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "get_RefineSilver")]
fn get_silver(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "get_RefineSteel")]
fn get_steel(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "set_RefineIron")]
fn game_user_data_set_iron(this: &GameUserData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "set_RefineSilver")]
fn game_user_data_set_silver(this: &GameUserData, value: i32, method_info: OptionalMethod); 

#[unity::from_offset("App", "GameUserData", "set_RefineSteel")]
fn game_user_data_set_steel(this: &GameUserData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "get_PieceOfBond")]
fn get_piece_of_bond(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "set_PieceOfBond")]
fn set_piece_of_bond(this: &GameUserData, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserData", "get_Sequence")]
fn game_user_data_get_sequence(this: &GameUserData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "GameUserData", "get_Status")]
fn get_game_user_data_status(this: &GameUserData, method_info: OptionalMethod) -> &'static mut GameUserDataStatus;

#[skyline::from_offset(0x0250e450)]
fn setgold(this: &GameUserData, value : i32, method_info: OptionalMethod);

#[skyline::from_offset(0x02515fa0)]
fn is_encounter_map(this: &GameUserData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02515b90)]
fn is_completed(this: &GameUserData, cid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x2515ca0)]
fn is_completed_chapterdata(this: &GameUserData, chapter: &ChapterData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "GameUserData", "get_Chapter")]
fn get_chapter_data(this: &GameUserData, method_info: OptionalMethod) -> &ChapterData;

#[unity::from_offset("App", "GameUserData", "IsEvilMap")]
fn is_evil_map(this: &GameUserData, method_info: OptionalMethod) -> bool;
