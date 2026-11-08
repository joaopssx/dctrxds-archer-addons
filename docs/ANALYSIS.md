# Analysis

Investigation for "DctrXD´s Archer Addons", an unofficial combination of Arrow Trails (Biverom, MIT) and
Real Arrow Tip (ToBinio, MIT), targeting Horizon as a `.hmod`. Nothing here is implemented yet.

Every claim is tagged:

- **[VERIFIED]**: read in code or docs at the revision listed below, or shown by a command output.
- **[ASSUMED]**: inferred, not proven. Needs an experiment before anyone relies on it.

## Glossary

| Term | Meaning |
| --- | --- |
| `.hmod` | Horizon's content mod file: a zip with `horizon-mod.toml`, `assets/` and `data/`. |
| `.hnmod` / native mod | Rust source patched into the game and compiled by the launcher. Not sandboxed. Out of scope by project rule. |
| WASM / WebAssembly mod | The old sandboxed code format (`mod.wasm` inside a `.hmod`). Removed from Horizon. |
| WIT | The interface definition language that described the old WASM API (`horizon:mods`). |
| Capability | A permission a mod declares in its manifest (`registries`, `hud`, `render`...). |
| Namespace | The `x` in `x:path` resource ids. `minecraft` is vanilla's. |
| Item model definition | `assets/<ns>/items/<item>.json`: a tree (`select`, `condition`, `range_dispatch`, `model`) that picks which model an item stack draws. |
| Select / condition property | The value a `select` or `condition` node reads (for example `minecraft:charge_type`). |
| Tint source | A rule that colours a model layer (for example `minecraft:potion`). |
| Mixin | A Java patch applied to Minecraft classes at load time (Fabric/NeoForge). Has no equivalent in a `.hmod`. |
| Billboard | A flat shape turned to face the camera. |
| Partial tick | The fraction (0 to 1) of the way between two game ticks at which a frame is drawn. |

## Step 1: environment

Workspace: `C:\Users\jpsen\Documents\code\rs\dctrxd-ports` (called `./` below).

| Path | Revision | Origin |
| --- | --- | --- |
| `./dctrxds-archer-addons` | `8e04a77fa8882c056544d3f5e2a6e9761a596cbb` (2026-11-08) | github.com/joaopssx/dctrxds-archer-addons |
| `./horizon` | `41f822b659789780f58d32c63ddfb6887ea1b8bf` (2026-11-06) | github.com/horizon-rust/horizon |
| `./horizon-mods` | `d2de4c18de0cfc4c1e4f193f2ec09b47adb0d236` (2026-11-06) | github.com/horizon-rust/horizon-mods |
| `./horizon-mixins` | `5a64af938ec86b00d53d16a408aabc0038fb3e57` | not used (native) |
| `./horizon-native-mod-template` | `cc8ce2d896782099abfa9f0abf26fa80c9e4bddb` | not used (native) |
| `./dctrxds-jei`, `./JustEnoughItems` | other projects | not used |
| `./arrowtrails` | **missing** | cloned to a scratch folder for reading: `d94f3866bbf33241e57e1166f7bd5351dbded68f` (2025-09-17), github.com/Biverom/arrowtrails |
| `./real-arrow-tip` | **missing** | cloned to a scratch folder for reading: `e2033a0d81e91a218bb0fce13f2aefc45abe3f6b` (2026-09-16), tangled.org/tobinio.dev/real-arrow-tip (the official source cloned fine; the GitHub mirror was not needed) |

Tools [VERIFIED by command output]:

| Tool | Result |
| --- | --- |
| `rustc` | 1.95.0 |
| `cargo` | 1.95.0 |
| `python3` | **not found** (Windows Store alias). `python` is 3.14.4. Horizon's tools are documented as `python3 ...`; run them as `python ...` here. |
| `git` | 2.56.0.windows.2 |
| `cargo horizon-mod` | **not installed** (`error: no such command`). Install: `cargo binstall --git https://github.com/horizon-rust/horizon cargo-horizon-mod` (prebuilt) or `cargo install --git https://github.com/horizon-rust/horizon cargo-horizon-mod` (source), per `horizon/docs/mods.md:39-43`. Not installed by me. |

The two original mods should be cloned into `./arrowtrails` and `./real-arrow-tip` at the revisions above so later work
reads the same source.

## Step 2: what a `.hmod` can do today

### Finding 1: a `.hmod` runs no code [VERIFIED]

