#[repr(i32)]
#[derive(Debug, PartialEq)]
pub enum TitleLoopSequenceLabel {
    Start = 0,
    StartFromMainMenu = 1,
    GrandOpening = 2,
    Title = 3,
    TitleFromMainMenu = 4,
    JobIntro = 5,
    End = 6,
}
