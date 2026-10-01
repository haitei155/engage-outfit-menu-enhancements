//! Methods related to the flow of maps and their scripting.

use unity::prelude::*;

use crate::{proc::ProcInst, script::DynValue};

#[repr(C)]
#[unity::class("App", "EventSequence")]
pub struct EventSequence { }

impl EventSequence {
    /// Note: the arguments are currently Option until more is figure out. They serve no purpose at the moment.
    pub fn try_create_bind(
        proc: &ProcInst,
        func: &'static DynValue,
        _pre_call: Option<()>,
        _post_call: Option<()>,
        _args: Option<()>,
    ) {
        // TODO: Figure out the arguments
        unsafe {
            eventsequence_trycreatebind(proc, func, 0 as _, 0 as _, 0 as _, None);
        }
    }
}

#[unity::from_offset("App", "EventSequence", "MapOpening")]
fn eventsequence_mapopening(parent: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "EventSequence", "TryCreateBind")]
fn eventsequence_trycreatebind(
    parent: &ProcInst,
    func: &'static DynValue,
    pre_call: *const u8,
    post_call: *const u8,
    args: *const u8,
    method_info: OptionalMethod,
) -> bool;
