//! Checks the `.hmod` content root (`mod/`) against this project's rules, so `cargo test`
//! catches mistakes before `cargo horizon-mod check` runs. Horizon's own rules (manifest
//! fields, the namespace fence) stay Horizon's: `cargo horizon-mod check` is the authority.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

/// The mod id: Horizon requires `[a-z0-9_]{1,64}` (horizon-mods `manifest.rs`).
pub const MOD_ID: &str = "dctrxds_archer_addons";
/// The languages every lang key must exist in.
pub const LANGS: [&str; 2] = ["en_us", "pt_br"];

/// Errors found in the content root, each with the file it concerns.
pub type Problems = Vec<String>;

/// The `mod/` folder next to this workspace.
pub fn content_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../mod")
}

/// Checks `horizon-mod.toml`: the id, a client-side content mod, and the project's own name.
pub fn check_manifest(text: &str) -> Problems {
    let mut problems = Problems::new();
    let table: toml::Table = match toml::from_str(text) {
        Ok(t) => t,
        Err(e) => return vec![format!("horizon-mod.toml: {e}")],
    };
    let field = |key: &str| table.get(key).and_then(toml::Value::as_str);
    match field("id") {
        Some(id) if is_valid_id(id) && id == MOD_ID => {}
        other => problems.push(format!(
            "horizon-mod.toml: id {other:?}, expected {MOD_ID:?}"
        )),
    }
    if field("api") != Some("0.2") {
        problems.push("horizon-mod.toml: api must be \"0.2\" (content mod)".into());
    }
    if field("side") != Some("client") {
        problems.push("horizon-mod.toml: side must be \"client\" (visual-only mod)".into());
    }
    if field("name") != Some("DctrXD\u{b4}s Archer Addons") {
        problems.push("horizon-mod.toml: name must be the display name with U+00B4".into());
    }
    problems
}

/// Horizon's id rule: `[a-z0-9_]{1,64}`.
pub fn is_valid_id(id: &str) -> bool {
    (1..=64).contains(&id.len())
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

/// Parses each language file and checks they all hold exactly the same keys.
pub fn check_lang(files: &[(&str, &str)]) -> Problems {
    let mut problems = Problems::new();
    let mut key_sets: Vec<(&str, BTreeSet<String>)> = Vec::new();
    for (name, text) in files {
        match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(text) {
            Ok(map) => {
                if let Some((key, _)) = map.iter().find(|(_, v)| !v.is_string()) {
                    problems.push(format!("{name}: {key} is not a string"));
                }
                key_sets.push((name, map.keys().cloned().collect()));
            }
            Err(e) => problems.push(format!("{name}: {e}")),
        }
    }
    if let Some((first_name, first)) = key_sets.first() {
        for (name, keys) in &key_sets[1..] {
            for missing in first.symmetric_difference(keys) {
                problems.push(format!("{first_name} and {name} differ on key {missing}"));
            }
        }
    }
    problems
}

/// Checks every content path: ASCII resource characters, and only under this mod's namespace.
pub fn check_paths<'a>(paths: impl IntoIterator<Item = &'a str>) -> Problems {
    let mut problems = Problems::new();
    for path in paths {
        let resource_chars = path
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b"_.-/".contains(&b));
        if !resource_chars {
            problems.push(format!("{path}: not [a-z0-9_.-/]"));
        }
        let mut parts = path.split('/');
        let top = parts.next();
        let namespace = parts.next();
        if !matches!(top, Some("assets" | "data")) || namespace != Some(MOD_ID) {
            problems.push(format!(
                "{path}: outside assets/{MOD_ID}/ and data/{MOD_ID}/"
            ));
        }
    }
    problems
}

/// Content files under `root/assets` and `root/data`, as `/`-separated relative paths.
pub fn content_files(root: &Path) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for top in ["assets", "data"] {
        let dir = root.join(top);
        if dir.exists() {
            walk(root, &dir, &mut out)?;
        }
    }
    out.sort();
    Ok(out)
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let path = entry.map_err(|e| format!("{}: {e}", dir.display()))?.path();
        if path.is_dir() {
            walk(root, &path, out)?;
        } else {
            let rel = path
                .strip_prefix(root)
                .map_err(|e| format!("{}: {e}", path.display()))?;
            let parts: Vec<_> = rel.iter().map(|p| p.to_string_lossy()).collect();
            out.push(parts.join("/"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(rel: &str) -> String {
        let path = content_root().join(rel);
        fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
    }

    #[test]
    fn the_shipped_manifest_passes() {
        assert_eq!(check_manifest(&read("horizon-mod.toml")), Problems::new());
    }

    #[test]
    fn the_shipped_lang_files_parse_and_match() {
        let texts: Vec<(String, String)> = LANGS
            .iter()
            .map(|l| {
                let rel = format!("assets/{MOD_ID}/lang/{l}.json");
                let text = read(&rel);
                (rel, text)
            })
            .collect();
        let files: Vec<(&str, &str)> = texts
            .iter()
            .map(|(n, t)| (n.as_str(), t.as_str()))
            .collect();
        assert_eq!(check_lang(&files), Problems::new());
    }

    #[test]
    fn the_shipped_paths_stay_in_the_mods_namespace() {
        let files = content_files(&content_root()).expect("content root readable");
        assert!(!files.is_empty());
        assert_eq!(
            check_paths(files.iter().map(String::as_str)),
            Problems::new()
        );
    }

    #[test]
    fn the_requested_hyphenated_id_is_not_a_horizon_id() {
        assert!(!is_valid_id("dctrxds-archer-addons"));
        assert!(is_valid_id(MOD_ID));
        assert!(!is_valid_id(""));
        assert!(!is_valid_id(&"a".repeat(65)));
    }

    #[test]
    fn manifest_problems_are_reported() {
        let bad = "id = \"dctrxds-archer-addons\"\nname = \"x\"\napi = \"0.1\"\nside = \"both\"";
        assert_eq!(check_manifest(bad).len(), 4);
        assert_eq!(check_manifest("not toml =").len(), 1);
    }

    #[test]
    fn lang_key_mismatch_and_bad_json_are_reported() {
        let p = check_lang(&[("en", r#"{"a":"A","b":"B"}"#), ("pt", r#"{"a":"A"}"#)]);
        assert_eq!(p, vec!["en and pt differ on key b".to_owned()]);
        assert_eq!(check_lang(&[("en", "{")]).len(), 1);
        assert_eq!(check_lang(&[("en", r#"{"a":1}"#)]).len(), 1);
    }

    #[test]
    fn paths_outside_the_namespace_are_reported() {
        assert_eq!(check_paths(["assets/minecraft/items/bow.json"]).len(), 1);
        assert_eq!(
            check_paths(["data/minecraft/tags/item/arrows.json"]).len(),
            1
        );
        assert_eq!(
            check_paths([format!("assets/{MOD_ID}/Lang/en_us.json").as_str()]).len(),
            1
        );
    }
}
