# Porting notes

Every behaviour difference from the originals, with the reason. Behaviour of the originals: [ANALYSIS.md](ANALYSIS.md).

| # | Original behaviour | This mod | Reason / evidence | Status |
| --- | --- | --- | --- | --- |
| 1 | Mod id `arrowtrails` / `real_arrow_tip` | One mod, id `dctrxds_archer_addons` (the requested `dctrxds-archer-addons` has hyphens) | Horizon ids are `[a-z0-9_]{1,64}` (`horizon-mods/src/manifest.rs:224-233`); test `the_requested_hyphenated_id_is_not_a_horizon_id` | Final |
| 2 | Arrow Trails: all trails, per-kind settings, delayed rendering | Not implemented | A `.hmod` runs no code and cannot draw (CAPABILITY_MATRIX rows 1, 2) | Deferred (ROADMAP Phase 3) |
| 3 | Arrow Trails removes vanilla crit and spectral particles, always | Vanilla particles stay | Needs code; also a visible change the original makes even with trails disabled | Deferred, and proposed to keep vanilla particles |
| 4 | Real Arrow Tip: bow shows the arrow the holder would shoot | Not implemented | No vanilla item-model property for it; needs code (row 5b) | Deferred |
| 5 | Real Arrow Tip: crossbow models per projectile and multishot | Not implemented | A `.hmod` cannot replace `assets/minecraft/items/crossbow.json` (`content.rs:714-724`) | Waits on upstream proposal 2a (Phase 2) |
| 6 | Real Arrow Tip: coloured head on tipped arrow entities | Not implemented | Horizon picks the arrow texture in code with no hook (`creatures.rs:7426-7437`) | Deferred |
| 7 | Config files and screens (MidnightLib, YACL + Mod Menu); per-feature switches | Only the Mods page on/off switch for the whole mod | A `.hmod` has no settings API (row 6, 7). The per-feature switches the architecture asks for cannot exist yet | Deferred |
| 8 | Real Arrow Tip's textures | Not used; will be redrawn | They appear to contain Mojang artwork (RISKS.md) | Final |
| 9 | Original mods add nothing to "permissions" | The Mods page shows "may add blocks, items, mob effects and entity types to your worlds" | `cargo horizon-mod build` requires capability `registries` for every content mod (`cargo-horizon-mod/src/main.rs:215-217`); this mod adds none of them | Final until Horizon changes the wording |
