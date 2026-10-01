use unity::prelude::*;

use crate::proc::Bindable;

#[repr(C)]
#[unity::class("App", "GmapSequence")]
pub struct GmapSequence {}

impl Bindable for GmapSequence {}
