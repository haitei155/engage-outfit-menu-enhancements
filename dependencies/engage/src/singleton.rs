//! Utilities to be used with the [`get_generic_class!`](unity::get_generic_class) macro.

#[repr(C)]
#[unity::class("App", "SingletonProcInst`1")]
pub struct SingletonProcInst { }

#[repr(C)]
#[unity::class("App", "SingletonClass`1")]
pub struct SingletonClass { }
