//! Lusk PDF : les pages communes aux applications Lusk, sur l'Accueil.
//!
//! - le renard de l'association à côté de « Bonjour », « Bon après-midi » ou « Bonsoir » ;
//! - « Ouvrir » : de grands boutons carrés (96 × 88), Parcourir… puis OneDrive et Google Drive,
//!   comme Lusk Texte et Lusk Tab. OneDrive et Google Drive ouvrent le sélecteur de fichiers
//!   dans le dossier que leur application synchronise sur cet ordinateur : Lusk PDF ne se
//!   connecte à aucun compte. Sans l'application, le bouton est grisé, et le « ? » à côté dit
//!   quoi installer.

use std::path::{Path, PathBuf};

use chrono::Timelike;
use egui::{Rect, Sense, Stroke, Ui, pos2, vec2};

use crate::theme::{self, Tokens};
use crate::{PdfCraftApp, icons};

/// The greeting for `hour` (0–23, local time): morning until noon, afternoon until 6 pm.
pub(crate) fn greeting(hour: u32) -> &'static str {
    if hour < 12 {
        "Good morning"
    } else if hour < 18 {
        "Good afternoon"
    } else {
        "Good evening"
    }
}

/// The fox and the greeting, at the top of Home.
pub(super) fn welcome(ui: &mut Ui) {
    let hour = chrono::Local::now().hour();
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing = vec2(12.0, 0.0);
        ui.add(
            egui::Image::from_bytes("bytes://lusk-renard.png", include_bytes!("../../../../assets/avatar/lusk-renard.png"))
                .fit_to_exact_size(vec2(44.0, 44.0))
                .alt_text("Lusk"),
        );
        ui.label(egui::RichText::new(tl!(greeting(hour))).font(theme::semibold(26.0)));
    });
}

/// The fox's head, `size` points square (About).
pub(crate) fn fox_head(ui: &mut Ui, size: f32) -> egui::Response {
    ui.add(
        egui::Image::from_bytes("bytes://lusk-renard-tete.png", include_bytes!("../../../../assets/avatar/lusk-renard-tete.png"))
            .fit_to_exact_size(vec2(size, size))
            .alt_text(tl!("The Lusk association's fox")),
    )
}

/// « Ouvrir » : Parcourir…, then OneDrive and Google Drive.
pub(super) fn open_row(app: &mut PdfCraftApp, ui: &mut Ui) {
    ui.label(egui::RichText::new(tl!("Open")).font(theme::semibold(17.0)));
    ui.add_space(8.0);
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing = vec2(10.0, 10.0);
        if square_button(ui, "folder-open", tl!("Browse…"), true).on_hover_text(tl!("Open a PDF from this computer")).clicked() {
            app.open_dialog();
        }
        #[cfg(not(target_arch = "wasm32"))]
        cloud_buttons(app, ui);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn cloud_buttons(app: &mut PdfCraftApp, ui: &mut Ui) {
    let (onedrive, google) = cloud_dirs(ui.ctx());
    let services = [
        (
            "OneDrive",
            onedrive,
            tl!(
                "To use OneDrive, the OneDrive app must be installed on this computer and signed in to your Microsoft account (it comes with Windows). Lusk PDF then opens the documents in your OneDrive folder, which OneDrive keeps in step with your account."
            ),
        ),
        (
            "Google Drive",
            google,
            tl!(
                "To use Google Drive, the free Google Drive for desktop app must be installed on this computer and signed in to your Google account. Lusk PDF then opens the documents in My Drive, which Google Drive keeps in step with your account."
            ),
        ),
    ];
    for (service, dirs, help) in services {
        ui.add_space(6.0);
        if dirs.is_empty() {
            square_button(ui, "cloud", service, false)
                .on_hover_text(crate::i18n::fmt(tl!("{service} is not installed on this computer."), &[("service", service)]));
            help_hint(ui, help);
            continue;
        }
        for (label, dir) in dirs {
            let tip = crate::i18n::fmt(
                tl!("Open a document from your {service} space ({dir})"),
                &[("service", service), ("dir", &dir.display().to_string())],
            );
            if square_button(ui, "cloud", &label, true).on_hover_text(tip).clicked() {
                app.open_dialog_in(dir.clone());
            }
            help_hint(ui, help);
        }
    }
}

/// A large square button, its icon above its label, as in Lusk Texte and Lusk Tab. Greyed out
/// when not `enabled` (a service that isn't installed): it then only answers hovering, for its
/// tooltip.
fn square_button(ui: &mut Ui, icon: &str, label: &str, enabled: bool) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(96.0, 88.0), if enabled { Sense::click() } else { Sense::hover() });
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, enabled, label));
    let ink = if enabled { t.text } else { t.text_faint };
    let hovered = enabled && resp.hovered();
    let p = ui.painter();
    p.rect_filled(rect, 6.0, if hovered { t.hover } else { t.card });
    p.rect_stroke(rect, 6.0, Stroke::new(1.0, if hovered { t.accent } else { t.border }), egui::StrokeKind::Inside);
    let icon_rect = Rect::from_center_size(pos2(rect.center().x, rect.top() + 32.0), vec2(28.0, 28.0));
    icons::paint(ui, icon_rect, icon, 26.0, if enabled { t.accent } else { t.text_faint });
    let galley = ui.painter().layout(label.to_string(), theme::regular(12.0), ink, rect.width() - 8.0);
    ui.painter().galley(pos2(rect.center().x - galley.size().x / 2.0, rect.top() + 56.0), galley, ink);
    if enabled { theme::hand(resp) } else { resp }
}

