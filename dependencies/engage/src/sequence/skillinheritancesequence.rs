use unity::prelude::*;

use crate::proc::{
    Bindable,
    desc::ProcDesc
};

use super::hubrefineshopsequence::SortieSequenceItemShop;

#[unity::class("App", "SkillInheritanceSequence")]
pub struct SkillInheritanceSequence { }

impl Bindable for SkillInheritanceSequence { }

impl SkillInheritanceSequence {
    pub fn new() -> &'static mut Self {
        let item = Self::instantiate().unwrap();
        item.klass = SortieSequenceItemShop::class().clone();
        item
    }

    pub fn create_desc(&self) -> &'static mut Il2CppArray<&'static mut ProcDesc> {
        unsafe { skillinheritancesequence_createdesc(self, None) }
    }
}

#[unity::from_offset("App", "SkillInheritanceSequence", "CreateDesc")]
fn skillinheritancesequence_createdesc(this: &SkillInheritanceSequence, method_info: OptionalMethod) -> &'static mut Il2CppArray<&'static mut ProcDesc>;