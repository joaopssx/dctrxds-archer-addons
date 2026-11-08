# Roadmap

Ordered by the recommendation in [DECISION.md](DECISION.md): Real Arrow Tip's crossbow first (data), everything
that needs code last. Each phase starts only after the previous one's acceptance is shown.

## Phase 0: environment (small)

- **Goal:** the tools and sources the later phases need.
- **Deliverables:** `./arrowtrails` at `d94f386`, `./real-arrow-tip` at `e2033a0`; `cargo-horizon-mod` installed;
  a runnable Horizon client (built from `./horizon` with resources prepared from your own Minecraft 26.3 copy, or a
  released build) for `cargo horizon-mod run --game PATH`.
- **Acceptance:** `cargo horizon-mod --help` prints; `git -C ../arrowtrails rev-parse HEAD` matches; Horizon starts.
- **Risks:** preparing resources needs your game copy and time; Windows `python3` alias missing (use `python`).

## Phase 1: proof of concept (small)

The smallest thing that can be shown in the game today. It does **not** show any arrow feature in a `.hmod`,
because none is possible ([CAPABILITY_MATRIX.md](CAPABILITY_MATRIX.md)).

- **Goal:** (a) prove the `.hmod` pipeline and the display name; (b) prove, in a throwaway experiment, that Horizon's
  item-model engine can drive the crossbow cases with vanilla data only.
- **Deliverables:**
  - (a) `mod/horizon-mod.toml` (`id = "dctrxds_archer_addons"`, `api = "0.2"`, `side = "client"`,
    `capabilities = ["registries"]`), `en_us`/`pt_br` lang, README, THIRD_PARTY_NOTICES.md. Not published.
  - (b) In a scratch folder outside the repo: a local resource pack with `assets/minecraft/items/crossbow.json` using
    `charge_type`, `select component minecraft:charged_projectiles` and the multishot enchantment condition, with
    placeholder textures I draw. Never published, never committed.
- **Acceptance:** `cargo horizon-mod build` and `cargo horizon-mod check` output shown with no errors; a screenshot of
  the Options > Mods row with the name; screenshots of the crossbow loaded with arrow, spectral, tipped and firework,
  each with and without Multishot, under the experiment pack.
- **Tests:** the commands above; a JSON check that the experiment file parses (Horizon logs a refused model).
- **Risks:** `.hmod` that is a pure placeholder could look like a "does nothing" mod: keep it unpublished. The
  `component` exact-match may not work for spectral (then only multishot and rockets are data-solvable).
- **Effort:** small.

## Phase 2: crossbow arrow tip in the `.hmod` (medium, depends on upstream)

- **Goal:** the vanilla crossbow shows the right projectile with the mod on, vanilla with it off.
- **Precondition:** the upstream change 2a (client asset overrides in content mods) is accepted and merged.
- **Deliverables:** the suggestion/issue with Phase 1 evidence; if asked, the small upstream PR following
  `CONTRIBUTING.md` (claim, `pr_ready.py`, < ~500 lines, vanilla reference, agent attribution lines); then
  `mod/assets/...` crossbow models and **redrawn** textures.
- **Acceptance:** with the mod on, the 4 cases x multishot match the Phase 1 screenshots; turned off, the crossbow is
  vanilla; `cargo horizon-mod check` passes.
- **Tests:** `check`; a small script (dev tool) that every model and texture referenced by the item JSON exists.
- **Risks:** maintainers decline 2a (then this phase is cancelled); tipped colour stays generic.
- **Effort:** medium (most of it is the upstream discussion).

## Phase 3: code features (large, blocked on a sandboxed code runtime)

- **Goal:** trails, bow arrow tip, projectile tint, entity tip, settings.
- **Precondition:** a sandboxed code path in Horizon (the wasm runtime noted in `docs/design/mod-hotload.md`), and
  your confirmation that its package format satisfies the `.hmod` rule.
- **Deliverables:** `core/` with unit tests, `adapter/`, settings page, lang.
- **Acceptance (verifiable):**
  - plain arrow, defaults: a white ribbon of 8 segments, alpha 0.25 to 0, width 0.2 to 0.1 blocks;
  - critical: 16 segments `#ffe8a8` to `#ffb055`; spectral: 16 segments `#fcfc76`; tipped: 16 segments in the
    potion colour;
  - a stuck arrow's trail shrinks into it within `L` ticks and then stops drawing;
  - `trails.enabled = false` draws nothing; the arrow tip keeps working, and the reverse.
- **Tests:** `cargo test -p core`: classification precedence, trim to `L`, visibility rule, the `t` formula, ribbon
  vertices for a fixed camera, colour parsing and clamping, `charge_case` for every projectile.
- **Risks:** the API differs from today's native hooks; per-frame cost with many arrows (measure); entity tip may
  still lack a hook.
- **Effort:** large.

## Phase 4: publish (small)

- **Goal:** listed in the Horizon registry.
- **Deliverables:** GitHub release with the `.hmod` (and `.minisig` if signing), `horizon-registry.toml`, icon,
  screenshots, submission through playhorizon.dev/mods/submit.
- **Acceptance:** registry checks green on the PR; reviewer merge.
- **Risks:** see [RISKS.md](RISKS.md) (licences, textures, naming).
- **Effort:** small.
