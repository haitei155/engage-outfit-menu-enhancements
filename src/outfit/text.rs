// IDs from Engage 2.0.0 Language.Langs, including both English regions.
pub struct Text {
    pub title: &'static str,
    pub current: &'static str,
    pub emblem: &'static str,
    pub help: &'static str,
}
pub fn text(lang: i32) -> Text {
    match lang {
        10 => Text {
            title: "结合时服装", current: "角色当前服装", emblem: "纹章士服装",
            help: "切换后，已结合的地图形象需重新结合或重新出击才能生效。",
        },
        9 => Text {
            title: "結合時服裝", current: "角色目前服裝", emblem: "紋章士服裝",
            help: "切換後，已結合的地圖形象需重新結合或重新出擊才能生效。",
        },
        0 => Text {
            title: "エンゲージ中の衣装", current: "現在の衣装", emblem: "紋章士の衣装",
            help: "変更後、エンゲージ中のマップ上の姿に反映するには、再度エンゲージするか出撃し直してください。",
        },
        _ => Text {
            title: "Engage Outfit", current: "Current Outfit", emblem: "Emblem Outfit",
            help: "After switching, engage again or redeploy to update the appearance of units already engaged on the map.",
        },
    }
}
pub fn decode_setting(value: &str) -> bool { value.trim() == "current" }
