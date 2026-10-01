//! Structures representing a singular entry from the gamedata files in memory.

use std::ops::Deref;
use unity::{prelude::*, system::ListFields};

pub mod accessory;
pub mod person;
pub mod skill;
pub mod job;
pub mod dispos;
pub mod unit;
pub mod item;
pub mod cook;
pub mod animal;
pub mod god;

#[unity::class("App", "HubFacilityData")]
pub struct HubFacilityData {
    pub parent: StructBaseFields,
    pub aid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
    pub condition_cid: &'static Il2CppString,
    pub icon_name: &'static Il2CppString,
}
impl Gamedata for HubFacilityData { }
impl HubFacilityData {
    pub fn is_complete(&self) -> bool { unsafe { hubdatafacility_iscomplete(self, None) } }
}

#[skyline::from_offset(0x28a80d0)]
fn hubdatafacility_iscomplete(this: &HubFacilityData, _method_info: OptionalMethod) -> bool;

#[unity::class("App", "JobData")]
pub struct JobData {
    pub parent: StructBaseFields, //0x0
    pub jid: &'static Il2CppString, //0x10
    pub name: &'static Il2CppString, //0x18
    pub aid: &'static Il2CppString, //0x20
    pub help: &'static Il2CppString, //0x28
    junk: [u8; 0xD0],
    pub learn_skill: Option<&'static Il2CppString>, // 0x100
    pub lunatic_skill: Option<&'static Il2CppString>, //0x108
}
impl Gamedata for JobData { }

#[unity::class("App", "PersonData")]
pub struct PersonData {
    pub parent: StructBaseFields,
    pub pid: &'static Il2CppString,
    pub name: &'static Il2CppString,
    // ...
}
impl Gamedata for PersonData { }

#[unity::class("App", "GodData")]
pub struct GodData {
    pub parent: StructBaseFields,
    pub gid: &'static Il2CppString,
    pub mid: &'static Il2CppString,
}
impl Gamedata for GodData {}


#[unity::class("App", "StructDataArray`1")]
pub struct StructDataArray {}

#[unity::class("App", "StructData`1")]
pub struct StructDataGeneric { }

#[unity::class("App", "StructData`1")]
pub struct StructData { }

// pub static_fields: &'static StructDataStaticFields<T>,

#[derive(Clone, Copy)]
pub struct StructDataStaticFields<T: 'static> {
    pub s_list: &'static StructList<T>,
    pub loaded: bool,
}

#[unity::class("App", "StructList<`1>")]
pub struct StructList<T: 'static> {
    pub list: ListFields<T>,
}

impl<T> Deref for StructListFields<T> {
    type Target = ListFields<T>;

    fn deref(&self) -> &Self::Target {
        &self.list
    }
}

impl<T> StructList<T> {
    pub fn add(&mut self, element: &'static mut T) {
        let method = self.get_class().get_virtual_method("Add").unwrap();
        let add = unsafe {
            std::mem::transmute::<_, extern "C" fn(&mut Self, &'static mut T, &MethodInfo)>(
                method.method_info.method_ptr,
            )
        };

        add(self, element, method.method_info);
    }
}

impl<T> StructListFields<T> {
    pub fn len(&self) -> usize {
        self.size as _
    }

    pub fn capacity(&self) -> usize {
        self.items.len() as _
    }
}

#[unity::class("App", "StructBase")]
pub struct StructBase {
    pub index: i32,
    pub hash: i32,
    pub key: &'static Il2CppString,
}

#[unity::class("App", "WeaponMask")]
pub struct WeaponMask {
    pub value: i32,
}

pub trait Gamedata: Il2CppClassData + Sized {
    fn get(name: &str) -> Option<&'static Self> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        }
        if method.is_none() {
            return None;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static Self>>(
                method.unwrap().method_ptr,
            )
        };
    
        get(name.into(), method.unwrap())
    }
    
    fn get_mut(name: &str) -> Option<&'static mut Self> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Get")));
        }
        if method.is_none() {
            return None;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> Option<&'static mut Self>>(
                method.unwrap().method_ptr,
            )
        };
    
        get(name.into(), method.unwrap())
    }

    fn get_index(name: &Il2CppString) ->  i32 {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetIndex")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetIndex")));
        }
        if method.is_none() {
            return -1;
        }
        let get = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Il2CppString, &MethodInfo) -> i32>(
                method.unwrap().method_ptr,
            )
        };
    
        get(name, method.unwrap())
    }

    fn get_list() -> Option<&'static StructList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        if method.is_none() {
            return None;
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static StructList<Self>>>(
                method.unwrap().method_ptr,
            )
        };
    
        get_list(method?)
    }
    
    fn get_list_mut() -> Option<&'static mut StructList<Self>> {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetList")));
        }
        if method.is_none() {
            return None;
        }
        let get_list = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> Option<&'static mut StructList<Self>>>(
                method.unwrap().method_ptr,
            )
        };
    
        get_list(method?)
    }

    fn get_count() -> i32 {
        let method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("GetCount"))).unwrap();
        
        let get_count = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> i32>(
                method.method_ptr,
            )
        };
        get_count(method)
    }

    fn unload() {
        let mut method = Self::class()._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        if method.is_none() {
            method = Self::class()._1.parent._1.parent.get_methods().iter().find(|method| method.get_name() == Some(String::from("Unload")));
        }
        if method.is_none() {
            return;
        }
        let unload = unsafe {
            std::mem::transmute::<_, extern "C" fn(&MethodInfo) -> ()> (
                method.unwrap().method_ptr,
            )
        };
        unload(method.unwrap());
    }
}