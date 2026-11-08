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

### 1b: crossbow data experiment (not started)

Blocked on OPEN_QUESTIONS.md Q3 (permission for a throwaway local resource pack) and on in-game interaction
(loading a crossbow, enchanting it), which needs you at the keyboard or a scripted session.

## Gates (last run 2026-11-08)

| Gate | Result |
| --- | --- |
| `cargo fmt --all --check` | exit 0 |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit 0, no warnings |
| `cargo test --workspace` | 7 passed, 0 failed |
| `cargo horizon-mod check` | ok |

Performance: not applicable in Phase 1 (no code runs). Nothing measured.
