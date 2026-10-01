use unity::prelude::*;
use crate::proc::{ProcInst, Bindable};
use super::access::HubAccessManager;

#[unity::class("App", "HubSequence")]
pub struct HubSequence {
    proc: ProcInst,
    is_resume: bool,
    m_is_loaded: bool,
    //
}
impl Bindable for HubSequence {}

#[unity::class("App", "HubMiniMap")]
pub struct HubMiniMap {}


impl HubSequence {
    pub fn get_instance() -> &'static Self {
        let method = HubSequence::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("get_Instance")));
        let get_instance = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> &'static HubSequence>( method.unwrap().method_ptr, ) };
        get_instance(method.unwrap())
    }
    pub fn is_exist_instance() -> bool { unsafe { hub_is_exist_instance(None) }}
    pub fn get_mini_map(&self) -> &HubMiniMap { unsafe { hub_get_mini_map(self, None)}}
    
    pub fn get_current_access_data(&self) -> &HubAccessManager { unsafe { hub_get_access_data(self, None)}}
}

impl HubMiniMap {
    pub fn hide(&self) { unsafe { hub_mini_map_hide(self, None); }}
    pub fn hide_system_menu(&self) { unsafe { hub_mini_map_system_hide(self, None); } }
    pub fn is_show(&self) -> bool { unsafe { hub_mini_map_is_show(self, None)}}
    pub fn set_mode(&self, mode: i32) { unsafe { hub_mini_map_set_mode(self, mode, None); }}
}


#[unity::from_offset("App", "HubSequence", "get_MiniMap")]
fn hub_get_mini_map(this: &HubSequence, method_info: OptionalMethod) -> &HubMiniMap;

#[unity::from_offset("App", "HubSequence", "get_CurrentAccessData")]
fn hub_get_access_data(this: &HubSequence, method_info: OptionalMethod) -> &HubAccessManager;


#[unity::from_offset("App", "HubSequence", "IsExistInstance")]
fn hub_is_exist_instance(method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "HubMiniMap", "HideSystemMenu")]
fn hub_mini_map_system_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "SetMode")]
fn hub_mini_map_set_mode(this: &HubMiniMap, mode: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "Hide")]
fn hub_mini_map_hide(this: &HubMiniMap, method_info: OptionalMethod);

#[unity::from_offset("App", "HubMiniMap", "IsShow")]
fn hub_mini_map_is_show(this: &HubMiniMap, method_info: OptionalMethod) -> bool;