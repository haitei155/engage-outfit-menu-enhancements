use unity::prelude::*;

#[repr(C)]
#[unity::class("App", "LoadingManager")]
pub struct LoadingManager { }

impl LoadingManager {
    pub fn bind() {
        unsafe { loadingmanager_bind(None) };
    }

    pub fn unbind() {
        unsafe { loadingmanager_unbind(None) };
    }

    pub fn try_fade_bind() {
        unsafe { loadingmanager_tryfadebind(None) };
    }

    pub fn is_bind() -> bool {
        unsafe { loadingmanager_isbind(None) }
    }
}

#[unity::from_offset("App", "LoadingManager", "Bind")]
fn loadingmanager_bind(method_info: OptionalMethod);

#[unity::from_offset("App", "LoadingManager", "Unbind")]
fn loadingmanager_unbind(method_info: OptionalMethod);

#[unity::from_offset("App", "LoadingManager", "TryFadeBind")]
fn loadingmanager_tryfadebind(method_info: OptionalMethod);

#[unity::from_offset("App", "LoadingManager", "IsBind")]
fn loadingmanager_isbind(method_info: OptionalMethod) -> bool;