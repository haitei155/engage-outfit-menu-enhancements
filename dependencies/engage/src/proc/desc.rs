use std::ops::{Deref, DerefMut};

use unity::prelude::*;

use super::{ProcBoolMethod, ProcVoidFunction, ProcVoidMethod, proc_label, proc_call, proc_end, proc_wait_while_true, proc_wait_time, Bindable, Delegate};

#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProcDescType {
    End = 0,
    Halt = 1,
    Jump = 2,
    Label = 3,
    Push = 4,
    Pop = 5,
    Persistent = 6,
    WaitTime = 7,
    WaitFrame = 8,
    Yield = 9,
    Call = 10,
    Tick = 11,
    Args = 12,
    WaitFunc = 13,
    JumpFunc = 14,
    User = 15,
    Max = 16,
}

/// A Descriptor to be executed as part of a [`ProcInst`](crate::proc::ProcInst)
///
/// Acts as a very high level assembly instruction that is executed sequentially.
#[repr(C)]
#[unity::class("App", "ProcDesc")]
pub struct ProcDesc {
    pub ty: ProcDescType,
}

impl ProcDesc {
    pub fn label(label: i32) -> &'static mut ProcDesc {
        unsafe { proc_label(label, None) }
    }

    pub fn call(method: &'static mut impl Delegate) -> &'static mut ProcDesc {
        unsafe { proc_call(method, None) }
    }

    pub fn wait_while_true<T>(method: &'static mut ProcBoolMethod<T>) -> &'static mut ProcDesc {
        unsafe { proc_wait_while_true(method, None) }
    }

    // pub fn function_call<T>(function: &'static mut ProcVoidFunction<T>) -> Il2CppResult<&'static mut ProcDescCall<T>> {
    //     ProcDescCall::<T>::instantiate().map(|desc| {
    //         desc.desc.ty = ProcDescType::Call;
    //         desc.function = function;
    //         desc
    //     })
    // }

    pub fn jump(to_label: i32) -> &'static mut ProcDesc {
        let result = ProcDescJump::instantiate()
            .map(|desc| {
                desc.desc.ty = ProcDescType::Jump;
                desc.label = to_label;
                desc
            }
        ).unwrap();

        unsafe { std::mem::transmute(result) }
    }

    pub fn yiel() -> Il2CppResult<&'static mut ProcDescYield> {
        ProcDescYield::instantiate().map(|desc| {
            desc.desc.ty = ProcDescType::Yield;
            desc
        })
    }

    pub fn wait_time(time: f32) -> &'static mut ProcDesc {
        unsafe { proc_wait_time(time, None) }
    }

    pub fn end() -> &'static mut ProcDesc {
        unsafe { proc_end(None) }
    }

    pub fn cast<T: AsRef<ProcDescFields>>(&self) -> &T {
        unsafe { std::mem::transmute::<&ProcDesc, &T>(&self) }
    }

    pub fn cast_mut<T: AsMut<ProcDescFields>>(&mut self) -> &mut T {
        unsafe { std::mem::transmute::<&mut ProcDesc, &mut T>(self) }
    }

    pub fn get_label(&self) -> i32 {
        let method = self.get_class().get_virtual_method("get_Label").unwrap();

        let get_label = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> i32>(
                method.method_info.method_ptr,
            )
        };

        get_label(self, method.method_info)
    }

    pub fn get_desc_type(&self) -> ProcDescType {
        let method = self.get_class().get_method_from_name("get_DescType", 0).unwrap();

        let get_desc_type = unsafe {
            std::mem::transmute::<_, extern "C" fn(&Self, &MethodInfo) -> ProcDescType>(
                method.method_ptr,
            )
        };

        get_desc_type(self, method)
    }  
}

#[unity::from_offset("App", "ProcDescJump", ".ctor")]
fn procdescjump_ctor(
    this: &'static mut ProcDesc,
    label: i32,
    method_info: OptionalMethod,
);

#[repr(C)]
#[unity::class("App", "ProcDescLabel")]
pub struct ProcDescLabel {
    pub desc: ProcDescFields,
    pub label: i32,
}

impl AsRef<ProcDescFields> for ProcDescLabel {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl AsMut<ProcDescFields> for ProcDescLabel {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

impl Deref for ProcDescLabelFields {
    type Target = ProcDescFields;

    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl DerefMut for ProcDescLabelFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescJump")]
pub struct ProcDescJump {
    pub desc: ProcDescFields,
    pub label: i32,
}

impl AsRef<ProcDescFields> for ProcDescJump {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl AsMut<ProcDescFields> for ProcDescJump {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescMCall")]
pub struct ProcDescMCall<T: 'static + Bindable> {
    pub desc: ProcDescFields,
    pub method: &'static mut ProcVoidMethod<T>,
}

impl<T: Bindable> AsRef<ProcDescFields> for ProcDescMCallFields<T> {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl<T: Bindable> AsMut<ProcDescFields> for ProcDescMCallFields<T> {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

impl<T: Bindable> Deref for ProcDescMCallFields<T> {
    type Target = ProcDescFields;

    fn deref(&self) -> &Self::Target {
        &self.desc
    }
}

impl<T: Bindable> DerefMut for ProcDescMCallFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescMWaitTrue")]
pub struct ProcDescMWaitTrue<T: 'static> {
    pub desc: ProcDescFields,
    pub method: &'static mut ProcBoolMethod<T>,
}

impl<T> AsRef<ProcDescFields> for ProcDescMWaitTrueFields<T> {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl<T> AsMut<ProcDescFields> for ProcDescMWaitTrueFields<T> {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescMWaitTrueFields<T> {
    type Target = ProcDescFields;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescMWaitTrueFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescCall")]
pub struct ProcDescCall<T: 'static> {
    pub desc: ProcDescFields,
    pub function: &'static mut ProcVoidFunction<T>,
}

impl<T> AsRef<ProcDescFields> for ProcDescCallFields<T> {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl<T> AsMut<ProcDescFields> for ProcDescCallFields<T> {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

impl<T> Deref for ProcDescCallFields<T> {
    type Target = ProcDescFields;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for ProcDescCallFields<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescEnd")]
pub struct ProcDescEnd {
    pub desc: ProcDescFields,
}

impl AsRef<ProcDescFields> for ProcDescEnd {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl AsMut<ProcDescFields> for ProcDescEnd {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescYield")]
pub struct ProcDescYield {
    pub desc: ProcDescFields,
}

impl AsRef<ProcDescFields> for ProcDescYield {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl AsMut<ProcDescFields> for ProcDescYield {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

#[repr(C)]
#[unity::class("App", "ProcDescWaitTime")]
pub struct ProcDescWaitTime {
    pub desc: ProcDescFields,
    pub time: f32,
}

impl AsRef<ProcDescFields> for ProcDescWaitTimeFields {
    fn as_ref(&self) -> &ProcDescFields {
        &self.desc
    }
}

impl AsMut<ProcDescFields> for ProcDescWaitTimeFields {
    fn as_mut(&mut self) -> &mut ProcDescFields {
        &mut self.desc
    }
}

impl Deref for ProcDescWaitTimeFields {
    type Target = ProcDescFields;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl DerefMut for ProcDescWaitTimeFields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}