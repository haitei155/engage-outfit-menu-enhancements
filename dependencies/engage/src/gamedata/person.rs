pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use super::{JobData, PersonData, skill::SkillArray, WeaponMask};
use std::ops::Deref;
use std::ops::DerefMut;
// Structs, methods required for PersonData, JobData, SkillArray

#[unity::class("App", "Capability")]
pub struct Capability { 
    pub data: &'static mut Array<u8>, 
}
impl Capability {
    pub fn is_zero(&self) -> bool { unsafe { capability_is_zero(self, None)} }
    pub fn add(&self, index: i32, value: u8) { unsafe { capability_add(self, index, value, None); }}
}

impl Deref for CapabilityFields {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) }
    }
}
impl DerefMut for CapabilityFields {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) }
    }
}

#[unity::class("App", "CapabilitySbyte")]
pub struct CapabilitySbyte {
     pub data: &'static mut Array<i8>, 
}

impl CapabilitySbyte {
    pub fn is_zero(&self) -> bool { unsafe { capabilitysbyte_is_zero(self, None)} }
    pub fn add(&self, index: i32, value: i8) { unsafe { capabilitysbyte_add(self, index, value, None); }}
}
impl Deref for CapabilitySbyteFields {
    type Target = [i8];
    fn deref(&self) -> &Self::Target {
        unsafe { std::slice::from_raw_parts(self.data.m_items.as_ptr(), self.data.max_length) }
    }
}
impl DerefMut for CapabilitySbyteFields {
    fn deref_mut(&mut self) -> &mut [i8] {
        unsafe { std::slice::from_raw_parts_mut(self.data.m_items.as_mut_ptr(), self.data.max_length) }
    }
}

#[unity::class("App", "PersonDataFlag")]
pub struct PersonDataFlag { 
    pub value: i32,
}

impl PersonData {
    // Getters
    pub fn get_aptitude(&self) -> &'static WeaponMask { unsafe {person_get_apt(self, None)}}
    pub fn get_sub_aptitude(&self) -> &'static WeaponMask { unsafe { person_get_sub_apt(self, None)}}
    pub fn get_ascii_name(&self) -> &Il2CppString { unsafe {person_get_ascii_name(self, None) } }
    pub fn get_asset_force(&self) -> i32 { unsafe { person_get_asset_force(self, None) }  }
    pub fn get_attrs(&self) -> i32 { unsafe { person_get_attrs(self, None)} }
    pub fn get_combat_bgm(&self) -> &Il2CppString { unsafe { person_get_combat_bgm(self, None)}}
    pub fn get_common_skills(&self) -> &SkillArray { unsafe { person_get_commonskill(self, None) }  }
    pub fn get_common_sids(&self) -> Option<&mut Array<&Il2CppString>> { unsafe { get_commonsids(self, None)}}
    pub fn get_flag(&self) -> &mut PersonDataFlag { unsafe { person_get_flag(self, None) }}
    pub fn get_gender(&self) -> i32 { unsafe { person_get_gender(self, None)}  }
    pub fn get_grow(&self) -> &mut Capability { unsafe { person_get_grow(self, None) } }
    pub fn get_help(&self) -> &Il2CppString { unsafe {person_get_help(self, None) }}
    pub fn get_internal_level(&self) -> i8 { unsafe { person_get_internal_level(self, None)} }
    pub fn get_job(&self) -> Option<&JobData> { unsafe { person_get_job(self, None) } }
    pub fn get_jid(&self) -> Option<&Il2CppString> { unsafe { person_get_jid(self, None) }}
    pub fn get_level(&self) -> u8 { unsafe { person_get_level(self, None) } }
    pub fn get_limit(&self) -> &mut CapabilitySbyte {  unsafe { person_get_limit(self, None) } }
    pub fn get_name(&self) -> Option<&Il2CppString> {  unsafe { person_get_name(self, None) } }
    pub fn get_summon_color(&self) -> i32 { unsafe { person_get_summoncolor(self, None)}}
    pub fn get_summon_rank(&self) -> i32 { unsafe { person_get_summon_rank(self, None)}}
    pub fn get_unit_icon_id(&self) -> &'static Il2CppString { unsafe { get_uniticonid(self, None )}}

    pub fn load() { unsafe { persondata_load(None); }}
    pub fn on_complete(&self) { unsafe { person_on_release(self, None); }}

    // Setters
    pub fn set_sub_aptitude(&self, mask: &WeaponMask) { unsafe { person_set_sub_apt(self, mask, None)}}
    pub fn set_ascii_name(&self, name: &Il2CppString) { unsafe { person_set_ascii_name(self, name, None); }}
    pub fn set_attrs(&self, attr: i32) { unsafe { person_set_attrs(self, attr, None); }}
    pub fn set_common_skills(&self, skill: &SkillArray) { unsafe { set_commonskill(self, skill, None); }}
    pub fn set_fid(&self, fid: &Il2CppString) {  unsafe { person_set_fid(self, fid, None);}}
    pub fn set_gender(&self, gender: i32) { unsafe { person_set_gender(self, gender, None); }}
    pub fn set_grow(&self, value: &Capability) { unsafe { person_set_grow(self, value, None); }}
    pub fn set_flag(&self, value: &PersonDataFlag) { unsafe { person_set_flag(self, value, None); }}
    pub fn set_help(&self, help: &Il2CppString) { unsafe { person_set_help(self, help, None); }}
    pub fn set_internal_level(&self, value: i8) { unsafe { person_set_internal_level(self, value, None); }}

