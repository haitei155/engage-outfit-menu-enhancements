# Fire Emblem Engage — Outfit, Accessory and Menu Enhancements v0.2

[简体中文](README.md)

Open the accessory shop directly from battle preparations for live 3D dressing. Confirm equipment and return to refresh the battlefield appearance before battle. v0.2 retains seven categories, equipment summaries, Engage outfit preferences and ZL/ZR paging, with plugin diagnostics off by default.

This project is modified and extended from **SierraSak**’s [EngageAcc_Code](https://github.com/SierraSak/EngageAcc_Code) / [Expanded Accessory Slots](https://gamebanana.com/mods/512066). Thank you to SierraSak for the original accessory-slot implementation and resources. Characters use seven available accessory categories. Mask 16 is not exposed as a character slot. The upstream eight-entry storage layout and version 1 save format are retained.

**Install [Cobalt](https://github.com/Raytwo/Cobalt) first.** Thank you to **Raytwo** for developing and maintaining Cobalt and providing the mod-loading foundation used by this project.

The current binary targets Fire Emblem Engage 2.0.0 with Cobalt 1.31.0 as its compatibility baseline.

## v0.2: Accessory shop in battle preparations

### Entry location

Before starting battle, open **Somniel → Accessory Shop** in the preparation menu. The new entry sits **below the Smithy and above Skill Inheritance**, so dressing does not require returning to the Somniel. Entry text supports Simplified Chinese, Traditional Chinese, Japanese and English.

The user-provided image 01 marks the new entry with an arrow. It shows the menu location; live preview is available after entering the shop.

![Battle preparations: accessory shop entry in the Somniel submenu](images/sortie-accessory-shop-entry.jpg)

### Dressing and preview

1. Enter the accessory shop, select buying or changing outfits, and choose a character.
2. Move the cursor through outfits or accessories for live preview. Previewing does not confirm equipment.
3. In lists supporting preview controls, use the right stick to rotate the character and adjust viewing distance.
4. Confirm equipment and exit. Inspect the refreshed battlefield appearance before starting battle.

The preview uses the native accessory-shop scene, camera, lighting and blue background. The previous character remains until the replacement finishes loading. Battlefield health bars, portraits, class markers and related overlays are hidden in the shop; the battlefield interface and camera are restored on exit.

The final v0.2 design uses native fades. Entry fades out the battlefield and waits for the first shop character before fading in. Exit fades out the shop and waits for changed battlefield actors to finish loading before fading the battlefield back in. Each fade takes about **0.25 seconds**; asset loading adds its own waiting time.

### Battlefield refresh on return

On exit, the plugin compares original and final equipment and refreshes only **player characters with existing battlefield actors whose final equipment changed**. All seven visible categories are compared: outfit, head, face, back, battle outfit, hair color and style. Browsing without confirming, or restoring the original selection after changes, causes no extra refresh.

Ordinary confirmed preparation outfits apply through the return refresh without redeploying. Appearance still follows the game's and installed outfit mods' rules: products need corresponding resources, while transformations and Engage states have their own display conditions.

This entry is for **preparations before battle starts**. The Engage outfit preference available during battle is a separate feature below.

## v0.1 features retained in v0.2

- Seven character categories: outfit, head, face, back, battle outfit, hair color and style (Mask 1/2/4/8/32/64/128). Mask 16 is not available as a character slot; the eight-entry storage layout and version 1 save format are preserved.
- Seven menu categories, purchase filters and current-equipment summaries, with unequipped summary rows hidden.
- Adds ZL/ZR page scrolling to ring, roster and preparation selection, arenas, cooking selection, support and bond lists, outer item selection, materials and valuables, shop selection and purchases, forging, engraving, exchanges and photo-mode weapon lists. Two-column lists retain the selected column. Inherited-skill and inventory lists gain paging only in states without native ZL/ZR actions; native help and category controls are preserved.
- Camera adjustments for head accessories and alternate styles.
- Engage outfit preference: current character outfit or emblem outfit. Re-engage or redeploy to update an already-engaged map model.

## v0.2 logging: off by default, manually enabled when needed

The v0.2 plugin **does not create or append its diagnostic log by default**. No configuration is needed for normal use. Missing or unreadable flags, or contents other than `on`, keep logging off. Dressing, menus and battlefield refresh still work. This switch controls this plugin only, not Cobalt, emulator or other mod logs.

### Enable logging again

1. Fully exit the game; stop the current game process when using an emulator.
2. Create the plain-text file **`fee-outfit-menu-debug.flag`** in the SD card's `engage` folder. Ensure its actual name does not end in `.flag.txt`.
3. Save as UTF-8 text containing this single line:

   ```text
   on
   ```

4. Fully restart the game and reproduce the operation under investigation.

The flag is **`sd:/engage/fee-outfit-menu-debug.flag`**. Logs append to **`sd:/engage/fee-outfit-menu-debug.log`**. In an emulator, use its configured virtual SD card directory, for example `<virtual SD card>/engage/fee-outfit-menu-debug.flag`, rather than the game mod folder.

Records cover the preparation entry and shop lifecycle, preview requests and model completion, scene and camera restoration, transition waits, actor refresh and seven-category equipment differences, plus language, Engage outfit settings, paging and errors. Records depend on the execution paths actually taken.

### Disable logging again

Delete `fee-outfit-menu-debug.flag` or change its contents to **`off`**, then **fully restart the game**. The value is cached after its first read in the process, so changing the file during play does not apply immediately. Logging stops on the next start. Existing `fee-outfit-menu-debug.log` files are retained as investigation evidence; back up or remove old logs after exiting if desired.

## v0.1 screenshots and settings retained in v0.2

Multiple accessories, equipment summaries and the Engage outfit preference described below were introduced in v0.1 and remain available in v0.2.

The original game allows up to two accessories. With these enhancements, a character can equip multiple accessories simultaneously, with clear entries in the equipment summary on the left. The screenshot shows multiple equipped accessories and the category menu.

![Multiple equipped accessories and the equipment summary](images/multiple-accessories-equipment-summary.jpg)

### Engage Outfit Setting

Under **System → Settings**, choose the Engage outfit preference: **current character outfit / emblem outfit**.

You can also switch the setting during battle and observe its effect. However, map models that are already engaged require re-engaging or redeploying before the change takes effect. Special transformations, such as dragon transformation, retain their existing rules.

![Settings during battle: current character outfit selected for Engage](images/engage-outfit-settings-in-battle.jpg)

## Download and Installation

Plugin file: [`engage_outfit_menu_enhancements.nro`](Releases/engage_outfit_menu_enhancements.nro); checksums: [SHA256SUMS](Releases/SHA256SUMS).

1. Install the prerequisite runtime using Cobalt's instructions.
2. Place the NRO in `engage/mods/engage-outfit-menu-enhancements/` and fully restart the game.
3. If the outfit integration package already contains this plugin, use that included copy and avoid installing another standalone copy.

## Source and Build

See [Build instructions](docs/BUILD.md). This snapshot includes the current implementation and the local dependencies used by it. New builds go to `dist/`, preserving the release binary in `Releases/`.

## License and Credits

Project-owned additions and documentation use the [GNU GPL v3.0](LICENSE), and combined-program distribution must comply with GPLv3. Third-party dependencies, upstream-derived code and resources retain their respective license terms and notices; see [THIRD_PARTY.md](THIRD_PARTY.md) and [NOTICE](NOTICE).

Thanks to [Raytwo](https://github.com/Raytwo/Cobalt), [SierraSak](https://github.com/SierraSak/EngageAcc_Code), [DivineDragonFanClub](https://github.com/DivineDragonFanClub/engage-il2cpp), [skyline-rs](https://github.com/skyline-rs) and all relevant dependency contributors.

## v0.2 validation

On 2026-10-03, the user confirmed that the current v0.2 functionality passed testing, including the preparation accessory-shop entry, native-scene live dressing, overlay hiding and restoration, shop transitions and battlefield appearance refresh on return. Build and static checks include Switch compilation, NRO structure, full ZIP CRC and unchanged-member checks. See the [Changelog](CHANGELOG.md).
