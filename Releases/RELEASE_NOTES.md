# v0.1 — 衣装、饰品与菜单增强 / Outfit, Accessory and Menu Enhancements

七个角色可用饰品分类、购买分类筛选与装备摘要、头饰与风格镜头、“结合时服装”设置，以及多个菜单的 ZL/ZR 整页翻动。Mask 16 不作为角色可用槽；底层八项存储及 version 1 存档格式保持。

Seven character accessory categories, purchase filters and equipment summaries, accessory cameras, Engage outfit preference and ZL/ZR paging across supported menus. Mask 16 is not a character slot. The eight-entry storage layout and version 1 save format are retained.

## 安装 / Installation

先安装 [Cobalt](https://github.com/Raytwo/Cobalt)。下载 `engage_outfit_menu_enhancements.nro`，放入 `engage/mods/engage-outfit-menu-enhancements/`，完整重启游戏。如果整合包已包含本插件，使用包内版本，避免重复加载。

Install Cobalt first. Place `engage_outfit_menu_enhancements.nro` in `engage/mods/engage-outfit-menu-enhancements/` and fully restart the game. Use the included copy when an integration package already provides the plugin.

基线 / Baseline: Fire Emblem Engage 2.0.0 / Cobalt 1.31.0.

## 验证 / Validation

当前 NRO 来自现用服装整合包，仅外部改名，原字节保持。 / Existing binary from the current outfit integration package, renamed without changing its bytes.

SHA-256: `c394a8a9ed4ef2b20fd2e665a5877563610567605048778a35b661a0ff3ec984`；下载资产含 `SHA256SUMS`。

许可、来源与致谢见仓库 LICENSE、NOTICE、THIRD_PARTY.md 和 README。
See LICENSE, NOTICE, THIRD_PARTY.md and README for licensing, origins and credits.

## 结合时服装设置 / Engage Outfit Setting

在“系统”→“环境设置”中，可选择“结合时服装：角色当前服装／纹章士服装”。战斗中也可即时切换并观察到效果，但切换后已经结合的地图形象需重新结合或重新出击才能生效。龙化等特殊变身将保持现有规则不变。截图见仓库 README。

Under System → Settings, choose the current character outfit or emblem outfit for Engage. The setting can also be switched during battle and its effect observed; already-engaged map models require re-engaging or redeploying before the change takes effect. Special transformations, such as dragon transformation, retain their existing rules. See the README for the screenshot.
