# Open questions

## For you (decisions)

1. **Which option?** (DECISION.md) 1 = unpublished skeleton only; 2a = propose client asset overrides for content
   mods; 2b = wait for the sandboxed wasm runtime; 2c = contribute the features to Horizon itself; 3 = defer.
   Recommendation: 2a as a proposal + defer the rest until 2b lands.
2. **If the wasm runtime returns with its own file format, does that count as ".hmod sandboxed" for you?**
   The rule's intent (sandboxed, no custom game build) would be met; the extension may differ.
3. ~~**Phase 1b experiment**~~ done after your "continue" (PROGRESS.md 1b). Was: may I build a throwaway local resource pack in a scratch folder (never published or
   committed) to test the crossbow data in Horizon? It is a test tool, not the deliverable.
4. **Running Horizon**: will you provide a runnable Horizon client (a release, or prepared resources from your own
   Minecraft 26.3 copy)? Needed for any in-game proof.
5. **Install `cargo-horizon-mod`** now (`cargo binstall --git https://github.com/horizon-rust/horizon cargo-horizon-mod`,
   or `cargo install --git ...`)? I did not install it.
6. **Clone the originals into `./arrowtrails` and `./real-arrow-tip`** at the analysed commits? They are only in a
   scratch folder now.
7. **Textures**: draw new ones ourselves, or contact ToBinio first about their origin?
8. **Contact the original authors** (courtesy note, and ToBinio's copyright holder name)?

## For Horizon maintainers (to ask in the proposal)

9. Would a `side = "client"` content mod be allowed to override `assets/minecraft/items|models|textures` as a resource
   pack layer? Where in the pack order?
10. Status and planned API of the wasm runtime (arismoko's `horizon-mod-wasm`): file format, capabilities, and whether
    it will expose `world_primitives`, `item_model_property`, configs, a mod **select** property and a mod **tint
    source**.
11. Is there any plan for a hook on arrow entity drawing (Real Arrow Tip's coloured head layer)?
12. Are `world_primitives` drawn after translucent terrain, particles and weather? (Decides whether trails behind
    water or rain look right; the original's "Delayed Rendering" exists for this kind of ordering.)

## Technical, to verify by experiment

13. ~~Does `select component minecraft:charged_projectiles` match an exact list value in Horizon?~~ **Answered
    2026-11-08 (PROGRESS.md 1b): yes.** `when` must be a list of alternatives whose items are the stored lists, e.g.
    `[[{"id":"minecraft:spectral_arrow"}]]`; count 1 is omitted in the stored value. Horizon compares raw JSON
    (`item_model.rs:1277`); whether vanilla would also match a `when` spelled with `"count": 1` is not checked.
14. Does Horizon's GUI font and the launcher render U+00B4 in the mod name?
15. Does Horizon's crossbow in first/third person use the same item-model path with `charged_projectiles` set on
    remote players' items (multiplayer)? [ASSUMED yes, unverified]
16. On a vanilla server, are the arrow's critical flag and colour synced to Horizon's client entity state as vanilla
    does? (`creatures.rs:7433` reads `effect_color`, so colour is present; crit not checked.)
