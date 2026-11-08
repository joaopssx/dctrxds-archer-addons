# Capability matrix (`.hmod` only)

Status: **OK** = possible in a `.hmod` today; **PARTIAL** = some of it; **MISSING** = not possible.
Evidence is in [ANALYSIS.md](ANALYSIS.md). The root cause of most rows is one fact: **a `.hmod` runs no code**
(`horizon/docs/mods.md:5-8`, `horizon/crates/horizon-client/src/game/mods.rs:1-3`).

| # | Feature | Original uses | `.hmod` offers today | Status | Proposed solution / API that would have to exist |
| --- | --- | --- | --- | --- | --- |
| 1 | Detect arrow entities (kind, position, velocity, flying/stuck) | Mixin on `AbstractArrow.tick`, fields `inGround`, `inGroundTime`, `isCritArrow`, `getColor` | Nothing. No code, no entity query. | MISSING | A client code API: per tick, the loaded arrows with `id`, `kind` (arrow / spectral / trident), `position`, `velocity`, `in_ground`, `in_ground_ticks`, `critical`, `effect_color`. Plus "entity removed". |
| 2 | Draw own geometry in the world (gradient lines, transparency, right order) | Custom `RenderType` (translucent, no depth write), delayed draw before `renderDebug` | Nothing. | MISSING | A client code API to submit world-space geometry each frame, with per-vertex ARGB: either a triangle strip / quad list, or a segment with two end colours and two end widths. The native `world_primitives.line` is close but has one colour per line and a width in pixels (ribbon taper would be approximated). Drawn after translucent layers, blended, depth-tested, no depth write. |
| 3 | Change bow/crossbow appearance by loaded arrow | Mixin swapping `minecraft/items/{bow,crossbow}.json` for its own; custom select property `charge_type` | Own-namespace models and textures only. Cannot override `assets/minecraft/items/*` (`content.rs:714-724`). Cannot change a vanilla item's `item_model`. | MISSING | (a) Data: let a client-side content mod override vanilla client assets as a resource pack does. (b) Code: a mod-supplied **select** property (the native hook is condition-only, `item_model.rs:995`). |
| 4 | Coloured tip on the arrow entity | Mixins on `TippableArrowRenderer`/`ArrowRenderer`: base texture + tinted head layer | Nothing. Horizon picks the texture in code (`creatures.rs:7431-7437`), no tint, no hook. | MISSING | Either a vanilla-shaped data hook (unlikely: vanilla has none) or a code API "draw extra layer on this arrow" / an upstream Horizon option. Even native mods lack this today (`mob_submit` excludes non-mobs). |
| 5 | Read crossbow charge state (multishot, spectral, tipped) | `charged_projectiles` component, enchantment predicate | Vanilla item-model nodes support `charge_type` (none/arrow/rocket), `component` select (raw value), and the `minecraft:enchantments` predicate (`item_model.rs:707-760,1403,1452`; `component_predicate.rs`). But a `.hmod` cannot attach them to the vanilla crossbow (row 3). | PARTIAL (engine yes, `.hmod` no) | Same as row 3a. With 3a, multishot is solved by data; spectral vs plain might be solvable by `select component minecraft:charged_projectiles` exact-match (unverified, see OPEN_QUESTIONS.md). Tipped colour from the projectile needs a tint source that reads `charged_projectiles`, which vanilla does not have: code. |
| 5b | Bow: which arrow the player would shoot | `LivingEntity.getProjectile(bow)` | Vanilla has no item-model property for it. | MISSING | Code: a select property (or N boolean conditions) answered by the mod, plus a tint source answered by the mod. |
| 6 | User configuration and persistence | MidnightLib / YACL files in `config/` | Nothing: no code to read a setting. The old `storage` capability belonged to the removed WASM runtime. | MISSING | A persisted key-value store owned by the game for the mod (the game writes the file, the mod never touches the filesystem), or a declarative settings schema in the manifest that the game stores and exposes to data (for example as a condition property). |
| 7 | Config screen | MidnightLib / YACL + Mod Menu | Options > Mods shows only an on/off switch per mod (`docs/mods.md:22-23`). | PARTIAL (on/off only) | A declarative settings page (toggles, sliders, colour fields) rendered by the game from the manifest, tied to row 6. |
| 8 | Own resources (textures, models, lang) in the `.hmod` | Jar resources | `assets/<id>/...` loads: textures, models, lang (`docs/mods.md:371-399`). | OK | Use namespace `dctrxds_archer_addons`. Only useful once something references them. |
| 9 | Performance inside the sandbox | Java, per-arrow deque, one quad per segment | No code, so no cost and nothing to measure. | N/A | For a future code API: one call per frame carrying all trails (batched), not one call per segment. Rough budget: 100 arrows x 16 segments = 1,600 quads per frame [ASSUMED fine for wgpu; unmeasured]. Trail state can live in the mod; the game sends only arrow snapshots per tick. |
| 10 | Remove vanilla crit/spectral particles (Arrow Trails does this always) | `@Redirect` of `addParticle` | Nothing. | MISSING | Code: a particle filter hook, or keep vanilla particles (simplest; recommended). |
| 11 | Per-item opt-out (`ignoredItems`) | Config + loader mixin | Nothing. | MISSING | Falls out of rows 3 and 6. |

## Display name

`name = "DctrXD´s Archer Addons"` is a TOML string; the manifest parser accepts any UTF-8 there
(`manifest.rs:36`). Whether Horizon's in-game font and the launcher render U+00B4 is **not verified** (no game was
run). See OPEN_QUESTIONS.md.

## Summary

Of 11 rows: 1 OK (own resources, which have nothing to attach to), 2 PARTIAL (the engine supports the crossbow's
vanilla properties but a `.hmod` cannot use them on the vanilla crossbow; the Mods page has an on/off switch),
the rest MISSING. **No user-visible feature of either original mod can be delivered as a `.hmod` today.**
