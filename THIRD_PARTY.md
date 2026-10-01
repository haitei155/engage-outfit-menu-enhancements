# 第三方来源与许可 / Third-party Sources and Licensing

根目录 GPL-3.0-only 适用于本项目有权许可的新增代码和文档，组合程序按 GPL v3 的要求分发。第三方原始许可与权利声明保留；选择 GPL 不能代替缺失的上游授权。源码中已有声明保持原样。

The root GPL-3.0-only license applies to project-owned additions and documentation, and combined-program distribution must comply with GPLv3. Third-party terms and notices remain in place; choosing GPL does not grant rights absent from upstream licensing. Existing notices are preserved.

| 组件 / Component | 来源 / Origin | 许可范围 / License scope |
|---|---|---|
| `src/expanded/`, `resources/` upstream portions | [SierraSak/EngageAcc_Code](https://github.com/SierraSak/EngageAcc_Code), baseline `e87b62b8393c8048fc40455634a8aa7613b68efb` | Upstream snapshot has no explicit standalone LICENSE. GPL-3.0 does not itself grant redistribution rights for these portions; review upstream permission before public distribution. |
| `dependencies/engage` | [DivineDragonFanClub/engage](https://github.com/DivineDragonFanClub/engage) | Retained GNU GPL v3 license text in its own LICENSE; original GPL license retained. |
| `dependencies/unity` | [DivineDragonFanClub/unity](https://github.com/DivineDragonFanClub/unity) | Retained GNU GPL v3 license text in its own LICENSE; original GPL license retained. |
| `dependencies/lazysimd` | [DivineDragonFanClub/lazysimd](https://github.com/DivineDragonFanClub/lazysimd) | Retained GNU GPL v3 license text in its own LICENSE; original GPL license retained. |
| `dependencies/engage-il2cpp-0.1.0-reconstructed` | [DivineDragonFanClub/engage-il2cpp](https://github.com/DivineDragonFanClub/engage-il2cpp), compile-input tag 0.1.0 / `27a129b0debd0973f6f6986226045bd8dc1b2689` | MPL-2.0; retained LICENSE. Local dependency paths adjusted. |
| `dependencies/unity-nx-0.1.0-local` | [DivineDragonFanClub/unity-nx](https://github.com/DivineDragonFanClub/unity-nx) | MPL-2.0; retained LICENSE-MPL and NOTICE, including godot-rust attribution. |
| `dependencies/horizon-svc` | [skyline-rs/horizon-svc](https://github.com/skyline-rs/horizon-svc), `ced970c` | No explicit standalone LICENSE in the copied snapshot; retain authorship and review permission before distribution. |
| `tools/dependencies/linkle-0.2.11` | [MegatonHammer/linkle](https://github.com/MegatonHammer/linkle) | MIT / Apache-2.0 per Cargo manifest; bundled license files retained where supplied. |

`tools/link.T` 来自本机 Skyline 链接脚本快照，保留用于 Switch 构建；其来源属于 skyline-rs 工具链，不作为本项目原创代码重新许可。

`tools/link.T` is the Skyline linker script snapshot used for Switch builds; it belongs to the skyline-rs toolchain and is not relicensed as original project code.

Cargo.lock 记录其余 Cargo 依赖及精确版本；这些依赖保持其上游许可。公开发布前需审阅整合发行的许可范围，尤其是 GPL 依赖和没有明确许可的上游部分。

Cargo.lock records other Cargo dependencies and their exact versions; their original terms continue to apply. Review the combined distribution terms before publication, especially GPL dependencies and upstream portions without explicit licensing.
