# DctrXD´s Archer Addons

**PT-BR:** DctrXD´s Archer Addons é uma combinação NÃO OFICIAL baseada em Arrow Trails (Biverom) e Real Arrow Tip
(ToBinio). Não é afiliado aos autores originais, à Mojang ou à Microsoft.

**EN:** DctrXD´s Archer Addons is an UNOFFICIAL combination based on Arrow Trails (Biverom) and Real Arrow Tip
(ToBinio). It is not affiliated with the original authors, Mojang or Microsoft.

Não tem relação com o projeto "ArcherAddons" para Hypixel Skyblock. / Unrelated to the "ArcherAddons" project for
Hypixel Skyblock.

## Estado / Status

**PT-BR:** Fase 1 (esqueleto). O `.hmod` carrega no Horizon, mas **ainda não muda nada no jogo**. Um `.hmod` hoje só
tem dados e recursos, sem código, então rastros e pontas coloridas ainda não são possíveis. Veja `docs/DECISION.md` e
`docs/PROGRESS.md`. Não publicado.

**EN:** Phase 1 (skeleton). The `.hmod` loads in Horizon but **changes nothing in the game yet**: a `.hmod` today is
data and assets only, with no code, so trails and coloured tips are not possible yet. See `docs/DECISION.md` and
`docs/PROGRESS.md`. Not published.

## Originais / Originals

- Arrow Trails, Biverom, MIT: https://github.com/Biverom/arrowtrails
- Real Arrow Tip, ToBinio, MIT: https://tangled.org/tobinio.dev/real-arrow-tip

Licenças e o que foi derivado de cada um / Licences and what derives from each: `LICENSES/`, `NOTICE`.

## Build

```sh
cd mod
cargo horizon-mod build                                          # target/horizon-mod/dctrxds_archer_addons.hmod
cargo horizon-mod check target/horizon-mod/dctrxds_archer_addons.hmod
cd ..
cargo test --workspace                                           # content checks (tools/content-check)
```

`mod/` holds the `.hmod` content; it must not contain a `Cargo.toml` (Horizon refuses it).

## Licença / License

MIT, João Pedro (`LICENSE`), for this project's own files. Parts derived from the originals keep their MIT notices
(`LICENSES/`).
