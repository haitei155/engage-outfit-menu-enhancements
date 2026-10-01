pub use unity::prelude::*;
use super::*;
// Structs, methods required for SkillArray and SkillData

#[unity::class("App", "SkillArray")]
pub struct SkillArray {}

#[unity::class("App", "SkillData")]
pub struct SkillData {
    pub parent: StructBaseFields,
    pub commands: *const u8,  // from the calculator part
    pub sid: &'static Il2CppString,
    pub name: Option<&'static Il2CppString>,
    pub help: Option<&'static Il2CppString>,
}
impl Gamedata for SkillData{}

impl SkillArray {
    pub fn clear(&self){ unsafe { skillarray_clear(self, None); } }
    pub fn copy(&self, src: &SkillArray) { unsafe { skillarray_copy(self, src, None); }}
    pub fn ctor(&self, src: &SkillArray) { unsafe { skillarray_ctor(self, src, None); }}
    pub fn find_sid(&self, sid: &Il2CppString) -> Option<&'static SkillData> { unsafe { skillarray_find(self, sid, None)}}
    pub fn remove_sid(&self, sid: &Il2CppString) -> bool { unsafe { skill_array_remove(self, sid, None)}}
    pub fn skill_array_add(&self, add: &SkillArray) -> bool { unsafe { add_skill_array(self, add, None)}}
    pub fn index_of(&self, sid: &Il2CppString) -> i32 { unsafe { sid_index_of(self, sid, None)}}
}

impl SkillData {
    pub fn can_override_skill(&self) -> bool { unsafe {skilldata_can_override_skill(self, None)}}
    pub fn get_efficacy(&self) -> i32 { unsafe { skilldata_get_efficacy(self, None)} }
    pub fn get_efficacy_value(&self) -> i32 { unsafe { skilldata_get_efficacy_value(self, None)}}
    pub fn get_flag(&self) -> i32{ unsafe { skilldata_get_flag(self, None)}}
    pub fn get_inheritance_cost(&self) -> u16 { unsafe { skilldata_get_inherit_cost(self, None)}}
    pub fn get_priority(&self) -> i32 { unsafe { skilldata_priority(self, None) }}
    pub fn has_effect(&self) -> bool { unsafe { skilldata_has_effect(self, None)}}
    pub fn is_style_skill(&self) -> bool { unsafe { skilldata_is_style(self, None)}}
    pub fn load() { unsafe { skilldata_load(None); }}
}

//Skill Array
#[unity::from_offset("App","SkillArray", "Clear")]
fn skillarray_clear(this: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x0247dda0)]
fn skillarray_ctor(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[unity::from_offset("App","SkillArray", "Copy")]
fn skillarray_copy(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod);

#[skyline::from_offset(0x02482850)]
fn skill_array_remove(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x24820c0)]
fn add_skill_array(this: &SkillArray, src: &SkillArray, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x02487990)]
fn skillarray_find(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> Option<&'static SkillData>;

#[skyline::from_offset(0x02487020)]
fn sid_index_of(this: &SkillArray, sid: &Il2CppString, method_info: OptionalMethod) -> i32;

//SkillData
#[unity::from_offset("App", "SkillData", "get_Efficacy")]
fn skilldata_get_efficacy(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_EfficacyValue")]
fn skilldata_get_efficacy_value(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_Flag")]
fn skilldata_get_flag(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "IsStyleSkill")]
fn skilldata_is_style(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "CanOverrideSkill")]
fn skilldata_can_override_skill(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "HasEffect")]
fn skilldata_has_effect(this: &SkillData, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "SkillData", "get_Priority")]
fn skilldata_priority(this: &SkillData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "SkillData", "get_InheritanceCost")]
fn skilldata_get_inherit_cost(this: &SkillData, method_info: OptionalMethod) -> u16;

#[unity::from_offset("App", "SkillData", "Load")]
fn skilldata_load(method_info: OptionalMethod);