- `horizon/docs/mods.md:5-8`: "Content mod | `.hmod` | Data and assets: blocks, items, recipes, loot, tags, models,
  textures, structures. **No code.**" Code is native (`.hnmod`).
- `horizon/docs/mods.md:55-57`: `cargo horizon-mod build` "refuses a folder with a `Cargo.toml` (code is native now)
  and a manifest without capability `registries`."
- `horizon/docs/api/mod-files.md:24`: "`mod.wasm` | a legacy WebAssembly mod: it doesn't run any more".
- `horizon/crates/horizon-client/src/game/mods.rs:1-3`: "Code mods are native (`native_hooks.rs`); WebAssembly code
  mods don't run any more." Line 396: a WASM mod's row reads "Doesn't run any more: needs a native port".
- `horizon/crates/horizon-mods/src/index/native.rs:9`: `WASM_GONE`.
- `horizon/launcher/src/browser/horizon.rs:794`: the launcher refuses to install them: "needs a native port".

The old sandboxed API still exists as types only: `horizon/crates/horizon-mods/src/manifest.rs:79-106` lists
capabilities `game-read`, `chat`, `hud`, `storage`, `command`, `registries`, `ui`, `render`, `input`, `players`,
`world`, `recipes`, and `horizon/crates/horizon-mods/src/api_level.rs:16-63` lists 46 WIT functions (`render.set-hud`,
`storage.get`...). None of them is callable today, because the runtime that executed `mod.wasm` is gone. Even when it
ran, that list had no function to enumerate entities or draw in the world (only HUD, corner, minimap, zoom, far
terrain).

History [VERIFIED in `git log`]: removed on 2026-10-29 (`e7614d219`, `0c9a49390`). The design note of 2026-11-05
(`docs/design/mod-hotload.md:155,168-170`) expects a wasm runtime from a contributor's fork (arismoko,
`horizon-mod-wasm`, "one API with native") to merge upstream later. Not merged at `41f822b`.

