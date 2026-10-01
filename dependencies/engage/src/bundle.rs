use unity::prelude::*;

#[unity::class("App", "TextAssetBundle")]
pub struct TextAssetBundle {
    handle: [u8; 18],
    asset: *const u8, // UnityEngine::TextAsset
    pub bytes: Option<&'static mut Il2CppArray<u8>>,
}

#[unity::from_offset("App", "TextAssetBundle", ".ctor")]
pub fn textassetbundle_ctor(this: &mut TextAssetBundle, path: &Il2CppString, method_info: OptionalMethod);