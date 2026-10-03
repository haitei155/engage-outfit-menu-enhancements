# v0.3 — 右上角翻页提示与新增页面支持 / Upper-right paging hints and expanded list support

- 本版重点更新：为支持翻页的页面补上右上角原生 ZL/ZR 图标与翻页文字，追加在现有提示末尾，并随菜单和可操作状态更新；翻页文字支持简中、繁中、日文和英文。
- 扩展列表翻页支持：转职职业列表、纹章士戒指／手镯及能力一览、整理持有物品、出售与戒指·手镯图鉴。ZL 向上翻一页、ZR 向下翻一页，支持长按，首尾不循环。
- 考虑到职业排序操作使用频率较低，取消转职列表原 ZR 键的职业排序操作，将 ZR 改为向下翻页；L/R 仍切换单位，职业详情、武器等级与能力预览随选择刷新。
- 戒指／手镯列表的能力一览打开与关闭均由 ZR 改为按下右摇杆；能力一览内部支持整页翻动，右上角显示右摇杆关闭与末尾翻页图标，关闭后恢复戒指／手镯页的完整按键说明。
- 整理持有物品改为按下右摇杆切换左右菜单；出售改为按下右摇杆多项选择。两页均以 ZL/ZR 上下整页翻动，右上角显示新的操作图标与末尾翻页提示。
- 戒指·手镯图鉴选择列表和能力列表加入整页翻动；无能力列表的戒指详情按一页切换条目，并显示末尾翻页提示。

- The main update is the native ZL/ZR paging glyph and caption in the upper-right corner of supported screens. It appears after existing hints and follows the menu and input state, with captions in Simplified Chinese, Traditional Chinese, Japanese and English.
- Paging expands to class-change jobs, Emblem rings/bracelets and abilities, item organization, selling and the ring/bracelet catalog. ZL pages up and ZR pages down, with held-button repeats and no wrapping at the boundaries.
- Because class sorting is considered infrequently used, its original ZR action has been removed from the class-change list. ZR now pages down; L/R still switches units, and class details, weapon ranks and stat previews follow the selection.
- Pressing the right stick replaces ZR for both opening and closing the ring/bracelet ability list. The ability list supports paging, with right-stick Close and the paging glyph last in the upper-right corner. Closing it restores the full ring/bracelet page controls.
- Press the right stick to switch panes in Organize Items or to select/deselect items for multiple selection when selling. Both screens use ZL/ZR for page up/down and show the new action glyph followed by the paging hint.
- The ring/bracelet catalog picker and ability lists gain paging. Ring details without an ability list page through entries one visible page at a time, with the paging hint last.

## 安装与升级 / Installation and upgrade

需要游戏 2.0.0 与 Cobalt，兼容基线为 Cobalt 1.31.0。退出游戏后，将 `engage_outfit_menu_enhancements.nro` 放入 SD 卡的 `engage/mods/engage-outfit-menu-enhancements/`，升级时替换旧文件，再完整重启。服装整合包已包含插件时使用包内副本，避免重复加载。已有配置与饰品存档格式沿用。

Requires game 2.0.0 and Cobalt, with Cobalt 1.31.0 as the compatibility baseline. Exit the game, place `engage_outfit_menu_enhancements.nro` in the SD card's `engage/mods/engage-outfit-menu-enhancements/` folder (replace the old file when upgrading), and fully restart. Use the included copy if an outfit integration package supplies the plugin. Existing configuration and accessory save format are retained.

## 诊断日志 / Diagnostic logging

默认关闭。SD 卡 `engage/fee-outfit-menu-debug.flag` 写 `on` 并完整重启可启用；删除文件或写 `off` 并完整重启可关闭。输出为 `engage/fee-outfit-menu-debug.log`，旧日志保留。模拟器使用其配置的虚拟 SD 卡目录。

Off by default. Write `on` to the SD card's `engage/fee-outfit-menu-debug.flag` and fully restart to enable; delete the file or write `off` and restart to disable. Output: `engage/fee-outfit-menu-debug.log`. Existing logs are retained. Emulators use their configured virtual SD card directory.

完整功能、旧版截图和操作说明见 [中文 README](../README.md) / [English README](../README.en.md)，版本变化见 [CHANGELOG](../CHANGELOG.md)。NRO 校验值见 [SHA256SUMS](SHA256SUMS)。
