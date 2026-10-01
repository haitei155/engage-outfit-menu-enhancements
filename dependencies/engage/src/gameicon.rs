use unity::prelude::*;
use unity::engine::Sprite;

use crate::spriteatlasmanager::SpriteAtlasManager;

#[unity::class("App", "GameIcon")]
#[static_fields(GameIconStaticFields)]
pub struct GameIcon { }

impl GameIcon {
    pub fn try_get_system<'a>(icon_name: impl Into<&'a Il2CppString>) -> Option<&'static mut Sprite> {
        unsafe { gameicon_trygetsystem(icon_name.into(), None) }
    }
}

pub struct GameIconStaticFields {
    skill: SpriteAtlasManager,
    item: SpriteAtlasManager,
    efficacy: SpriteAtlasManager,
    efficacy_outline: SpriteAtlasManager,
    item_kinds: SpriteAtlasManager,
    item_outline_kinds: SpriteAtlasManager,
    god_symbol: SpriteAtlasManager,
    god_ring: SpriteAtlasManager,
    system: SpriteAtlasManager,
    unit_icon_index: SpriteAtlasManager,
    unit_icon_pallete: SpriteAtlasManager,
}

#[unity::from_offset("App", "GameIcon", "TryGetSystem")]
extern "C" fn gameicon_trygetsystem(icon_name: &Il2CppString, method_info: OptionalMethod) -> Option<&'static mut Sprite>;