    pub fn set_jid(&self, jid: &Il2CppString) { unsafe { person_set_jid(self, jid, None); }}
    pub fn set_level(&self, level: u8) { unsafe { person_set_level(self, level, None); }}
    pub fn set_limit(&self, limits: &CapabilitySbyte) { unsafe { person_set_limit(self, limits, None); }}
    pub fn set_name(&self, name: &Il2CppString) { unsafe { person_set_name(self, name, None); }}
    pub fn set_unit_icon_id(&self, icon_id: &Il2CppString) { unsafe { person_set_uniticonid(self, icon_id, None ); }}
}


// PersonData 
#[unity::from_offset("App", "PersonData", "OnCompleted")]
fn person_on_release(this: &PersonData, method_info: OptionalMethod);

#[unity::from_offset("App", "PersonData", "set_Fid")]
fn person_set_fid(this: &PersonData, fid: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App", "PersonData", "get_Name")] //#[skyline::from_offset(0x1f25d40)]
fn person_get_name(this: &PersonData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[unity::from_offset("App", "PersonData", "get_UnitIconID")] //#[skyline::from_offset(0x1f25d20)]
fn get_uniticonid(this: &PersonData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App", "PersonData", "get_Gender")] //#[skyline::from_offset(0x1f25da0)]
fn person_get_gender(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_Grow")] //#[skyline::from_offset(0x1f26020)]
fn person_get_grow(this: &PersonData, method_info: OptionalMethod) -> &mut Capability;

#[skyline::from_offset(0x1f26140)]
fn person_get_combat_bgm(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[unity::from_offset("App", "PersonData", "get_CommonSids")] //#[skyline::from_offset(0x1f26040)]
fn get_commonsids(this: &PersonData, method_info: OptionalMethod) -> Option<&mut Array<&Il2CppString>>;

#[skyline::from_offset(0x1f2a6f0)]
fn person_get_commonskill(this: &PersonData, method_info: OptionalMethod) -> &SkillArray;

#[skyline::from_offset(0x1f26000)]
fn person_get_limit(this: &PersonData, method_info: OptionalMethod) -> & mut CapabilitySbyte;

#[skyline::from_offset(0x1f2a790)]
fn get_facedata(this: &PersonData, method_info: OptionalMethod) -> &PersonData;

#[skyline::from_offset(0x1f26160)]
fn person_get_ascii_name(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25f40)]
fn person_get_flag(this: &PersonData, method_info: OptionalMethod) -> &mut PersonDataFlag;

#[skyline::from_offset(0x1f261a0)]
fn person_get_attrs(this: &PersonData, method_info: OptionalMethod) -> i32;

#[skyline::from_offset(0x1f29e30)]
fn person_get_job(this: &PersonData, method_info: OptionalMethod) -> Option<&JobData>;

#[skyline::from_offset(0x1f25c60)]
fn person_get_jid(this: &PersonData, method_info: OptionalMethod) -> Option<&Il2CppString>;

#[skyline::from_offset(0x1f25dc0)]
fn person_get_level(this: &PersonData, method_info: OptionalMethod) -> u8;

#[skyline::from_offset(0x1f25de0)]
fn person_get_internal_level(this: &PersonData, method_info: OptionalMethod) -> i8;

#[skyline::from_offset(0x1f25df0)]
fn person_set_internal_level(this: &PersonData, value: i8, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25dd0)]
fn person_set_level(this: &PersonData, value: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25db0)]
fn person_set_gender(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25c50)]
fn person_set_name(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26050)]
fn set_commonsids(this: &PersonData, value: &mut Array<&Il2CppString>, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25d30)]
fn person_set_uniticonid(this: &PersonData, name: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26030)]
fn person_set_grow(this: &PersonData, value: &Capability, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26010)]
fn person_set_limit(this: &PersonData, value: &CapabilitySbyte, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25cc0)]
fn person_get_help(this: &PersonData, method_info: OptionalMethod) -> &Il2CppString;

#[skyline::from_offset(0x1f25cd0)]
fn person_set_help(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a700)]
fn set_commonskill(this: &PersonData, value : &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x1f2a7a0)]
fn set_facedata(this: &PersonData, value : &PersonData, method_info: OptionalMethod);

#[skyline::from_offset(0x1f26170)]
fn person_set_ascii_name(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25f50)]
fn person_set_flag(this: &PersonData, value: &PersonDataFlag, method_info: OptionalMethod);

#[skyline::from_offset(0x1f261b0)]
fn person_set_attrs(this: &PersonData, value: i32, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25c70)]
fn person_set_jid(this: &PersonData, value: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x1f25e60)]
fn person_get_asset_force(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_SubAptitude")]
fn person_get_sub_apt(this: &PersonData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "PersonData", "get_Aptitude")]
fn person_get_apt(this: &PersonData, method_info: OptionalMethod) -> &'static mut WeaponMask;

#[unity::from_offset("App", "PersonData", "set_SubAptitude")]
fn person_set_sub_apt(this: &PersonData, value: &WeaponMask, method_info: OptionalMethod);

#[unity::from_offset("App", "PersonData", "get_SummonRank")]
fn person_get_summon_rank(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "get_SummonColor")]
fn person_get_summoncolor(this: &PersonData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "PersonData", "Load")]
fn persondata_load(method_info: OptionalMethod);
//Capability

#[skyline::from_offset(0x25bcda0)]
fn capability_is_zero(this: &Capability, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x025be030)]
fn capabilitysbyte_is_zero(this: &CapabilitySbyte, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x25bcd00)]
fn capability_add(this: &Capability, i: i32, v: u8, method_info: OptionalMethod);

#[skyline::from_offset(0x25bdf90)]
fn capabilitysbyte_add(this: &CapabilitySbyte, i: i32, v: i8,  method_info: OptionalMethod);

