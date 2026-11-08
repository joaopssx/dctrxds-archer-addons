# Architecture

One repository, one mod (`id = "dctrxds_archer_addons"`), two features that never depend on each other:
**trails** (from Arrow Trails) and **arrow tip** (from Real Arrow Tip). Parts that need an API Horizon does not offer
yet are marked _blocked_ and are not written until that API exists (no speculative scaffolding).

## Layout

```text
dctrxds-archer-addons/
  mod/                         the .hmod content root (cargo horizon-mod build runs here)
    horizon-mod.toml
    assets/dctrxds_archer_addons/
      lang/en_us.json, pt_br.json
      models/item/...          arrow tip: crossbow/bow layer models     (Phase 2)
      textures/item/...        original, redrawn textures               (Phase 2)
      items/...                only if a future API lets them apply to vanilla items
  core/                        pure Rust library, no Horizon dependency  (Phase 3, blocked)
    Cargo.toml
    src/lib.rs, trail.rs, kind.rs, color.rs, charge.rs, settings.rs
  adapter/                     the only code that talks to Horizon        (Phase 3, blocked)
  tools/content-check/         dev tool: unit tests over mod/ (Phase 1)
  Cargo.toml                   workspace (tools/, later core/ and adapter/)
  docs/
  LICENSE                      MIT, João Pedro (this project's own code)
  LICENSES/                    Biverom's and ToBinio's MIT licences, verbatim
  NOTICE                       what derives from each original
  README.md                    "unofficial combination", credits, links (PT-BR and EN)
  CHANGELOG.md                 PT-BR and EN
```

Why `mod/` is a subfolder: `cargo horizon-mod build` refuses a folder that holds a `Cargo.toml`
(`horizon/docs/mods.md:55-57`), so content and Rust code must not share a root.

## Feature switches

| Switch | Default | Controls | Available when |
| --- | --- | --- | --- |
| Mod on/off (Options > Mods) | off (Horizon's rule for all mods) | everything | now |
| `trails.enabled` | on | all trail drawing | settings API exists (blocked) |
| `trails.<kind>.*` | values of ANALYSIS.md table | length, colours, alphas, widths per kind | settings API |
| `arrow_tip.items` | on | bow/crossbow models | asset override (Phase 2) or code |
| `arrow_tip.entity` | on | coloured tip on arrow entities | code + entity hook (blocked) |
| `arrow_tip.ignored_items` | `[]` | per-item opt-out (`bow`, `crossbow`) | settings API |

Isolation rule: each feature reads only its own settings and its own state. A failure in one (for example an asset
missing) must leave the other drawing normally. In the data-only phase this is automatic: trails do not exist and the
arrow tip is only data.

## `core` (pure logic, tested with plain `cargo test`)

No dependencies beyond `std`. Inputs are plain values, outputs are plain values, so every behaviour in
[ANALYSIS.md](ANALYSIS.md#step-3-the-original-mods) becomes a unit test.

| Module | Responsibility | Key items |
| --- | --- | --- |
| `kind` | Classify an arrow snapshot, first match wins: spectral, tipped (`color != -1`), critical, plain. | `ArrowSnapshot { id, kind, pos, in_ground, in_ground_ticks, critical, color }`, `TrailKind` |
| `trail` | Per-arrow point history: push at tick start, trim to `L`, visibility (`!in_ground || in_ground_ticks < L`), drop on entity removal. | `TrailStore` keyed by entity id |
| `trail` (geometry) | Ribbon: `t = (i + partial) / (L + 1)`, lerp width/colour/alpha, side vector `(dir x view).normalize() * w/2`, one quad per segment. | `fn ribbon(points, camera, partial, &TrailStyle) -> Vec<Vertex>` |
| `color` | `#rrggbb` parsing, ARGB packing, per-channel lerp. | `Rgb`, `Argb` |
| `charge` | Which case a bow/crossbow shows: `none`, `arrow`, `spectral`, `tipped`, `firework`, multishot flag; a plain arrow with a colour counts as tipped. | `fn charge_case(projectile_id, color, multishot) -> ChargeCase` |
| `settings` | Defaults, ranges (length 1..=64, alpha and width 0..=1), validation of loaded values (clamp, reject bad colours). | `Settings::default()`, `Settings::validated()` |

## `adapter` (blocked)

Translates the future Horizon API into `core` calls and back: arrow snapshots per tick into `TrailStore`, each frame
`ribbon()` into one batched geometry submission, item-model questions into `charge_case()`, settings storage into
`Settings`. It stays thin so that a change of Horizon API touches only this crate. Its shape cannot be designed
before that API exists; [CAPABILITY_MATRIX.md](CAPABILITY_MATRIX.md) lists what it would need.

## Data (`mod/`)

- Namespace `dctrxds_archer_addons` only (the fence, `content.rs:714-724`).
- Item model trees are written by hand while they are small (crossbow: 4 cases x multishot). A generator, like
  Real Arrow Tip's `item-model-generator.py`, is added only if the case list grows.
- Lang keys are prefixed `dctrxds_archer_addons.`; Horizon wants no hardcoded user-facing strings.
