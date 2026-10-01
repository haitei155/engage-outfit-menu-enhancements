use unity::prelude::*;

use crate::force::Force;

#[unity::class("App", "UnitPool")]
pub struct UnitPool { }

impl UnitPool {
    pub fn get_force(index: i32) -> &'static Force {
        unsafe { unitpool_getforce(index, None) }
    }
}

#[unity::from_offset("App", "UnitPool", "GetForce")]
fn unitpool_getforce(index: i32, method_info: OptionalMethod) -> &'static Force;