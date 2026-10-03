# v0.2 — 战前实时换装 / Live dressing in battle preparations

在战斗准备“据点索拉涅尔”子菜单新增“装饰品店”，位于锻造店与特技继承之间。进入原生店铺场景实时试穿，支持右摇杆旋转和缩放；确认装备后返回即可刷新战场形象，方便战前换装。店内隐藏战场图标，退出恢复；原生淡出／淡入等待人物加载完成。全部七个可见分类参与最终装备差分，只刷新实际修改过的我方战场人物。

Adds Accessory Shop below the Smithy in the preparation Somniel submenu. Use native-scene live previews and right-stick rotation/zoom, then confirm equipment and return to refresh the battlefield appearance. Battlefield overlays hide in the shop and restore on exit. Native fades wait for character loading. Final equipment across all seven visible categories is compared; only changed player actors refresh.

七分类、装备摘要、ZL/ZR翻页及“结合时服装”是v0.1已有功能，v0.2继续保留。截图统一1280×720 JPG。

Seven categories, equipment summaries, ZL/ZR paging and the Engage outfit preference are retained from v0.1. Screenshots use 1280×720 JPG.

## 日志 / Logging

默认不输出插件诊断日志。需要时，在SD卡engage目录建立UTF-8文件fee-outfit-menu-debug.flag，内容写on并完整重启。日志追加到sd:/engage/fee-outfit-menu-debug.log。删除标志文件或写off并完整重启可关闭；进程首次读取后缓存，旧日志保留。详细步骤见双语README。

Diagnostics are off by default. To enable, create the UTF-8 file engage/fee-outfit-menu-debug.flag on the SD card containing on, then fully restart. Output appends to sd:/engage/fee-outfit-menu-debug.log. Delete the flag or write off and fully restart to disable. The value is cached after its first read; old logs remain. See the README for full instructions.

## 安装 / Installation

先安装[Cobalt](https://github.com/Raytwo/Cobalt)。将engage_outfit_menu_enhancements.nro放入engage/mods/engage-outfit-menu-enhancements/，完整重启。整合包已含本插件时使用包内副本，避免重复安装。

Install Cobalt first. Place engage_outfit_menu_enhancements.nro in engage/mods/engage-outfit-menu-enhancements/ and fully restart. Use the included copy if an integration package already supplies this plugin.

基线 / Baseline: Fire Emblem Engage 2.0.0 / Cobalt 1.31.0.

## 验证与编译版来源 / Validation and binary provenance

2026-10-03用户确认当前v0.2功能通过。发布源码与已测试的原生转场版（native-fade-v8）一致；发行NRO取自同版整合包，原字节保持。静态验证包括NRO0/MOD0、SHA-256、ZIP CRC和其他成员一致性；发布项目另进行离线锁定Switch重建验证。

The user confirmed current v0.2 functionality on 2026-10-03. Published source matches the tested native-fade-v8 implementation. The release NRO is copied unchanged from that integration build. Static validation includes NRO0/MOD0, SHA-256, ZIP CRC and unchanged-member checks; the publication project also undergoes a locked offline Switch rebuild.

SHA-256: `95d74f95d615028d787ef95060227f6bedd1f012c05620980c605d806ec5f5e9`.

许可、来源与致谢见LICENSE、NOTICE、THIRD_PARTY.md及README。
See LICENSE, NOTICE, THIRD_PARTY.md and README for licenses, origins and credits.
