use unity::prelude::*;

use super::{BasicDialogItem, BasicDialogItemFields};
use crate::{menu::BasicMenuResult, proc::Bindable};

#[repr(C)]
pub struct YesNoDialog(super::BasicDialog<BasicDialogItem>);

impl YesNoDialog {
    pub fn bind<Methods: TwoChoiceDialogMethods>(
        proc: &impl Bindable,
        message: impl AsRef<str>,
        first_text: impl AsRef<str>,
        second_text: impl AsRef<str>,
    ) {
        let first_item = BasicDialogItemYes::new(first_text);
        let second_item = BasicDialogItemNo::new(second_text);

        first_item
            .get_class_mut()
            .get_virtual_method_mut("ACall")
            .map(|method| method.method_ptr = Methods::on_first_choice as _);

        second_item
            .get_class_mut()
            .get_virtual_method_mut("ACall")
            .map(|method| method.method_ptr = Methods::on_second_choice as _);

        unsafe {
            yesnodialog_createbind(proc, message.as_ref().into(), first_item, second_item, None);
        }
    }
}

pub trait TwoChoiceDialogMethods {
    extern "C" fn on_first_choice(_this: &mut BasicDialogItemYes, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true)
    }
    extern "C" fn on_second_choice(_this: &mut BasicDialogItemNo, _method_info: OptionalMethod) -> BasicMenuResult {
        BasicMenuResult::new().with_close_this(true)
    }
}

#[unity::from_offset("App", "YesNoDialog", "CreateBind")]
extern "C" fn yesnodialog_createbind<P: Bindable>(
    proc: &P,
    mess: &Il2CppString,
    yes_item: &BasicDialogItemYes,
    no_item: &BasicDialogItemNo,
    method_info: OptionalMethod,
);

#[skyline::from_offset(0x2456430)]
fn dialog_item_yes_ctor(proc: &BasicDialogItemYes, text: &Il2CppString, method_info: OptionalMethod);

#[skyline::from_offset(0x24562a0)]
fn dialog_item_no_ctor(proc: &BasicDialogItemNo, text: &Il2CppString, method_info: OptionalMethod);

#[repr(transparent)]
#[unity::class("App", "BasicDialogItemYes")]
pub struct BasicDialogItemYes {
    pub parent: BasicDialogItemFields,
}

impl BasicDialogItemYes {
    pub fn new(text: impl AsRef<str>) -> &'static mut BasicDialogItemYes {
        let item = BasicDialogItemYes::instantiate().unwrap();

        unsafe {
            dialog_item_yes_ctor(item, text.into(), None);
        }

        item
    }
}

#[repr(transparent)]
#[unity::class("App", "BasicDialogItemNo")]
pub struct BasicDialogItemNo {
    pub parent: BasicDialogItemFields,
}

impl BasicDialogItemNo {
    pub fn new(text: impl AsRef<str>) -> &'static mut BasicDialogItemNo {
        let item = BasicDialogItemNo::instantiate().unwrap();

        unsafe {
            dialog_item_no_ctor(item, text.into(), None);
        }

        item
    }
}
