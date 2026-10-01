pub const KINDS: [i32; 7] = [0, 1, 2, 3, 5, 6, 7];
pub fn white_pixel(rgba: u32) -> u32 { (rgba & 0xff000000) | 0x00ffffff }
pub fn header_pixel(rgba: u32, kind: i32) -> u32 {
    let r = rgba & 255;
    let g = (rgba >> 8) & 255;
    let b = (rgba >> 16) & 255;
    // Original BattleOutfit PNG: cyan fabric, magenta sword, including edge pixels.
    if kind == 5 && r > g && b > g { 0x00ffffff } else { white_pixel(rgba) }
}
// Exact audited namespaces; never opt in a similarly named map/preparation menu.
pub fn character_menu(namespace: &str, name: &str) -> bool {
    (namespace == "App.CookingMenu" && name == "UnitSelectMenu") ||
    (namespace == "App" && matches!(name,
        "ArenaExpUnitSelectMenu" | "ArenaBondUnitSelectMenu" | "FortuneTellingUnitSelectMenu" |
        "UnitSelectMenu" | "UnitSelectAllUnitMenu" | "UnitSelectRingMenu" | "UnitSelectSortieMenu" |
        "GodUnitSelectMenu" | "GodRoomUnitSelectMenu" |
        "SortieRelianceSelectionUnitMenu" | "SortieRelianceSelectionPartnerMenu" |
        "MyRoomRelianceSelect" | "MyRoomRelianceSubSelect" |
        "ShopUnitSelectMenu" | "WeaponShopBuyMenu" | "ItemShopBuyMenu" | "AccessoryShopBuyMenu" |
        "MaterialListMenu" | "UnitItemMenu" | "PhotographSelectWeaponMenu" | "SkillEditPoolSkillMenu" |
        "SortieTradeItemMenu" | "SendItemMenu" | "DiscardItemMenu" |
        "RefineShopRefineBaseMenu" | "RefineShopRefineTargetMenu" | "RefineShopEngraveItemSelectMenu" |
        "RefineShopEngraveGodMenu" | "RefineShopExchangeSourceMenu" | "RefineShopExchangeTargetMenu" |
        "RefineGodWeaponSelectMenu" | "RefineGodWeaponParamMenu" | "RefineRingUnitSelectMenu"))
}
// Scroll is measured in rows while the selected index includes every column.
pub fn grid_page(index: i32, scroll: i32, count: i32, visible: i32, columns: i32, down: bool) -> (i32, i32) {
    if count <= 0 || visible <= 0 || columns <= 0 { return (index, scroll); }
    let index = index.clamp(0, count - 1);
    let column = index % columns;
    let last_row = (count - 1 - column) / columns;
    let row = index / columns;
    let next_row = (row + if down { visible } else { -visible }).clamp(0, last_row);
    let rows = (count + columns - 1) / columns;
    (next_row * columns + column, (scroll + next_row - row).clamp(0, (rows - visible).max(0)))
}
pub fn category(mask: i32) -> Option<i32> {
    match mask {
        1 => Some(0), 2 => Some(1), 4 => Some(2), 8 => Some(3),
        32 => Some(5), 64 => Some(6), 128 => Some(7),
        _ => None,
    }
}
pub fn page(index: i32, scroll: i32, count: i32, visible: i32, down: bool) -> (i32, i32) {
    if count <= 0 || visible <= 0 { return (index, scroll); }
    let step = visible.min(count);
    let index = index.clamp(0, count - 1);
    let next = (index + if down { step } else { -step }).clamp(0, count - 1);
    let start = (scroll + next - index).clamp(0, (count - step).max(0));
    (next, start)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn silhouettes_preserve_alpha_and_menu_scope_is_explicit() {
        for alpha in 0..=255u32 {
            assert_eq!(white_pixel((alpha<<24)|0x00eaf17f), (alpha<<24)|0x00ffffff);
        }
        assert!(character_menu("App.CookingMenu", "UnitSelectMenu"));
        for name in ["ArenaExpUnitSelectMenu", "ArenaBondUnitSelectMenu", "FortuneTellingUnitSelectMenu"] {
            assert!(character_menu("App", name));
        }
        assert!(character_menu("App", "UnitSelectMenu"));
        assert!(character_menu("App", "SkillEditPoolSkillMenu"));
        assert!(!character_menu("App", "SkillEditEquipSkillMenu"));
        assert!(!character_menu("App.CookingMenu", "DishSelectMenu"));
        assert!(!character_menu("App", "MapDeployMenu"));
    }
    #[test] fn sword_is_transparent_and_grid_keeps_column() {
        assert_eq!(header_pixel(0xffaa2288, 5), 0x00ffffff);
        assert_eq!(header_pixel(0xff88ffee, 5), 0xffffffff);
        assert_eq!(header_pixel(0x7faa2288, 3), 0x7fffffff);
        assert_eq!(grid_page(5,0,31,5,2,true), (15,5));
        assert_eq!(grid_page(15,5,31,5,2,false), (5,0));
        assert_eq!(grid_page(29,11,31,5,2,true), (29,11));
        assert_eq!(grid_page(0,0,0,5,2,true), (0,0));
        for name in ["InventoryUnitItemMenu", "InventoryPoolItemMenu", "ShopSellMenu", "WellItemSelectMenu"] {
            assert!(!character_menu("App", name));
        }
    }
    #[test] fn masks_keep_every_supported_slot_and_exclude_sommie() {
        for (i, mask) in [1,2,4,8,32,64,128].iter().enumerate() {
            assert_eq!(category(*mask), Some(KINDS[i]));
        }
        for mask in [0,16,3,256] { assert_eq!(category(mask), None); }
    }
    #[test] fn paging_preserves_screen_row_and_clamps_both_ends() {
        assert_eq!(page(7,0,100,12,true),(19,12));
        assert_eq!(page(19,12,100,12,false),(7,0));
        assert_eq!(page(3,0,100,12,false),(0,0));
        assert_eq!(page(95,88,100,12,true),(99,88));
        assert_eq!(page(0,0,5,12,true),(4,0));
        assert_eq!(page(4,0,5,12,false),(0,0));
        assert_eq!(page(0,0,0,12,true),(0,0));
        assert_eq!(page(3,0,100,0,true),(3,0));
    }
}

// Empty summaries stay hidden even while browsing that category.
pub fn summary_visible(_kind: i32, equipped: bool, _selected: i32) -> bool { equipped }
#[cfg(test)]
mod summary_tests {
    use super::*;
    #[test]
    fn empty_slots_never_show_rows_or_category_cursor() {
        for selected in KINDS {
            for kind in KINDS {
                assert!(!summary_visible(kind, false, selected));
                assert!(summary_visible(kind, true, selected));
            }
        }
        assert!(character_menu("App", "MaterialListMenu"));
        assert!(!character_menu("App", "InventoryPoolItemMenu"));
    }
    #[test]
    fn shop_categories_partition_items_and_preserve_empty_result() {
        let masks = [1,2,4,8,32,64,128];
        for kind in KINDS {
            let page: Vec<_> = masks.iter().filter(|m| category(**m) == Some(kind)).collect();
            assert_eq!(page.len(),1);
            let empty: Vec<i32> = [].into_iter().filter(|m| category(*m) == Some(kind)).collect();
            assert!(empty.is_empty());
        }
        assert_eq!(category(16),None);
    }
}
