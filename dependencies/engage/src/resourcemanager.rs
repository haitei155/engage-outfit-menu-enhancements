use unity::prelude::*;

#[unity::from_offset("App", "ResourceManager", "IsLoading")]
pub fn is_loading(method_info: OptionalMethod) -> bool;
