# Risks

## Legal

| Risk | Evidence | Mitigation |
| --- | --- | --- |
| Losing the MIT notices of the originals | Arrow Trails: "Copyright (c) 2025 Biverom" (`arrowtrails/LICENSE` at `d94f386`). Real Arrow Tip: "Copyright (c) 2023 " with **no holder name** (`real-arrow-tip/LICENSE` at `e2033a0`); its sources say `@author Tobias Frischmann`, commits by ToBinio. | Both LICENSE texts are copied verbatim into `LICENSES/`; `NOTICE` names project URL, commit and what derives from each. Any file adapted from either (item JSON trees, defaults, algorithm) starts with a comment naming the origin; JSON has no comments, so those are listed by path in `NOTICE`. Credit "ToBinio" as the README does; ask ToBinio how to name the holder (OPEN_QUESTIONS.md). |
| Real Arrow Tip's textures contain Mojang artwork | `textures/item/crossbow_spectral.png` (16x16) depicts a whole crossbow with an arrow, not only the arrow [VERIFIED by viewing]. It is almost certainly an edit of the vanilla crossbow texture [ASSUMED]. MIT from ToBinio cannot license Mojang's part. | Do **not** copy any of the 20 PNGs. Draw original textures, or design layers that contain only the projectile and let vanilla's own bow/crossbow layer show beneath (the player's copy provides it). `horizon-mods/MOD-POLICY.md` promise 3 and Horizon's `AGENTS.md` "Never commit Mojang's files". |
| Mojang code or assets in the repo | Horizon forbids game files, decompiled source and prepared resources (`CONTRIBUTING.md:47-50`). | Never commit anything from a Minecraft jar or `.local/`. Experiments that need vanilla assets stay in a scratch folder outside the repo. Port behaviour, never Java code of Minecraft. |
| Port without permission | `MOD-POLICY.md` promise 2: a port keeps notices, credits and links the original; licence must allow it. | Both are MIT: allowed. README states it is an **unofficial** combination, not endorsed by Biverom or ToBinio, with links to both. Optional courtesy: tell both authors. |
| Reading code under other licences | Horizon's rule lists what may be read (`CONTRIBUTING.md:52-71`). Arrow Trails depends on MidnightLib; Real Arrow Tip on YACL and Mod Menu. | Only the two MIT repositories were read. Do not read those libraries' sources; they are not needed. |
| Contributing upstream with an AI agent | `horizon/AGENTS.md:18-27`: PR body first line names the person whose agent opened it; commits carry `Co-Authored-By`; comments start with the agent marker that file specifies. | Follow it on any Horizon issue or PR in Phase 2. |

## Naming

| Risk | Mitigation |
| --- | --- |
| Confusion with your Hypixel Skyblock project "ArcherAddons" | Mod id `dctrxds_archer_addons`, namespace `dctrxds_archer_addons`, display name "DctrXD´s Archer Addons", repository `dctrxds-archer-addons`. Never `archeraddons`/`archer_addons` alone. README says it is unrelated to ArcherAddons for Hypixel. |
| The requested id `dctrxds-archer-addons` is rejected | The manifest id must be `[a-z0-9_]{1,64}` (`horizon/crates/horizon-mods/src/manifest.rs:224-233`). Use the underscore form in the manifest and lang keys; the hyphen stays in the repo and folder name. |
| U+00B4 in the display name does not render | Unverified (no game run). Fallback: "DctrXD's Archer Addons" with ASCII apostrophe, as the brief allows. Check in Phase 1. |

## Quality and project

| Risk | Mitigation |
| --- | --- |
| Building on an API that does not exist | The `.hmod` has no code API today. Nothing in `core/`/`adapter/` is written before one exists (ROADMAP Phase 3). |
| Upstream proposal 2a declined | It contradicts an explicit rule ("Vanilla's own files stay vanilla's", `content.rs:716`). Ask before coding; accept a no. |
| Publishing a mod that does nothing | Phase 1's `.hmod` is not published. Only publish once a player-visible feature works. |
| Registry policy | `MOD-POLICY.md`: MIT source on public GitHub, release asset, optional minisign key (then every later version must be signed with it; keep the secret key backed up), page fields from `horizon-registry.toml`. `side`/`capabilities` come from the manifest. |
| Stale docs mislead later work | `horizon-mods/README.md` still describes the removed sandbox. Re-check `docs/mods.md` and `git log` before each phase. |
| Performance (future code) | Batch all trails into one submission per frame; cap trail length at 64 as the original does; measure with many arrows before release. |
| "Multiplayer and vanilla servers" claim | Holds only if every input is client-side synced data [ASSUMED]. Verify on a vanilla server in Phase 3. |
