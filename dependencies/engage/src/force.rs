use unity::prelude::*;
use crate::gamedata::unit::Unit;

#[repr(C)]
pub enum ForceType {
    Player = 0,
    Enemy = 1,
    Ally = 2,
    Absent = 3,
    Dead = 4,
    Lost = 5,
    Temporary = 6,
    Empty = 7,
}

#[unity::class("App", "Force")]
pub struct Force {
    pub head: Option<&'static Unit>,
    pub tail: Option<&'static Unit>,
    pub force_type: i32,
    // ...
}

impl Force {
    pub fn get(ty: ForceType) -> Option<&'static mut Force> {
         unsafe { force_gettype(ty, None) } 
    }
    pub fn get_count(&self) -> i32 { 
        unsafe { force_get_count(self, None) } 
    }
    pub fn iter(&self) -> ForceIterator { 
        ForceIterator(self.head) 
    }
    pub fn transfer(&self, force_type: i32, is_last: bool) {
        unsafe { force_transfer(self, force_type, is_last, None); } 
    }
    pub fn get_hero_unit(&self) -> &'static mut Unit {
        unsafe { force_get_hero_unit(self, None) }
    }
}

pub struct ForceIterator(Option<&'static Unit>);

impl Iterator for ForceIterator {
    type Item = &'static Unit;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(unit) => {
                let res = Some(unit);
                self.0 = unit.next;
                res
            },
            None => None,
        }
    }
}
#[skyline::from_offset(0x2616200)]
fn force_gettype(ty: ForceType, _method_info: OptionalMethod) -> Option<&'static mut Force>;

#[unity::from_offset("App", "Force", "GetCount")]
fn force_get_count(this: &Force, method_info: OptionalMethod) -> i32;

#[unity::from_offset("App","Force","Transfer")]
fn force_transfer(this: &Force, forcetype: i32, is_last: bool,method_info: OptionalMethod);

#[unity::from_offset("App", "Force", "GetHeroUnit")]
fn force_get_hero_unit(this: &Force, method_info: OptionalMethod) -> &'static mut Unit;