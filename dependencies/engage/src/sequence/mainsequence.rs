use unity::prelude::*;

use crate::proc::ProcInstFields;

#[repr(C)]
#[unity::class("App", "MainSequence")]
pub struct MainSequence {
    // Start SingletonProcInst here
    pub proc: ProcInstFields,
    is_resume: bool,
    is_loaded: bool,
    // End here
    pub scene_name: &'static mut Il2CppString,
    pub scene_mode: i32,
}

#[repr(C)]
pub struct MainSequenceStaticFields {
    pub jump_label: MainSequenceLabel,
    pub fake_label: i32,
    pub initialized: bool,
}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum MainSequenceLabel {
    None = 0,
    Startup = 1,
    TitleLoop = 2,
    TitleLoopFromMainMenu = 3,
    MainMenu = 4,
    Chapter = 5,
    Gmap = 6,
    Kizuna = 7,
    Hub = 8,
    HubToSavePosition = 9,
    Ending = 10,
    NextChapter = 11,
    Map = 12,
    Complete = 13,
    GameOver = 14,
    ChapterSave = 15,
    AfterChapterSave = 16,
    SetSaveDataLoadTarget = 17,
    SaveDataLoad = 18,
    SaveDataLoadFailed = 19,
    SaveDataVersionFailed = 20,
    DataLoadFailed = 21,
    AfterLoadFailed = 22,
    ContentsResume = 23,
    RelayDebug = 24,
    Relay = 25,
    Versus = 26,
    Challenge = 27,
    BackToTitle = 28,
    End = 29,
}

impl AsRef<ProcInstFields> for MainSequence {
    fn as_ref(&self) -> &ProcInstFields {
        &self.proc
    }
}

impl AsMut<ProcInstFields> for MainSequence {
    fn as_mut(&mut self) -> &mut ProcInstFields {
        &mut self.proc
    }
}
