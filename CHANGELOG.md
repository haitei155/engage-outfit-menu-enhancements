# 更新记录 / Changelog

## v0.2 — 战前换装与默认关闭日志 / Preparation dressing and optional logging

- 在战斗准备“据点索拉涅尔”子菜单内新增“装饰品店”，位于锻造店与特技继承之间，支持简中、繁中、日文和英文入口。
- 进入后使用原生装饰品店场景、相机、灯光和蓝色背景进行实时试穿；支持右摇杆旋转／缩放，新人物加载完成后替换旧预览。
- 店内隐藏战场血条／头像／职业标记，返回恢复战场界面与镜头。
- 退出比较全部七个可见分类的最终装备，仅刷新确实修改过且已有战场模型的我方单位；仅浏览或改后改回原值不刷新。
- 过渡采用原生约0.25秒淡出／淡入，并在人物完成加载后显示；2026-10-03用户已确认当前v0.2功能测试通过。
- 正式插件默认不输出诊断日志。SD卡 `engage/fee-outfit-menu-debug.flag` 写 `on` 并完整重启可开启；删除或写 `off` 后完整重启可关闭，旧日志保留。详细步骤见双语 README。
- 截图统一为1280×720 JPG，新增战前入口截图；底层独立诊断包不随公开插件发布。

Adds the accessory shop below the Smithy in the preparation Somniel submenu, with native-scene live dressing, right-stick controls and restored battlefield overlays on exit. Final equipment across all seven categories is compared; only changed player actors refresh. Transitions use native fades and wait for model loading. The user confirmed that current v0.2 functionality passed testing on 2026-10-03. Diagnostics are off by default: write `on` to the SD-card `engage/fee-outfit-menu-debug.flag` and restart to enable; delete the flag or write `off` and restart to disable. Existing logs are retained. Screenshots use 1280×720 JPG, including the new preparation entry image.

## v0.1 — 2026-10-02

七个角色可用饰品分类、购买分类筛选与装备摘要、头饰与风格镜头、“结合时服装”设置，以及多个菜单的 ZL/ZR 整页翻动。Mask 16 不作为角色可用槽；底层八项存储及 version 1 存档格式保持。

Seven character accessory categories, purchase filters and equipment summaries, accessory cameras, Engage outfit preference and ZL/ZR paging across supported menus. Mask 16 is not a character slot. The eight-entry storage layout and version 1 save format are retained.

当前 NRO 来自现用服装整合包，仅外部改名，原字节保持。 / Existing binary from the current outfit integration package, renamed without changing its bytes.
