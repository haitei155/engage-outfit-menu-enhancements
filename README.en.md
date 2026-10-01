# Fire Emblem Engage — Outfit, Accessory and Menu Enhancements

[简体中文](README.md)

Initial release: **[v0.1](https://github.com/haitei155/engage-outfit-menu-enhancements/releases/tag/v0.1)**.

This project is modified and extended from **SierraSak**’s [EngageAcc_Code](https://github.com/SierraSak/EngageAcc_Code) / [Expanded Accessory Slots](https://gamebanana.com/mods/512066). Thank you to SierraSak for the original accessory-slot implementation and resources. Characters use seven available accessory categories. Mask 16 is not exposed as a character slot. The upstream eight-entry storage layout and version 1 save format are retained.

**Install [Cobalt](https://github.com/Raytwo/Cobalt) first.** Thank you to **Raytwo** for developing and maintaining Cobalt and providing the mod-loading foundation used by this project.

The current binary targets Fire Emblem Engage 2.0.0 with Cobalt 1.31.0 as its compatibility baseline.

## Features

- Seven character categories: outfit, head, face, back, battle outfit, hair color and style (Mask 1/2/4/8/32/64/128). Mask 16 is not available as a character slot; the eight-entry storage layout and version 1 save format are preserved.
- Seven menu categories, purchase filters and current-equipment summaries, with unequipped summary rows hidden.
- Adds ZL/ZR page scrolling to ring, roster and preparation selection, arenas, cooking selection, support and bond lists, outer item selection, materials and valuables, shop selection and purchases, forging, engraving, exchanges and photo-mode weapon lists. Two-column lists retain the selected column. Inherited-skill and inventory lists gain paging only in states without native ZL/ZR actions; native help and category controls are preserved.
- Camera adjustments for head accessories and alternate styles.
- Engage outfit preference: current character outfit or emblem outfit. Re-engage or redeploy to update an already-engaged map model.
- Includes the current read-only map-body diagnostic logging.

## Feature Screenshot

The original game allows up to two accessories. With these enhancements, a character can equip multiple accessories simultaneously, with clear entries in the equipment summary on the left. The screenshot shows multiple equipped accessories and the category menu.

![Multiple equipped accessories and the equipment summary](images/multiple-accessories-equipment-summary.png)

### Engage Outfit Setting

Under **System → Settings**, choose the Engage outfit preference: **current character outfit / emblem outfit**.

You can also switch the setting during battle and observe its effect. However, map models that are already engaged require re-engaging or redeploying before the change takes effect. Special transformations, such as dragon transformation, retain their existing rules.

![Settings during battle: current character outfit selected for Engage](images/engage-outfit-settings-in-battle.png)

## Download and Installation

Current binary: [`engage_outfit_menu_enhancements.nro`](Releases/engage_outfit_menu_enhancements.nro); checksums: [SHA256SUMS](Releases/SHA256SUMS).

1. Install the prerequisite runtime using Cobalt's instructions.
2. Place the NRO in `engage/mods/engage-outfit-menu-enhancements/` and fully restart the game.
3. If the outfit integration package already contains this plugin, use that included copy and avoid installing another standalone copy.

## Source and Build

See [Build instructions](docs/BUILD.md). This snapshot includes the current implementation and the local dependencies used by it. New builds go to `dist/`, preserving the release binary in `Releases/`.

## License and Credits

Project-owned additions and documentation use the [GNU GPL v3.0](LICENSE), and combined-program distribution must comply with GPLv3. Third-party dependencies, upstream-derived code and resources retain their respective license terms and notices; see [THIRD_PARTY.md](THIRD_PARTY.md) and [NOTICE](NOTICE).

Thanks to [Raytwo](https://github.com/Raytwo/Cobalt), [SierraSak](https://github.com/SierraSak/EngageAcc_Code), [DivineDragonFanClub](https://github.com/DivineDragonFanClub/engage-il2cpp), [skyline-rs](https://github.com/skyline-rs) and all relevant dependency contributors.
