pub use unity::prelude::*;
use unity::il2cpp::object::Array;
use super::{Gamedata, GodData, unit::GodUnit, skill::SkillArray};
use crate::gamedata::StructBaseFields;
#[unity::class("App", "UnitItemList")]
pub struct UnitItemList {
	pub unit_items: &'static Array<&'static UnitItem>
}

#[unity::class("App", "ItemData")]
pub struct ItemData {
	pub parent: StructBaseFields,
	pub iid: &'static Il2CppString,
	pub name: &'static Il2CppString,
	pub help: &'static Il2CppString,
	pub tutorial: &'static Il2CppString,
	pub aid: &'static Il2CppString,
	pub kind: u32,
	pub attr: u32,
	pub usetype: u32,
	pub weapon_attr: u32,
	pub icon: &'static Il2CppString,
	pub endurance: u8,
	pub power: u8,
}
impl Gamedata for ItemData { }

#[unity::class("App", "UnitItem")]
pub struct UnitItem {
    pub index: i32,
    pub item: &'static ItemData,
    pub endurance: u8,
    pub refine_level: u8,
    pub flags :i32,
    pub engrave: Option<&'static GodData>,
    pub god_unit: Option<&'static GodUnit>,
}
impl ItemData {
	pub fn get_kind(&self) -> i32 { unsafe { itemdata_get_kind(self, None)}}
	pub fn get_weapon_level(&self) -> i32 { unsafe { itemdata_get_weapon_level(self, None)}}
	
}
impl UnitItem {
	pub fn ctor(&self, item: &ItemData) { unsafe { unititem_ctor(self, item, None); }}
	pub fn ctor_str(&self, key: &str) {
		let item = ItemData::get(key);
		if item.is_some() {
			self.ctor(item.unwrap());
		}
	}
	pub fn dispose(&self) { unsafe { unititem_dispose(self, None); }}
	pub fn get_equipped_skills(&self) ->  Option<&SkillArray> { unsafe { unititem_get_equip_skills(self, None)}}
	pub fn get_power(&self) -> i32 { unsafe { unititem_get_power(self, None)}}

	pub fn is_empty(&self) -> bool { unsafe { unititem_is_empty(self, None) } }
	pub fn is_weapon(&self) -> bool {  unsafe { unititem_is_weapon(self, None) } }
	pub fn is_drop(&self) -> bool { unsafe {  unititem_get_is_drop(self, None) } }
	pub fn refine_data_exist(&self) -> bool { unsafe { unititem_is_exsistrefinedata(self, None)} }

	pub fn set_engrave(&self, engrave: &GodData) -> bool { unsafe { unititem_set_engrave(self, engrave, None)}}
	pub fn set_refine_level(&self, level: i32) { unsafe { unititem_set_refine_level(self, level, None); }}
}

impl UnitItemList {
	pub fn add(&self, item: &ItemData) { unsafe { unititemlist_add(self, item, None); }}
	pub fn get_count(&self) -> i32 { unsafe { unititemlist_get_count(self, None)}}
	pub fn get_item(&self, index: i32) -> Option<&'static mut UnitItem> { unsafe { unititemlist_get_item(self, index, None)}}
	pub fn has_item(&self, item: &ItemData) -> bool { unsafe { unititemlist_has_item(self, item, None)}}
	pub fn has_item_iid(&self, iid: &str) -> bool { 
		let item = ItemData::get(iid);
		if item.is_some() {
			unsafe { return unititemlist_has_item(self, item.unwrap(), None)}
		}
		return false;
	}
	pub fn add_item_no_duplicate(&self, item: &ItemData){
		if !self.has_item(item) { self.add(item); }
	}
	pub fn move_item(&self, from: i32, to: i32) { unsafe { unititemlist_move(self, from, to, None) } }
}

//ItemData
#[unity::from_offset("App", "ItemData", "get_Kind")]
fn itemdata_get_kind(this: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "ItemData", "GetWeaponLevel")]
fn itemdata_get_weapon_level(this: &ItemData, method_info: OptionalMethod) -> i32;

//UnitItemList
#[unity::from_offset("App", "UnitItemList", "get_Count")]
pub fn unititemlist_get_count(this: &UnitItemList, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "get_Item")]
pub fn unititemlist_get_item(this: &UnitItemList, index: i32, method_info: OptionalMethod) ->  Option<&'static mut UnitItem>;

#[unity::from_offset("App", "UnitItemList", "HasItem")]
pub fn unititemlist_has_item(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) ->  bool;

#[skyline::from_offset(0x01fb3ab0)]
pub fn unititemlist_add(this: &UnitItemList, item: &ItemData, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItemList", "Move")]
pub fn unititemlist_move(this: &UnitItemList, from: i32, to: i32, method_info: OptionalMethod);
//UnitItem
#[unity::from_offset("App", "UnitItem", "Dispose")]
pub fn unititem_dispose(this: &UnitItem, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "set_RefineLevel")]
pub fn unititem_set_refine_level(this: &UnitItem, value: i32, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsExistRefineData")]
pub fn unititem_is_exsistrefinedata(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "SetEngrave")]
pub fn unititem_set_engrave(this: &UnitItem, data: &GodData, method_info: OptionalMethod) -> bool;

#[skyline::from_offset(0x01fad9e0)]
pub fn unititem_ctor(this: &UnitItem, item: &ItemData, method_info: OptionalMethod);

#[unity::from_offset("App", "UnitItem", "IsWeapon")]
pub fn unititem_is_weapon(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "get_IsDrop")]
pub fn unititem_get_is_drop(this: &UnitItem, method_info: OptionalMethod) -> bool;

#[unity::from_offset("App", "UnitItem", "GetPower")]
pub fn unititem_get_power(this: &UnitItem, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App", "UnitItem", "GetEquipSkills")]
pub fn unititem_get_equip_skills(this: &UnitItem, method_info: OptionalMethod) -> Option<&SkillArray>;

#[unity::from_offset("App", "UnitItem", "IsEmpty")]
pub fn unititem_is_empty(this: &UnitItem, method_info: OptionalMethod) -> bool;