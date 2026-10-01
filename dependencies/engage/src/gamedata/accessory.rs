use unity::prelude::*;

use super::{Gamedata, StructBaseFields};

#[unity::class("App", "AccessoryData")]
pub struct AccessoryData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    pub help: &'static Il2CppString,
    pub name_m: &'static Il2CppString,
    pub help_m: &'static Il2CppString,
    pub name_f: &'static Il2CppString,
    pub help_f: &'static Il2CppString,
    pub first: bool,
    pub amiibo: bool,
    pub condition_cid: &'static Il2CppString,
    pub condition_gender: i32,
    pub condition_skills: &'static [Il2CppString; 0],
    pub gid: &'static Il2CppString,
    pub asset: &'static Il2CppString,
    pub price: i32,
    pub iron: i32,
    pub steel: i32,
    pub silver: i32,
    pub mask: i32,
    pub kind: i32,
    pub god_data: u64,
    pub flag_name: &'static Il2CppString,
    
    // ...
}

impl Gamedata for AccessoryData { }