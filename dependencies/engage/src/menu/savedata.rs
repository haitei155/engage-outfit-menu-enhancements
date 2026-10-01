use unity::engine::ui::Image;

use crate::{gameuserdata::GameMode, tmpro::TextMeshProUGUI};

use super::BasicMenuItem;

#[unity::class("App", "SaveDataMenuItemContent")]
pub struct SaveDataMenuItemContent {
    __: [u8; 0x98],
    pub mode_text: &'static mut TextMeshProUGUI,
    pub game_mode_image: &'static mut Image,
    // ...
}

// We use BasicMenuItem here because we can't represent this class as an Object yet
#[repr(C)]
pub struct SaveDataMenuMenuItem {
    pub basicmenuitem: BasicMenuItem,
    pub save_data_header_handle: &'static mut GameSaveDataHeaderReaderHandle,
    // ...
}

// Child class of GameSaveDataHeaderReader, can't properly represent it for now
#[repr(C)]
pub struct GameSaveDataHeaderReaderHandle {
    object: [u8; 0x10],
    __: [u8; 0x10],
    pub header: Option<&'static mut GameSaveDataHeader>,
    // ...
}

#[unity::class("App", "GameSaveDataHeader")]
pub struct GameSaveDataHeader {
    __: [u8; 0x24],
    pub gamemode: GameMode,
    // ...
}