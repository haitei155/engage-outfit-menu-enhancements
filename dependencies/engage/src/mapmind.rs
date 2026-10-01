//! Types and methods used in the zoomed-out map view.
use unity::prelude::*;

use crate::{gamedata::unit::Unit, util::get_instance};

#[repr(C)]
#[unity::class("App", "MapMind")]
pub struct MapMind {}

// App.MapMind$$get_Unit	7101dee2b0	App_Unit_o * App.MapMind$$get_Unit(App_MapMind_o * __this, MethodInfo * method)	12
#[unity::from_offset("App", "MapMind", "get_Unit")]
fn get_unit(this: &MapMind, method_info: OptionalMethod) -> &Unit;

impl MapMind {
    pub fn get_instance() -> &'static mut MapMind {
        get_instance::<MapMind>()
    }

    /// Seems to get the current unit that is selected by the player. Needs more experimentation.
    pub fn get_unit() -> &'static Unit {
        let instance = Self::get_instance();
        unsafe { get_unit(instance, None) }
    }
}
