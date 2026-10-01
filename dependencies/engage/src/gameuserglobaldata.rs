//! Methods to interact with the global savedata variables.

use unity::prelude::*;
use unity::il2cpp::class::Il2CppRGCTXData;
use crate::singleton::SingletonClass;

#[unity::class("App", "GameUserGlobalData")]
pub struct GameUserGlobalData {}

impl GameUserGlobalData {
    pub fn get_instance() -> &'static mut GameUserGlobalData {
        let idk = get_generic_class!(SingletonClass<GameUserGlobalData>).unwrap();
        let pointer = unsafe { &*(idk.rgctx_data as *const Il2CppRGCTXData as *const u8 as *const [&'static MethodInfo; 6]) };
        let get_instance =
            unsafe { std::mem::transmute::<_, extern "C" fn(OptionalMethod) -> &'static mut GameUserGlobalData>(pointer[5].method_ptr) };
        get_instance(Some(&pointer[5]))
    }
    pub fn get_last_save_data_index() -> i32 {
        let instance = Self::get_instance();
        unsafe { gugd_get_last_save_data_index(instance, None) }
    }
    pub fn get_last_save_data_type() -> i32 {
        let instance = Self::get_instance();
        unsafe {  gugd_get_last_save_data_type(instance, None) }
    }
}

#[unity::from_offset("App", "GameUserGlobalData", "IsCompleted")]
pub fn is_completed(this: *const u8, cid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "GameUserGlobalData", "SetCompleted")]
pub fn set_completed(this: *const u8, chapter_data: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App", "GameUserGlobalData", "ClearCompleted")]
pub fn clear_completed(this: *const u8, chapter_data: *const u8, method_info: OptionalMethod);

#[unity::from_offset("App","GameUserGlobalData","get_LastSaveDataIndex")]
fn gugd_get_last_save_data_index(this: &GameUserGlobalData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App","GameUserGlobalData","get_LastSaveDataType")]
fn gugd_get_last_save_data_type(this: &GameUserGlobalData, method_info: OptionalMethod) -> i32;
