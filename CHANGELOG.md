# 更新记录 / Changelog

## v0.3 — 右上角翻页提示与新增页面支持 / Upper-right paging hints and expanded list support

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

## v0.2 — 战前换装与默认关闭日志 / Preparation dressing and optional logging

- 在战斗准备“据点索拉涅尔”子菜单内新增“装饰品店”，位于锻造店与特技继承之间，支持简中、繁中、日文和英文入口。
- 进入后使用原生装饰品店场景、相机、灯光和蓝色背景进行实时试穿；支持右摇杆旋转／缩放，新人物加载完成后替换旧预览。
- 店内隐藏战场血条／头像／职业标记，返回恢复战场界面与镜头。
- 退出比较全部七个可见分类的最终装备，仅刷新确实修改过且已有战场模型的我方单位；仅浏览或改后改回原值不刷新。
- 过渡采用原生约0.25秒淡出／淡入，并在人物完成加载后显示。
- 正式插件默认不输出诊断日志。SD卡 `engage/fee-outfit-menu-debug.flag` 写 `on` 并完整重启可开启；删除或写 `off` 后完整重启可关闭，旧日志保留。详细步骤见双语 README。
- 新增战前装饰品店入口截图。

Adds the accessory shop below the Smithy in the preparation Somniel submenu, with native-scene live dressing, right-stick controls and restored battlefield overlays on exit. Final equipment across all seven categories is compared; only changed player actors refresh. Transitions use native fades and wait for model loading. Diagnostics are off by default: write `on` to the SD-card `engage/fee-outfit-menu-debug.flag` and restart to enable; delete the flag or write `off` and restart to disable. Existing logs are retained. Adds a screenshot of the preparation accessory-shop entry.

## v0.1 — 2026-10-02

七个角色可用饰品分类、购买分类筛选与装备摘要、头饰与风格镜头、“结合时服装”设置，以及多个菜单的 ZL/ZR 整页翻动。Mask 16 不作为角色可用槽；底层八项存储及 version 1 存档格式保持。

Seven character accessory categories, purchase filters and equipment summaries, accessory cameras, Engage outfit preference and ZL/ZR paging across supported menus. Mask 16 is not a character slot. The eight-entry storage layout and version 1 save format are retained.
