//! Embed the interface catalogs. Nothing is read at run time.
//!
//! Lusk PDF : no contributor credits (the About window has only About and Updates).
//!
//! - Lusk PDF : les catalogues de traduction sont dans le dépôt, `i18n/locales/<code>/`
//!   (`common.tsv` et `pdfcraft.tsv`, repris de storytold/craft-i18n au commit de
//!   `craft-i18n.rev`, puis complétés et adaptés par Lusk). `craft_i18n.rs` holds `CRAFT_I18N`, one
//!   (locale, shared catalog, PdfCraft catalog) entry per locale that has a `pdfcraft.tsv`.
//!   `CRAFT_I18N_DIR` is ignored: an upstream checkout would replace Lusk's French. A missing
//!   `i18n/locales` is an error, so a build never ships English only by accident.

use std::fmt::Write as _;

fn main() {
    let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default()).join("craft_i18n.rs");
    if let Err(e) = std::fs::write(&out, craft_i18n()) {
        println!("cargo::error=writing {}: {e}", out.display());
    }
}

/// The catalog this app reads in each locale: `locales/<locale>/pdfcraft.tsv`.
const APP_CATALOG: &str = "pdfcraft.tsv";

/// `CRAFT_I18N` from `CRAFT_I18N_DIR`: the `Embedded` list the craft-i18n crate reads.
fn craft_i18n() -> String {
    // Lusk PDF : les catalogues du dépôt. Not canonicalized: on Windows that gives `\\?\C:\…`,
    // which `include_str!` can't read.
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../i18n/locales");
    let entries = match locales(&dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("cargo::error=i18n/locales: {e}; Lusk PDF needs its French catalogs");
            Vec::new()
        }
    };
    let mut o = String::from("/// (locale, shared catalog, PdfCraft's catalog), one entry per locale; `\"\"` for a missing file.\n");
    o.push_str("pub static CRAFT_I18N: &craft_i18n::Embedded = &[\n");
    for (code, shared, app) in entries {
        // `{:?}` escapes the path (Windows backslashes) as a Rust string literal.
        let shared = shared.map_or_else(|| "\"\"".to_string(), |p| format!("include_str!({p:?})"));
        let _ = writeln!(o, "    ({code:?}, {shared}, include_str!({app:?})),");
    }
    o.push_str("];\n");
    o
}

/// One embedded locale: its code, its `common.tsv` if it has one, and its PdfCraft catalog.
type Locale = (String, Option<std::path::PathBuf>, std::path::PathBuf);

/// Every locale under `dir` with PdfCraft's catalog, by code. An error when `dir` isn't a
/// craft-i18n `locales/` directory.
fn locales(dir: &std::path::Path) -> Result<Vec<Locale>, String> {
    // Build scripts run in the crate's directory and `include_str!` resolves against the
    // generated file, so a relative path would mean two different places.
    if !dir.is_absolute() {
        return Err(format!("{} is not an absolute path", dir.display()));
    }
    // Before read_dir, so a checkout that appears later reruns the script.
    println!("cargo::rerun-if-changed={}", dir.display());
    let list = std::fs::read_dir(dir).map_err(|e| format!("{} is not a craft-i18n checkout ({e})", dir.display()))?;
    let mut out = Vec::new();
    for entry in list.flatten() {
        let path = entry.path();
        let Some(code) = path.file_name().and_then(|n| n.to_str()).map(str::to_string) else { continue };
        let app = path.join(APP_CATALOG);
        if !app.is_file() {
            continue;
        }
        println!("cargo::rerun-if-changed={}", app.display());
        let shared = Some(path.join("common.tsv")).filter(|p| p.is_file());
        if let Some(p) = &shared {
            println!("cargo::rerun-if-changed={}", p.display());
        }
        out.push((code, shared, app));
    }
    if out.is_empty() {
        return Err(format!("{} has no locale with a {APP_CATALOG}", dir.display()));
    }
    out.sort();
    Ok(out)
}
