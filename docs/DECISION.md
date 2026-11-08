# Decision

Status: **waiting for the owner's choice.** Nothing is built until then.

## The blocker

The project rule is: `.hmod` only, sandboxed. The brief assumed a `.hmod` can run sandboxed code. At Horizon
`41f822b` it cannot: the WebAssembly runtime was removed and a `.hmod` is data and assets only, confined to its own
namespaces (evidence: [ANALYSIS.md, Step 2](ANALYSIS.md#step-2-what-a-hmod-can-do-today)). Every visible feature of
both mods needs either code or replacing vanilla files ([CAPABILITY_MATRIX.md](CAPABILITY_MATRIX.md)). Per the rule,
the format is not changed here; the options are laid out instead.

## Options

### Option 1: deliver the possible subset now

What can be delivered: a `.hmod` with manifest, lang file, licence notices and the mod's own textures. It loads and
shows on Options > Mods with an on/off switch that changes nothing in the game.

- Proves: the build (`cargo horizon-mod build`), `cargo horizon-mod check`, and that the name renders.
- Does not prove anything about arrows. It should **not** be published: a mod that does nothing would mislead
  players and reviewers.

### Option 2: contribute the missing API upstream

Three candidates, from smallest to largest:

**2a. Client asset overrides in content mods (data only).** Let a `side = "client"` content mod carry
`assets/minecraft/items/`, `models/` and `textures/` files that apply like a resource pack layered above vanilla and
below the player's own packs. Today `content.rs:714-724` refuses them on purpose ("Vanilla's own files stay
vanilla's"), so this is a design change: it starts as a suggestion/issue to the maintainers, not as a PR.
If accepted, it unlocks only part of Real Arrow Tip:

| Unlocked by 2a | Still needs code |
| --- | --- |
| Crossbow: multishot models (enchantment predicate exists) | Bow: which arrow the player would shoot |
| Crossbow: firework multishot | Tipped colour from the loaded projectile (no vanilla tint source) |
| Crossbow: spectral vs plain, **if** `select component` exact-match works on `charged_projectiles` (unverified) | Coloured tip on the arrow entity |
| Generic tipped look (fixed colour) | All of Arrow Trails; settings |

Size [ASSUMED]: the fence rule, the resource-layer order, tests and docs; plausibly under 500 lines, but only the
maintainers can say whether they want it.

**2b. A sandboxed code runtime again.** Everything in the matrix "MISSING" column needs it. History [VERIFIED in
`git log`]: the maintainer removed the WebAssembly runtime on 2026-10-29 (`e7614d219` "Game: WebAssembly code mods
don't run any more", `0c9a49390` "Crates: remove the WebAssembly mod runtime..."). A design note from 2026-11-05
(`e95658709`, `docs/design/mod-hotload.md:155,168-170`) then lists "E. wasm mods (arismoko's fork:
`horizon-mod-wasm`, wasmtime, **one API with native**)" and says: "C and E are arismoko's work, and we build on it ...
They merge upstream through their own PRs." So a sandboxed runtime may come back, sharing the native hook catalog
(which already has `world_primitives`, `item_model_property`, configs). It is another contributor's work in progress:
writing it ourselves is neither small nor ours to claim. The useful contribution is to follow it and, when it lands,
report which hooks this mod still lacks (a mod tint source, a mod select property, an arrow-entity layer hook).
Whether those mods will be packaged as `.hmod` is not stated anywhere I found.

**2c. Contribute the features to Horizon itself** as Horizon-only options, off by default (allowed by
`CONTRIBUTING.md` "Horizon's own features", MIT code may be adapted with notices). This is a real path to players, but
the deliverable stops being "a `.hmod` by DctrXD": it becomes Horizon code reviewed and owned by Horizon. Listed for
completeness; it changes what this project is, so it is your call.

### Option 3: defer

Keep the analysis, ship nothing, and revisit when Horizon offers sandboxed client code or asset overrides for content
mods.

### Not options (excluded by the project rule, listed only so the choice is informed)

- Native mod (`.hnmod`): existing hooks would cover trails approximately (`world_primitives.line`: one colour per
  line, width in pixels), bow/crossbow by several boolean `item_model_property` conditions, and settings. It would
  still lack a mod tint source and an arrow-entity draw hook. Forbidden by the brief.
- Resource pack (`.zip`): can already override `minecraft/items/crossbow.json` in Horizon (`docs/resource-packs.md:40`),
  so the 2a subset works as a pack today. A different format, so not chosen here.

## Recommendation

**Option 2a as a proposal, plus Option 3 for everything that needs code.** In order:

1. Validate the crossbow item-model data in Horizon with a throwaway local resource pack in a scratch folder (an
   experiment, never published). This turns the "unverified" cell above into a fact and gives the proposal evidence.
2. Open a suggestion to Horizon (hub `/suggest` or an issue, per `CONTRIBUTING.md:11-14,95-105`) describing 2a with
   that evidence. No code until a maintainer agrees.
3. If accepted and merged: build the `.hmod` with the crossbow part of Real Arrow Tip (Phase 2 in ROADMAP.md).
4. Arrow Trails, the bow, tinting and the entity tip stay deferred until a sandboxed code path exists (most likely
   the wasm runtime of option 2b). When it lands, re-run the capability matrix against its real API.

**Delivery order: Real Arrow Tip first, Arrow Trails last.** Real Arrow Tip's crossbow half is the only part that a
data-only change can unlock; Arrow Trails is entirely code (entity tracking plus world geometry).

What you need to decide: [OPEN_QUESTIONS.md, Q1](OPEN_QUESTIONS.md).
