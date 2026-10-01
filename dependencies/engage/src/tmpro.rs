use unity::prelude::*;

/// You can usually use this instead of a class inheriting from it at the risk of missing fields and methods.
#[unity::class("TMPro", "TextMeshProUGUI")]
pub struct TextMeshProUGUI {

}

impl TextMeshProUGUI {
    pub fn set_text(&mut self, source_text: &Il2CppString, sync_text_input_box: bool) {
        unsafe { tmptext_settext(self, source_text, sync_text_input_box, None) };
    }
}

#[skyline::from_offset(0x2837690)]
fn tmptext_settext(this: &mut TextMeshProUGUI, source_text: &Il2CppString, sync_text_input_box: bool, method_info: OptionalMethod);