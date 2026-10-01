use unity::prelude::*;

use crate::proc::{ProcInstFields, Bindable};

#[unity::class("App", "ConfigSequence")]
pub struct ConfigSequence {
    pub proc: ProcInstFields,
}

impl ConfigSequence {
    pub fn create_bind(parent: &impl Bindable) {
        unsafe { configsequence_createbind(parent, None) }
    }
}

impl AsRef<ProcInstFields> for ConfigSequence {
    fn as_ref(&self) -> &ProcInstFields {
      &self.proc
    }
}

impl AsMut<ProcInstFields> for ConfigSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}

impl Bindable for ConfigSequence { }

#[unity::from_offset("", "ConfigSequence", "CreateBind")]
fn configsequence_createbind<T: Bindable + ?Sized>(parent: &T, method_info: OptionalMethod);