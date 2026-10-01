# Fire Emblem Engage — 衣装、饰品与菜单增强

[English](README.en.md)

首发版本：**[v0.1](https://github.com/haitei155/engage-outfit-menu-enhancements/releases/tag/v0.1)**。

本项目基于 **SierraSak** 的 [EngageAcc_Code](https://github.com/SierraSak/EngageAcc_Code) / [Expanded Accessory Slots](https://gamebanana.com/mods/512066) 修改并扩展。感谢原作者 SierraSak 提供饰品槽实现与资源。角色使用七个可用饰品分类；Mask 16 不作为角色可用槽。保留上游底层八项存储布局和 version 1 存档格式，新增功能见下文。

**必须先安装 [Cobalt](https://github.com/Raytwo/Cobalt)。** 感谢 **Raytwo** 开发并维护 Cobalt，为本项目提供 MOD 加载与扩展基础。

当前编译版面向《火焰纹章 Engage》2.0.0，使用 Cobalt 1.31.0 作为兼容基线。

## 功能

- 七个角色可用分类：衣装、头部、面部、背部、战场衣装、发色、风格（Mask 1/2/4/8/32/64/128）。Mask 16 不作为角色可用槽；底层八项存储与 version 1 存档格式保持。
- 七类菜单标签、购买分类筛选和当前装备摘要；未装备的摘要项隐藏。
- 为戒指／编组／整备选人、斗技场、料理选人、支援与牵绊、外层持有物品、材料与贵重品、商店选人与购买、锻造／刻印／交换及拍照武器等列表加入 ZL/ZR 整页翻动；双列列表保持所在列。已继承特技和库存仅在 ZL/ZR 无原生用途的状态补翻页，保留原生帮助、类别切换等按键操作。
- 头部配饰与另类风格的镜头调整。
- “结合时服装”设置：角色当前服装／纹章士服装。已结合的地图形象需要重新结合或重新出击才能应用切换。
- 内含当前地图主体只读诊断日志功能。

## 功能截图

原版最多佩戴 2 件饰品；衣装增强后，角色可以同时佩戴多件饰品，并在左侧装备摘要中明确显示。下图展示了多件饰品同时装备及分类菜单的效果。

![多件饰品同时装备与装备摘要](images/multiple-accessories-equipment-summary.png)

### 结合时服装设置

在“系统”→“环境设置”中，可选择“结合时服装：角色当前服装／纹章士服装”。

战斗中也可即时切换并观察到效果，但切换后已经结合的地图形象需重新结合或重新出击才能生效。龙化等特殊变身将保持现有规则不变。

![战斗中的环境设置：结合时服装选择角色当前服装](images/engage-outfit-settings-in-battle.png)

## 下载与安装

当前编译版：[`engage_outfit_menu_enhancements.nro`](Releases/engage_outfit_menu_enhancements.nro)；校验值见 [SHA256SUMS](Releases/SHA256SUMS)。

1. 先按 Cobalt 的说明安装基础运行环境。
2. 将 NRO 放在 `engage/mods/engage-outfit-menu-enhancements/` 下，完整重启游戏。
3. 如果服装主包已包含本插件，就使用包内版本，避免再装一个独立副本。

## 源码与构建

见 [构建说明](docs/BUILD.md)。源码包括当前功能实现和实际使用的本地依赖快照；构建产物输出到 `dist/`，不会覆盖 `Releases/` 中的发行编译版。

## 许可与致谢

本项目自行新增的代码与文档采用 [GNU GPL v3.0](LICENSE)，组合程序按 GPL v3 要求分发。第三方依赖、上游派生代码及资源保留各自许可与权利声明；详细来源与范围见 [THIRD_PARTY.md](THIRD_PARTY.md) 和 [NOTICE](NOTICE)。

感谢 [Raytwo](https://github.com/Raytwo/Cobalt)、[SierraSak](https://github.com/SierraSak/EngageAcc_Code)、[DivineDragonFanClub](https://github.com/DivineDragonFanClub/engage-il2cpp)、[skyline-rs](https://github.com/skyline-rs) 以及相关依赖贡献者。
