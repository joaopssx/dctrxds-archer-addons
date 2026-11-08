# Upstream proposal: client asset overrides in content mods

Status: **draft, not posted.** To be posted by the owner (or by an agent on the owner's account, with the marker
lines `horizon/AGENTS.md` requires) as a suggestion on playhorizon.dev/hub or a GitHub issue, per
`horizon/CONTRIBUTING.md:11-14`. No Horizon code is written before a maintainer agrees.

The text below the line is ready to paste.

---

## Let a client-side content mod override vanilla's client assets, as a resource pack does

### What

Today a content mod (`.hmod`) may only add files in its own namespaces, plus vanilla/convention tags and atlas
sources. Anything else, such as `assets/minecraft/items/crossbow.json`, is refused:

- `crates/horizon-mods/src/content.rs:714-724`: "a mod adds files in its own namespaces ..., to vanilla and
  convention (c) tags, or to vanilla's atlases".

Proposal: a content mod with `side = "client"` may also carry **client assets** under `assets/minecraft/`
(item definitions `items/`, models `models/`, textures `textures/`, and only these to start), and they apply
exactly as a resource pack's files do. Data (`data/minecraft/...`) stays fenced as today.

### Why

- Many client-only Fabric/NeoForge mods are, for their visible part, a resource pack plus a little code. Several
  need nothing but vanilla's own item-model language. Today they cannot be content mods, and a separate resource
  pack loses what a mod has: one switch on the Mods page, the registry's mod kind, `side`, and being part of a
  modpack as one item.
- It is vanilla-first: no new format and no new property, only vanilla 26.3 item definitions and models, read by
  Horizon's existing resolver (`horizon-assets/src/item_model.rs`).

### Evidence it works with Horizon as it is

Tested on Nightly 2026-11-05 (66f3981) with an equivalent local resource pack overriding
`assets/minecraft/items/crossbow.json` with `select component minecraft:charged_projectiles` (exact values),
falling back to `select charge_type`, each leaf under `condition component minecraft:enchantments` (Multishot).
Nine crossbows (arrow, spectral, tipped poison, firework, each with and without Multishot, and empty) were drawn with
the nine expected models in the hotbar; first-person checked for two of them. Placeholder textures, no Mojang
artwork. Details: (link to this repository's `docs/PROGRESS.md`, Phase 1b).

### Suggested shape (open to the maintainers' preference)

1. **Fence** (`content.rs`): for `side = "client"` manifests only, also allow `assets/minecraft/{items,models,textures}/`.
   `side = "both"`/`"server"` mods keep today's fence, so nothing a server compares changes.
2. **Order:** mod overrides sit above vanilla and below the player's resource packs, in mod load order (the order
   tags already use), so a player's pack still wins.
3. **Clashes:** two mods overriding the same path: the later in load order wins, with a log line naming both, as tag
   `replace` already reports.
4. **Switch:** assets-only, so the Mods switch applies live as it does for assets today (`docs/mods.md:24-28`).
5. **Players see it:** the Mods row could say "May change how vanilla items look".

Expected size: the fence rule, the layer order, tests and docs; small, but the maintainers know the overlay code
(`horizon-client/src/mod_content.rs:1-16`) best.

### Alternatives considered

- **Publish a resource pack instead.** Works today, and is the fallback if this is declined. Loses the points in
  "Why".
- **A mod-supplied select property or tint source.** Needs code; out of scope for content mods.

### Not asking for

Any change for native mods, any new item-model property, or server-side data overrides.
