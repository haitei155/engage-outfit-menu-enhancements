//! Utility to spawn a notification popup similar to the achievements.

use unity::prelude::*;

#[unity::from_offset("App", "NoticeManager", "Add")]
fn noticemanager_add(kind: i32, text: &Il2CppString, method_info: OptionalMethod);

pub struct NoticeManager;

impl NoticeManager {
    pub fn add<'a, S: Into<&'a Il2CppString>>(message: S) {
        unsafe { noticemanager_add(1, message.into(), None) }
    }
}
