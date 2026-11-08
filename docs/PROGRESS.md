# Progress

## Phase 0: environment (done, 2026-11-08)

- `cargo-horizon-mod` installed from `../horizon` at `41f822b` (`cargo install --locked --path`).
- `../arrowtrails` at `d94f386`, `../real-arrow-tip` at `e2033a0`.
- Horizon client: the launcher's `versions/nightly-2026-11-05-66f3981/horizon-client.exe` with its own prepared
  resources (`.local/horizon-client/26.3`), run with scratch `--game-dir` and `--home`.

## Phase 1: proof of concept

### 1a: `.hmod` skeleton (done, 2026-11-08)

- `mod/horizon-mod.toml`, `mod/assets/dctrxds_archer_addons/lang/{en_us,pt_br}.json`.
- `tools/content-check`: 7 unit tests (manifest id and fields, lang parse and key parity, namespace and ASCII paths).
- Evidence:
  - `cargo horizon-mod check`: `{"ok":true, ... "id":"dctrxds_archer_addons","kind":"mod","side":"client","size":1097,
    "sha256":"e5f2ab7f849a55825db27803f24e435950b3c1df0d9448c104d71cbfcd6ebefc"}`.
  - Game run (`--dev-mod`, `--frames 900`): exit 0, smoke report `"error": null`, log "content from 1 mod(s)",
    `horizon-mod-ids.json` = `["dctrxds_archer_addons"]`, empty `horizon-mod-content-warnings.txt`, both lang files
    in the assembled content.
- Not verified: the Options > Mods row and whether U+00B4 renders (no screen was looked at).

### 1a: confirmed by the owner in the game (2026-11-08)

The row appears in Options > Mods and switches on and off. There is no settings panel and nothing changes in the
game, as expected for a content-only `.hmod` (PORTING_NOTES.md rows 2-7).

### 1b: crossbow data experiment (done, 2026-11-08)

Throwaway, never committed: a local resource pack in the scratch folder (`scripts/make_pack.py`, placeholder
solid-colour textures drawn by the script, `pack.mcmeta` format `[97, 1]` from `horizon-core/src/version.rs:13`) that
overrides `assets/minecraft/items/crossbow.json`. Run on the launcher's nightly 66f3981 with scratch game dir and home,
driven by `horizon-ctl` (`/give` of 9 crossbows, `select`, `screenshot`).

Stored form of the component, read with `horizon-ctl inventory`: `charged_projectiles` is
`[{id:"minecraft:spectral_arrow"}]` (count omitted when 1); a tipped arrow is
`{components:{"minecraft:potion_contents":{potion:"minecraft:poison"}},id:"minecraft:tipped_arrow"}`; multishot
crossbows hold three entries.

Item definition tested: `select component minecraft:charged_projectiles` with exact `when` values (single and triple
spectral, single and triple poison tipped), falling back to `select charge_type` (`arrow`, `rocket`, else none),
each leaf wrapped in `condition component minecraft:enchantments [{enchantments: minecraft:multishot}]`.

Result (screenshot taken before any crossbow was fired; hotbar, GUI context):

| Slot | Crossbow | Expected | Seen |
| --- | --- | --- | --- |
| 1 | arrow | red | red |
| 2 | spectral | yellow | yellow |
| 3 | tipped (poison) | green | green |
| 4 | firework | blue | blue |
| 5 | arrow x3, Multishot | red striped | red striped |
| 6 | spectral x3, Multishot | yellow striped | yellow striped |
| 7 | tipped x3, Multishot | green striped | green striped |
| 8 | firework x3, Multishot | blue striped | blue striped |
| 9 | empty | grey | grey |

First-person hand: slot 1 (red) and slot 8 (blue striped) seen correct. The other hand views were spoiled: mouse
input from a person reached the window during the run (log: "mouse input from a person while an agent was driving"),
which fired the crossbows ("Ol' Betsy" advancement) and unloaded them. Not re-run.

Conclusions:
- Q13 answered: Horizon's engine tells spectral and tipped apart from plain arrows with vanilla data only, by exact
  match of the component value.
- Limits: a tipped arrow matches only the exact potion listed (one case per potion), and an arrow with any extra
  component (a name, custom effects) does not match and falls back to the plain-arrow look. The tip colour cannot
  come from the projectile (no vanilla tint source), so tipped arrows get one fixed look.
- Still impossible in a `.hmod`: shipping this file (the namespace fence). This is the evidence for proposal 2a.
- Evidence files (scratch, not committed because screenshots contain game artwork):
  `C:\Users\jpsen\AppData\Local\Temp\claude\6d40a5309ca0a603\9cf0188a-c1aa-408e-ac0d-d6c213a20211\scratchpad\run4\slot1.png`
  and `...\scratchpad\sheet4.png`.

## Phase 2: crossbow arrow tip (in progress)

### 2.1: upstream proposal drafted (2026-11-08)

`docs/UPSTREAM_PROPOSAL.md`: suggestion for Horizon (client asset overrides in `side = "client"` content mods), with
the Phase 1b evidence. Not posted: waiting for the owner to post it or to authorise posting.

## Gates (last run 2026-11-08)

| Gate | Result |
| --- | --- |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, no warnings |
| `cargo test --workspace` | 7 passed, 0 failed |
| `cargo horizon-mod check` | ok |

Performance: not applicable in Phase 1 (no code runs). Nothing measured.
