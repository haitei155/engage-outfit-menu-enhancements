pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use unity::system::List;
use super::GodData;

impl GodData {
    pub fn get_engage_attack(&self) -> &'static Il2CppString { unsafe { goddata_get_engage_attack(self, None) }}
    pub fn get_engrave_avoid(&self) -> i8 { unsafe{ goddata_get_engrave_avoid(self, None) }}
    pub fn get_engrave_critical(&self) -> i8 { unsafe{ goddata_get_engrave_critical(self, None) }}
    pub fn get_engrave_hit(&self) -> i8 { unsafe{ goddata_get_engrave_hit(self, None) }}
    pub fn get_engrave_power(&self) -> i8 { unsafe{ goddata_get_engrave_power(self, None) }}
    pub fn get_engrave_secure(&self) -> i8 { unsafe{ goddata_get_engrave_secure(self, None) }}
    pub fn get_engrave_weight(&self) -> i8 { unsafe{ goddata_get_engrave_weight(self, None) }}

    pub fn load() { unsafe { goddata_load(None); }}

    pub fn set_engage_attack(&self, value: &Il2CppString) { unsafe { goddata_set_engage_attack(self, value, None); }}
    pub fn set_engrave_avoid(&self, value: i8) { unsafe{ goddata_set_engrave_avoid(self, value, None); }}
    pub fn set_engrave_critical(&self, value: i8) { unsafe{ goddata_set_engrave_critical(self, value, None); }}
    pub fn set_engrave_hit(&self, value: i8) { unsafe{ goddata_set_engrave_hit(self, value, None); }}
    pub fn set_engrave_power(&self, value: i8)  { unsafe{ goddata_set_engrave_power(self, value, None); }}
    pub fn set_engrave_secure(&self, value: i8)  { unsafe{ goddata_set_engrave_secure(self, value, None); }}
    pub fn set_engrave_weight(&self, value: i8) { unsafe{ goddata_set_engrave_weight(self, value, None); }}

    pub fn set_engrave(&self, index: i32, value: i8){
        match index {
            0 => self.set_engrave_avoid(value),
            1 => self.set_engrave_critical(value),
            2 => self.set_engrave_hit(value),
            3 => self.set_engrave_power(value),
            4 => self.set_engrave_secure(value),
            5 => self.set_engrave_weight(value),
            _ => {},
        }
    }

}

#[unity::class("App", "GodGrowthData")]
pub struct GodGrowthData {}

impl GodGrowthData {
    pub fn get_engage_skills(&self) -> &mut Array<&Il2CppString> { unsafe { ggd_get_engage_skills(self, None)}}
    pub fn get_inheritance_skills(&self) ->  Option<&mut Array<&Il2CppString>> { unsafe { ggd_set_inherit_skills(self, None)}}
    pub fn load() { unsafe { ggd_load(None); }}
    pub fn on_complete(&self) { unsafe { ggd_on_complete(self, None); }}
    pub fn try_get_from_god_data(this: &GodData) -> Option<&'static List<GodGrowthData>> { unsafe { ggd_try_get_from_god(this, None)}}
    pub fn unload() {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        if method.is_none() {  method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));  }
        if method.is_none() { return; }
        let unload = unsafe { std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> ( method.unwrap().method_ptr,) };
        unload(method.unwrap());
    }
}


// GodData 
#[unity::from_offset("App","GodData","get_EngageAttack")]
fn goddata_get_engage_attack(this: &GodData, method_info: OptionalMethod) -> &'static Il2CppString;

#[unity::from_offset("App","GodData","get_EngraveAvoid")]
fn goddata_get_engrave_avoid(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveCritical")]
fn goddata_get_engrave_critical(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveHit")]
fn goddata_get_engrave_hit(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngravePower")]
fn goddata_get_engrave_power(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveSecure")]
fn goddata_get_engrave_secure(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","get_EngraveWeight")]
fn goddata_get_engrave_weight(this: &GodData, method_info: OptionalMethod) -> i8;

#[unity::from_offset("App","GodData","set_EngageAttack")]
fn goddata_set_engage_attack(this: &GodData, value: &Il2CppString, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveAvoid")]
fn goddata_set_engrave_avoid(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveCritical")]
fn goddata_set_engrave_critical(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveHit")]
fn goddata_set_engrave_hit(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngravePower")]
fn goddata_set_engrave_power(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveSecure")]
fn goddata_set_engrave_secure(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","set_EngraveWeight")]
fn goddata_set_engrave_weight(this: &GodData, value: i8, method_info: OptionalMethod);

#[unity::from_offset("App","GodData","Load")]
fn goddata_load(method_info: OptionalMethod);

// God Growth Data
#[unity::from_offset("App", "GodGrowthData", "get_EngageSkills")]
fn ggd_get_engage_skills(this: &GodGrowthData, method_info: OptionalMethod) -> &mut Array<&Il2CppString>;

#[unity::from_offset("App", "GodGrowthData", "get_InheritanceSkills")]
fn ggd_set_inherit_skills(this: &GodGrowthData, method_info: OptionalMethod) -> Option<&mut Array<&Il2CppString>>;

#[unity::from_offset("App", "GodGrowthData", "Load")]
fn ggd_load(method_info: OptionalMethod);

#[unity::from_offset("App", "GodGrowthData", "OnCompleted")]
fn ggd_on_complete(this: &GodGrowthData, method_info: OptionalMethod);

#[unity::from_offset("App", "GodGrowthData", "TryGetFromGodData")]
fn ggd_try_get_from_god(god: &GodData, method_info: OptionalMethod) -> Option<&'static List<GodGrowthData>>;