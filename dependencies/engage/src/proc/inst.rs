use std::cell::UnsafeCell;

use unity::prelude::*;

use super::{Bindable, ProcDesc, RawValueStack};

/// Represents a Instruction unit for a [`Proc`].
///
/// The ProcInst is chained to other ProcInsts to be executed in order by one of the three Proc instances of the game.
///
/// They act as a high-level virtual machine to execute [instructions](crate::proc::ProcDesc) in sequence. Said instructions are queued in a ProcDesc array.
/// 
/// A lot of classes inherit from it, so implement [`IsProcInst`] on them to represent this relation.
///
/// If it is not possible for some reason, use the [cast](ProcInst::cast) methods to represent the chains as a ProcInst derivate of your choice.
///
/// Example:
///
/// ```
/// pub fn createmenubind_hook(proc: &mut Il2CppObject<ProcInst>) {
///     let casted_child = proc.child.cast_mut::<BasicMenu>();
/// }
/// ```
/// 
/// Keep in mind that [`IsProcInst`] is much more preferable, and [cast](ProcInst::cast) should only be used as a last resort.
#[repr(C)]
#[unity::class("App", "ProcInst")]
pub struct ProcInst {
    descs: &'static mut UnsafeCell<Il2CppArray<&'static mut ProcDesc>>,
    pub desc_index: i32,
    pub name: Option<&'static Il2CppString>,
    /// Unique ID derived from the name of the ProcInst.
    pub hashcode: i32,
    /// The ProcInst this instance is attached to
    pub parent: Option<&'static mut ProcInst>,
    /// The next ProcInst to process. ProcInsts are processed from child to parent.
    pub child: Option<&'static mut ProcInst>,
    pub prev: Option<&'static mut ProcInst>,
    pub next: Option<&'static mut ProcInst>,
    /// Note:  Observed a ProcVoidMethod being set here
    persistent: *const u8,
    /// Note: Actually a bitfield to mark ProcInsts for death (to be destroyed)
    pub state: i32,
    /// This is supposed to be a bool, but struct alignment said otherwise.
    pub suspend: i32,
    wait_time: f32,
    tick_time: f32,
    // RawValueStack
    stack: &'static Il2CppObject<RawValueStack>,
}

impl Drop for ProcInst {
    fn drop(&mut self) {
        panic!("ProcInst dropped");
    }
}

impl ProcInst {
    pub fn get_child(&'static self) -> &'static ProcInst {
        // Ray: yes, this'd crash if null. I'll fix later.
        *self.child.as_ref().unwrap()
    }

    pub fn get_child_mut(&'static mut self) -> &'static mut ProcInst {
        // Ray: yes, this'd crash if null. I'll fix later.
        *self.child.as_mut().unwrap()
    }

    pub fn get_next(&'static self) -> &'static ProcInst {
        // Ray: yes, this'd crash if null. I'll fix later.
        *self.next.as_ref().unwrap()
    }

    pub fn get_next_mut(&'static mut self) -> &'static mut ProcInst {
        // Ray: yes, this'd crash if null. I'll fix later.
        *self.next.as_mut().unwrap()
    }

    pub fn get_descs(&self) -> &Il2CppArray<&'static mut ProcDesc> {
        unsafe { &*self.descs.get() }
    }

    pub fn get_descs_mut(&self) -> &mut Il2CppArray<&'static mut ProcDesc> {
        unsafe {&mut *self.descs.get() }
    }

    pub fn cast<T: AsRef<ProcInstFields>>(&self) -> &T {
        unsafe { std::mem::transmute::<&ProcInst, &T>(self) }
    }

    pub fn cast_mut<T: AsMut<ProcInstFields>>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut ProcInst, &mut T>(self) }
    }
}

impl Bindable for ProcInst { }

#[unity::from_offset("App", "ProcInst", "CreateBind")]
pub fn procinst_createbind<T: Bindable + ?Sized, P: Bindable>(
    this: &T,
    parent: &P,
    descs: &'static Il2CppArray<&'static mut ProcDesc>,
    name: &'static Il2CppString,
    method_info: OptionalMethod,
);

#[unity::from_offset("App", "ProcInst", "OnDispose")]
pub fn procinst_ondispose(this: &ProcInst, method_info: OptionalMethod);

#[unity::from_offset("App", "ProcInst", "Jump")]
pub fn procinst_jump<T: Bindable + ?Sized>(this: &T, label: i32, method_info: OptionalMethod);