/// A small « ? » whose tooltip explains `text`.
#[cfg(not(target_arch = "wasm32"))]
fn help_hint(ui: &mut Ui, text: &str) {
    let (r, resp) = ui.allocate_exact_size(vec2(18.0, 18.0), Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Label, true, tl!("Help")));
    let t = Tokens::get(ui.ctx());
    icons::paint(ui, r, "circle-help", 16.0, t.text_muted);
    resp.on_hover_text(text);
}

/// The OneDrive and Google Drive folders of this computer (shown name, folder).
#[cfg(not(target_arch = "wasm32"))]
type CloudDirs = (Vec<(String, PathBuf)>, Vec<(String, PathBuf)>);

/// [`CloudDirs`], looked for at most every 10 seconds (Google Drive's search tries every drive
/// letter).
#[cfg(not(target_arch = "wasm32"))]
fn cloud_dirs(ctx: &egui::Context) -> CloudDirs {
    let id = egui::Id::unique("lusk_cloud_dirs");
    let now = ctx.input(|i| i.time);
    if let Some((at, dirs)) = ctx.data(|d| d.get_temp::<(f64, CloudDirs)>(id))
        && now - at < 10.0
    {
        return dirs;
    }
    let dirs = (onedrive_dirs(), google_drive_dirs());
    ctx.data_mut(|d| d.insert_temp(id, (now, dirs.clone())));
    dirs
}

/// The OneDrive folders synced on this computer, with their shown name (« OneDrive »,
/// « OneDrive - Entreprise »…). Windows names them in OneDriveConsumer (personal account),
/// OneDriveCommercial (work account) and OneDrive; elsewhere (Linux with a OneDrive client),
/// ~/OneDrive. Empty when OneDrive is not installed.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn onedrive_dirs() -> Vec<(String, PathBuf)> {
    let vars: Vec<PathBuf> = ["OneDriveConsumer", "OneDriveCommercial", "OneDrive"]
        .iter()
        .filter_map(std::env::var_os)
        .filter(|p| !p.is_empty())
        .map(PathBuf::from)
        .collect();
    onedrive_dirs_in(&vars, home().as_deref())
}

/// [`onedrive_dirs`] from the folders the variables name and the home folder.
#[cfg(not(target_arch = "wasm32"))]
fn onedrive_dirs_in(vars: &[PathBuf], home: Option<&Path>) -> Vec<(String, PathBuf)> {
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    let candidates = vars.iter().cloned().chain(home.map(|h| h.join("OneDrive")));
    for dir in candidates {
        if !dir.is_dir() {
            continue;
        }
        let key = dir.canonicalize().unwrap_or_else(|_| dir.clone());
        if found.iter().any(|(_, d)| d.canonicalize().unwrap_or_else(|_| d.clone()) == key) {
            continue;
        }
        let label = dir.file_name().map_or_else(|| "OneDrive".to_string(), |n| n.to_string_lossy().into_owned());
        found.push((label, dir));
    }
    found
}

