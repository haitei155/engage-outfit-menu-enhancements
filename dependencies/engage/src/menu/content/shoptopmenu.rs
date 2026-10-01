use unity::prelude::*;

use crate::menu::BasicMenuContent;

#[unity::from_offset("App", "ShopTopMenuContent", "LoadPrefabAsync")]
pub fn shop_top_menu_content_load_prefab_async(method_info: OptionalMethod);

#[unity::from_offset("App", "ShopTopMenuContent", "LoadPrefabAsync")]
pub fn shop_top_menu_content_unload_prefab(method_info: OptionalMethod);

#[unity::from_offset("App", "ShopTopMenuContent", "Create")]
pub fn shop_top_menu_content_create(method_info: OptionalMethod) -> Option<&'static mut BasicMenuContent>;