**Stale statement:** `horizon-mods/README.md` ("Sandboxed `.hmod` mods run in Horizon's sandbox, with no file, network
or clock access, and can only do what their page lists") describes the removed WASM runtime. The mission brief was
written from it. The game's code is the authority here.

### Finding 2: a `.hmod` cannot replace vanilla's files [VERIFIED]

`horizon/crates/horizon-mods/src/content.rs:714-724`: a file is allowed only in the mod's own namespaces, in
`data/minecraft/tags/` and `data/c/tags/`, or in `assets/minecraft/atlases/`. Anything else is refused with "a mod
adds files in its own namespaces ..., to vanilla and convention (c) tags, or to vanilla's atlases". `minecraft`, `c`
and `horizon` cannot be claimed (`manifest.rs:25`, `manifest.rs:286-289`). So a `.hmod` cannot ship
`assets/minecraft/items/bow.json` or `crossbow.json`, which is the whole data half of Real Arrow Tip.

A `.hmod` also cannot change a vanilla item's `minecraft:item_model` component: items in `data/<id>/horizon/item/`
are new items copying a template (`docs/mods.md:210-224`), not patches of vanilla items.

### What a `.hmod` does offer [VERIFIED, `docs/mods.md:66-478`]

- Manifest `horizon-mod.toml`: `id` (`[a-z0-9_]{1,64}`, checked in `manifest.rs:224-233`), `name`, `version`,
  `description`, `authors`, `api = "0.2"`, `side` (`client` default, `both`, `server`), `capabilities = ["registries"]`,
  `namespaces`, `experimental`. Unknown keys are refused (`#[serde(deny_unknown_fields)]`, `manifest.rs:33`).
- New blocks, items, creative tabs, mob effects, entity types (thrown items, three mob templates) and menu types under
  `data/<id>/horizon/`.
- Assets in its own namespace (models, textures, lang, sounds), tag additions, atlas source additions, worldgen data,
  damage types.
- "Permissions" are the capabilities. For a content mod the only one that matters is `registries`; the Mods page
  shows each as "May ..." before the player turns the mod on (`docs/mods.md:22-23`, `Capability::describe`).

**Consequence: `id = "dctrxds-archer-addons"` is invalid** (hyphens). The manifest id must be
`dctrxds_archer_addons`. The repository and folder can keep the hyphen.

### How the mod's page is described [VERIFIED, `docs/api/mod-index.md`, `docs/mods.md:495-537`]

- Source in a public GitHub repository, licence reviewers can read (MIT is on the list, `horizon-mods/MOD-POLICY.md`).
- Each version is a GitHub release with the file as an asset, optionally with a `.minisig`.
- First listing: the form at playhorizon.dev/mods/submit opens a PR adding `horizon-mods/mods/<id>.toml`; a reviewer
  reads the source and merges.
- Page fields: `name`, `summary`, `description` (Markdown), `authors`, `license` (SPDX), `source`, `icon`,
  `categories` (1-3), `tags` (max 10), `screenshots` (max 8), `key` (minisign public key). They can live in a
  `horizon-registry.toml` at the repository root; the registry entry wins.
- `side` and `capabilities` are read from the manifest by CI, "never typed by hand".

### What exists only for native mods (out of scope, listed so the gaps are precise) [VERIFIED]

From `horizon/docs/native-mods.md` and `horizon/native-hooks.json` (432 hooks):

- `world_primitives` (`native-mods.md:3315-3342`): `line(from, to, color, width)` in world coordinates, ARGB, width in
  pixels; drawn "after the entities and before the first-person hand", blended, no depth write.
- `item_model_property` (`native-mods.md:37-40`, `item_draw.rs:27-35`): a mod-supplied boolean **condition** property.
  There is no mod-supplied **select** property and no mod-supplied **tint source** in `ItemEnv`
  (`horizon-assets/src/item_model.rs:993-1031`).
- `register_configs`, `config_loading`, settings hooks (`docs/mods.md:487-493`).
- `mob_submit` (`native-mods.md:2829-2833`): "every mob in view but players". Arrows are not mobs.
- Entity, projectile and level-tick hooks (`entity_tick_post`, `projectile_launch`, `client_level_load`...).

### Horizon's vanilla item-model support, relevant to Real Arrow Tip [VERIFIED]

`horizon/crates/horizon-assets/src/item_model.rs`:

- Select properties (`:707-722`): `custom_model_data`, `main_hand`, `charge_type` (values `none`/`rocket`/`arrow`,
  `:757-760`, computed from `minecraft:charged_projectiles`, `:1403`), `trim_material`, `block_state`,
  `display_context`, `local_time`, `context_entity_type`, `context_dimension`, `component` (`:1452`, returns the raw
  component value).
- Condition properties (`:660-679`): includes `using_item` and `component`; the `minecraft:enchantments` component
  predicate is implemented (`item_model/component_predicate.rs:6-40`), so "crossbow has Multishot" is expressible.
- Tint sources (`:635-647`): `constant`, `custom_model_data`, `dye`, `grass`, `firework`, `potion` (reads the drawn
  stack's own `potion_contents`, `:1653-1657`), `team`.
- Resource packs may override `items/` and `models/item/` ("works", `docs/resource-packs.md:40`).

### Horizon's arrow entity rendering [VERIFIED]

`horizon/crates/horizon-client/src/game/creatures.rs:7426-7437`: the arrow texture is chosen in code:
`arrow_spectral` if spectral, `arrow_tipped` if `effect_color > 0`, else `arrow`. No tint by `effect_color`, and no
hook (content or native) in that path.

## Step 3: the original mods

### Arrow Trails (Biverom, MIT, Minecraft 1.21.1, Fabric + NeoForge)

Architecture [VERIFIED, `common/src/main/java/com/biverom/arrowtrails/`]:

| Class | Role | Port? |
| --- | --- | --- |
| `mixin/AbstractArrowMixin` | Adds a `Deque<Vec3> trailPoints` to every arrow; at the start of `tick` pushes the current position to the front; at the end trims to the configured length. Also redirects `Level.addParticle` inside `AbstractArrow.tick` to nothing (removes vanilla crit particles), always. | Behaviour yes, mechanism no |
| `mixin/SpectralArrowMixin` | Removes the spectral arrow's particles, always. | Behaviour, optional |
| `mixin/ArrowRendererMixin` | After `ArrowRenderer.render`: if the trail should draw, builds `points = [interpolated position] + trailPoints` and calls `TrailRenderer.drawTrail`. | Behaviour yes |
| `TrailRenderer` | Geometry: a camera-facing ribbon of quads with per-vertex colour, alpha and width. "Delayed" mode queues trails and draws them later. | Algorithm yes |
| `mixin/LevelRendererMixin` | Calls `drawDelayedTrails` in `LevelRenderer.renderLevel` right before `renderDebug`. | Concept only |
| `ModRenderTypes` | Render type: `NEW_ENTITY` format, quads, entity translucent shader, `white.png`, translucent blending, no culling, lightmap, `COLOR_WRITE` only (no depth write), depth test on. | Concept only |
| `ArrowTrailsConfig` | MidnightLib config (file + screen). | Values yes, library no |
| `TrailSettings` | Record: length, start/end colour, alpha, width. | Yes |
| `MixinMinecraft`, `platform/*`, `fabric/`, `neoforge/` | Template leftovers and loader glue. | Discard |

Expected behaviour, in verifiable terms [VERIFIED from code]:

1. Each client tick, every arrow entity records its position at the start of the tick (newest first) and keeps at
   most `L` points, where `L` is the length for its kind. The trail therefore covers the last `L` ticks.
2. Kind selection, first match wins: spectral arrow; arrow with potion colour `!= -1` ("tipped", colour = the
   potion colour for both ends); critical arrow; plain arrow; anything else uses the plain arrow settings.
3. Each frame the trail has `L + 1` points: the arrow's interpolated position, then the stored points; `L` segments.
4. Point `i` (0 = arrow) has parameter `t = (i + partialTick) / (L + 1)`. Width, colour (RGB, linear per channel) and
   alpha are `lerp(t, start, end)`.
5. Each point is offset by `±(dir × viewDir).normalize() * width / 2`, where `dir` points to the next point (the last
   point uses the previous segment's direction) and `viewDir` points from the point to the camera entity's position.
   Each segment is one quad `left[i], left[i+1], right[i+1], right[i]`.
6. Lit with the arrow's packed light; no texture (white).
7. Drawn while not in the ground, or while in the ground for fewer than `L` ticks. Stuck arrows keep pushing the same
   position, so the trail visibly shrinks into the arrow over `L` ticks, then stops.
8. Fewer than 2 points: nothing drawn. `disableTrails`: nothing drawn (particles stay suppressed).

Defaults [VERIFIED, `ArrowTrailsConfig.java`]:

| Kind | Length (1-64) | Start colour | End colour | Start/End alpha | Start/End width |
| --- | --- | --- | --- | --- | --- |
| Arrow | 8 | `#ffffff` | `#ffffff` | 0.25 / 0.0 | 0.2 / 0.1 |
| Critical | 16 | `#ffe8a8` | `#ffb055` | 0.5 / 0.0 | 0.3 / 0.15 |
| Spectral | 16 | `#fcfc76` | `#fcfc76` | 0.5 / 0.0 | 0.3 / 0.15 |
| Tipped | 16 | potion colour | potion colour | 0.5 / 0.0 | 0.3 / 0.15 |
| Advanced | `delayedRendering = true`, `disableTrails = false` | | | | |

Works on any server because it only reads what the server already syncs to clients (position, the critical flag, the
arrow's colour) [ASSUMED from vanilla's entity data; consistent with the code reading only client-side state].

Note: the option the brief calls "Deferred Rendering" is named "Delayed Rendering" (`delayedRendering`) in the code
and lang file, and it is **on by default**.

#### Why "Delayed Rendering" exists

What the code proves [VERIFIED]: without it, the quads go into the entity buffer source while each arrow is drawn,
with a translucent, depth-tested, no-depth-write render type. With it, they are queued and drawn into the level's
buffer source just before `renderDebug`, near the end of `renderLevel`.

Why that fixes invisible trails [ASSUMED, from how 1.21.1's pipeline is generally known to work; not checked in
bytecode]: translucent geometry that writes no depth can be overwritten or composited away by what is drawn after it.
In "Fabulous" graphics, and with shader mods such as Iris, entity translucency goes to separate targets that are
merged later, and some setups drop or hide geometry submitted through a custom render type during the entity pass.
Drawing the trails late, on the main target, after the world's translucent layers, sidesteps all of this.

What it means in Horizon [VERIFIED for the native API only]: Horizon's renderer is wgpu (Vulkan/DX12/Metal), with
reversed depth and TAA jitter. The only existing way to draw lines in the world, the native `world_primitives`, is
already "after the entities and before the first-person hand", blended, no depth write, depth-tested unless
`.through_walls()`. That is the delayed mode by construction, so a port would not need the option. Whether those
primitives are drawn after translucent terrain, particles and weather is not stated (see OPEN_QUESTIONS.md). For a
`.hmod` the question is moot today: it cannot draw anything.

### Real Arrow Tip (ToBinio, MIT, Fabric, this revision targets 26.3)

`gradle.properties` at `e2033a0`: `minecraft_version=26.3`, `mod_version=0.3.1+mc26.3`, YACL + Mod Menu for config.

Architecture [VERIFIED, `src/main/java/tobinio/realarrowtip/`]:

| Class / file | Role | Port? |
| --- | --- | --- |
| `RealArrowTip` | Registers select property `real_arrow_tip:charge_type` and tint source `real_arrow_tip:projectile`. Helper: colour = opaque `PotionContents.getColor()`, or -1. | Behaviour yes |
| `model/ChargeTypeProperty` | Bow: the projectile the holder would shoot (`user.getProjectile(bow)`; skeletons: the arrow they would actually fire). Crossbow: the first of `charged_projectiles`, or `none`. A plain arrow with a potion colour reports `minecraft:tipped_arrow` (strays and bogged). Value = the item id. | Behaviour yes |
| `model/TintSource` | Bow: the would-be projectile's potion colour (the holder exists) or the default. Crossbow: the first charged projectile with a colour, or the default. | Behaviour yes |
| `assets/real_arrow_tip/items/bow.json`, `crossbow.json` | Item model trees using those two (see below). | Data yes, after redesign |
| `assets/real_arrow_tip/models/item/*.json` (12) | Layered models over `item/bow` / `item/crossbow`. | Data yes |
| `assets/real_arrow_tip/textures/**` (20 PNGs, 16x16 and 32x32) | Bow/crossbow overlays, entity tipped arrow base and head. | See RISKS.md |
| `mixin/ClientItemInfoLoaderMixin` | When loading item definitions, reads `real_arrow_tip/items/<x>.json` in place of `minecraft/items/<x>.json` unless `<x>` is in `ignoredItems`. | Concept only |
| `mixin/TippableArrowRendererMixin`, `ArrowEntityRenderingStateMixin`, `ArrowRendererMixin` | Entity: swaps the tipped arrow texture for the mod's base texture, then draws the arrow model again with `tipped_arrow_head.png` (cutout) tinted with the arrow's colour. | Behaviour yes |
| `config/*` | YACL JSON5 file `real_arrow_tip.json5`: `renderCustomArrowEntity = true`, `ignoredItems = []`. Screen via Mod Menu. Category title says "Requires Game Restart". | Values yes |
| `item-model-generator.py` | Generates the two item JSONs from a list of arrows. | Idea, as a dev tool |

Expected behaviour [VERIFIED from code and JSON]:

1. Bow, not in use: vanilla model. In use: `select real_arrow_tip:charge_type`
   - `minecraft:spectral_arrow`: spectral pulling models 0/1/2, chosen by `use_duration * 0.05` thresholds 0.65 and 0.9;
   - `minecraft:tipped_arrow`: tipped pulling models 0/1/2 with layer 0 (head) tinted by the projectile's colour,
     default `0xFF385DC6`;
   - anything else: vanilla's pulling models.
2. Crossbow: `select charge_type` over `arrow`, `firework_rocket`, `spectral_arrow`, `tipped_arrow`; each case is
   `condition component minecraft:enchantments [{enchantments: minecraft:multishot}]` choosing a multishot model
   (three projectiles drawn) or the single one; tipped is tinted like the bow; `none` falls back to vanilla's
   charging/pulling tree. Multishot is decided by the crossbow's enchantment, not by the number of charged projectiles.
3. Tipped arrow entity: base texture without coloured head, plus a head layer multiplied by the arrow's colour.
   Arrows with colour -1 are unchanged. Toggle `renderCustomArrowEntity` (read at class init, so a restart applies it).
4. Client only; works on any server (reads synced item components and entity colour) [ASSUMED, as for Arrow Trails].

Licence notes [VERIFIED]: Real Arrow Tip's `LICENSE` reads "Copyright (c) 2023 " with no holder name. Source headers
say `@author Tobias Frischmann`; commits are by `tobinio`/`ToBinio`/`to_binio`. Arrow Trails' `LICENSE`:
"Copyright (c) 2025 Biverom".

## Experiments run

None in the game. The only actions were reading files, running version commands, and cloning the two original
repositories into a scratch folder outside the workspace. No Horizon build, no `.hmod` build, no game run. Nothing in
these documents is claimed to work in the game.