/// The « Mon Drive » folders of Google Drive for desktop, with their shown name. Windows: the
/// drive the app creates (G: by default, any letter from D to Z) or the mirrored folder in the
/// home folder; macOS: ~/Library/CloudStorage/GoogleDrive-<account>; Linux (no official client):
/// ~/GoogleDrive or ~/google-drive. Empty when Google Drive is not installed.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn google_drive_dirs() -> Vec<(String, PathBuf)> {
    google_drive_dirs_in(home().as_deref())
}

/// [`google_drive_dirs`] with `home` as the home folder.
#[cfg(not(target_arch = "wasm32"))]
fn google_drive_dirs_in(home: Option<&Path>) -> Vec<(String, PathBuf)> {
    const MY_DRIVE: [&str; 2] = ["Mon Drive", "My Drive"];
    let mut candidates: Vec<(String, PathBuf)> = Vec::new();
    if cfg!(windows) {
        for letter in 'D'..='Z' {
            for name in MY_DRIVE {
                candidates.push(("Google Drive".into(), PathBuf::from(format!("{letter}:\\{name}"))));
            }
        }
    }
    if let Some(h) = home {
        for name in MY_DRIVE {
            candidates.push(("Google Drive".into(), h.join(name)));
        }
        if let Ok(entries) = std::fs::read_dir(h.join("Library").join("CloudStorage")) {
            for e in entries.flatten() {
                let n = e.file_name().to_string_lossy().into_owned();
                if let Some(account) = n.strip_prefix("GoogleDrive-") {
                    for name in MY_DRIVE {
                        candidates.push((format!("Google Drive ({account})"), e.path().join(name)));
                    }
                }
            }
        }
        for name in ["GoogleDrive", "google-drive", "Google Drive"] {
            candidates.push(("Google Drive".into(), h.join(name)));
        }
    }
    let mut found: Vec<(String, PathBuf)> = Vec::new();
    for (label, dir) in candidates {
        if dir.is_dir() && !found.iter().any(|(_, d)| d == &dir) {
            found.push((label, dir));
        }
    }
    found
}

/// The user's home folder.
#[cfg(not(target_arch = "wasm32"))]
fn home() -> Option<PathBuf> {
    std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")).filter(|h| !h.is_empty()).map(PathBuf::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_greeting_follows_the_time_of_day() {
        assert_eq!(greeting(0), "Good morning");
        assert_eq!(greeting(11), "Good morning");
        assert_eq!(greeting(12), "Good afternoon");
        assert_eq!(greeting(17), "Good afternoon");
        assert_eq!(greeting(18), "Good evening");
        assert_eq!(greeting(23), "Good evening");
    }

    #[test]
    fn the_greetings_are_in_french() {
        for (en, fr) in [("Good morning", "Bonjour"), ("Good afternoon", "Bon après-midi"), ("Good evening", "Bonsoir")] {
            assert_eq!(crate::i18n::tr(crate::i18n::Lang::from_code("fr").expect("French"), en), fr);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn cloud_folders_are_found_only_where_they_exist() {
        let home = std::env::temp_dir().join(format!("lusk-cloud-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&home);
        std::fs::create_dir_all(&home).unwrap();
        assert!(onedrive_dirs_in(&[], Some(&home)).is_empty(), "no OneDrive app, no button");
        assert!(google_drive_dirs_in(Some(&home)).iter().all(|(_, d)| !d.starts_with(&home)));
        std::fs::create_dir_all(home.join("OneDrive")).unwrap();
        std::fs::create_dir_all(home.join("Mon Drive")).unwrap();
        // The variable and the home folder name the same folder: one button.
        let found = onedrive_dirs_in(&[home.join("OneDrive")], Some(&home));
        assert_eq!(found, vec![("OneDrive".to_string(), home.join("OneDrive"))]);
        assert!(google_drive_dirs_in(Some(&home)).contains(&("Google Drive".to_string(), home.join("Mon Drive"))));
        let _ = std::fs::remove_dir_all(&home);
    }
}
