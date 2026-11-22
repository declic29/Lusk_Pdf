//! Lusk PDF : l'aide (Aide › Aide de Lusk PDF), comme celle de Lusk Texte : des guides en
//! français embarqués dans le programme (dossier `aide/`, un Markdown simple), dans une fenêtre
//! avec le sommaire à gauche et la page à droite. Tout fonctionne sans connexion.
//!
//! La page « Vos données et votre vie privée » s'ouvre aussi seule (Aide › Confidentialité, et
//! l'onglet Mises à jour de « À propos »).

use egui::text::{LayoutJob, TextFormat};
use egui::{Align2, Color32, RichText, ScrollArea, Sense, Ui, pos2, vec2};

use crate::theme::{self, Tokens};
use crate::{PdfCraftApp, icons};

/// Les guides : titre et icône dans le sommaire, texte. Le texte accepte `# titre`,
/// `## sous-titre`, les listes `- …` et `1. …`, et `**gras**`.
pub(crate) const GUIDES: [(&str, &str, &str); 7] = [
    ("Bien démarrer", "house", include_str!("../aide/01-bien-demarrer.md")),
    ("Lire et naviguer", "book-open", include_str!("../aide/02-lire.md")),
    ("Commenter et annoter", "message-square-text", include_str!("../aide/03-commenter.md")),
    ("Remplir et signer", "pen-line", include_str!("../aide/04-remplir-signer.md")),
    ("Organiser, modifier et protéger", "layout-grid", include_str!("../aide/05-organiser-modifier.md")),
    ("Convertir, imprimer et OCR", "printer", include_str!("../aide/06-convertir-imprimer.md")),
    // Tout ce qui reste sur l'ordinateur, et les seuls moments où Internet sert.
    ("Confidentialité", "shield-check", include_str!("../aide/07-confidentialite.md")),
];

/// La page « Vos données et votre vie privée ».
pub(crate) const PRIVACY: usize = 6;

/// Taille et interligne du texte des pages.
const TEXT: f32 = 14.0;
const LINE: f32 = 22.0;

/// The Help dialog's content: the contents on the left, the page on the right. `next` is the
/// dialog that replaces this one at the end of the frame (the keyboard shortcuts): setting
/// `app.dialog` here would be undone, since the dialog frame puts its own dialog back.
pub(crate) fn dialog(ui: &mut Ui, app: &mut PdfCraftApp, next: &mut Option<crate::Dialog>) {
    let t = Tokens::get(ui.ctx());
    let page = app.help_page.min(GUIDES.len() - 1);
    ui.horizontal_top(|ui| {
        ui.vertical(|ui| {
            ui.set_width(230.0);
            ui.label(RichText::new(tl!("Lusk PDF Help")).font(theme::semibold(18.0)));
            ui.add_space(10.0);
            for (i, (title, icon, _)) in GUIDES.iter().enumerate() {
                if nav_item(ui, icon, title, i == page) {
                    app.help_page = i;
                }
            }
            ui.add_space(10.0);
            ui.separator();
            ui.add_space(6.0);
            if nav_item(ui, "circle-help", tl!("Keyboard shortcuts"), false) {
                *next = Some(crate::Dialog::Shortcuts);
            }
        });
        // As tall as the window allows, so the dialog's buttons stay on screen.
        let height = (ui.ctx().content_rect().height() - 220.0).clamp(220.0, 560.0);
        // A line between the contents and the page, as tall as the page (a separator in a row
        // would take all the height the dialog could have).
        let (line, _) = ui.allocate_exact_size(vec2(25.0, height), Sense::hover());
        ui.painter().vline(line.center().x, line.y_range(), egui::Stroke::new(1.0, t.divider));
        ui.add_space(8.0);
        ui.vertical(|ui| {
            ui.set_width(640.0);
            ui.set_max_height(height);
            ScrollArea::vertical().id_salt(("lusk-help", page)).max_height(height).min_scrolled_height(height).auto_shrink([false, true]).show(
                ui,
                |ui| {
                    ui.set_width(620.0);
                    markdown(ui, GUIDES[page].2, &t);
                    ui.add_space(12.0);
                },
            );
        });
    });
}

/// A line of the contents: icon and title, highlighted when it is the page shown.
fn nav_item(ui: &mut Ui, icon: &str, label: &str, active: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let (rect, resp) = ui.allocate_exact_size(vec2(ui.available_width(), 34.0), Sense::click());
    resp.widget_info(|| egui::WidgetInfo::selected(egui::Role::Button, true, active, label));
    if active {
        ui.painter().rect_filled(rect, 6.0, t.selected);
    } else if resp.hovered() {
        ui.painter().rect_filled(rect, 6.0, t.hover);
    }
    let colour = if active { t.accent_text } else { t.icon };
    icons::paint(ui, egui::Rect::from_center_size(pos2(rect.left() + 18.0, rect.center().y), vec2(18.0, 18.0)), icon, 16.0, colour);
    let font = if active { theme::semibold(13.5) } else { theme::regular(13.5) };
    ui.painter().text(pos2(rect.left() + 36.0, rect.center().y), Align2::LEFT_CENTER, label, font, if active { t.accent_text } else { t.text });
    theme::hand(resp).clicked()
}

/// A page of the help: titles, paragraphs, lists and `**bold**`.
fn markdown(ui: &mut Ui, text: &str, t: &Tokens) {
    for line in text.lines() {
        let l = line.trim_end();
        if let Some(h) = l.strip_prefix("# ") {
            ui.label(RichText::new(h).font(theme::semibold(24.0)).color(t.text));
            ui.add_space(10.0);
        } else if let Some(h) = l.strip_prefix("## ") {
            ui.add_space(12.0);
            ui.label(RichText::new(h).font(theme::semibold(17.0)).color(t.accent_text));
            ui.add_space(4.0);
        } else if l.trim().is_empty() {
            ui.add_space(6.0);
        } else if let Some(item) = l.strip_prefix("- ") {
            item_line(ui, None, item, t);
        } else if let Some((n, item)) = numbered(l) {
            item_line(ui, Some(n), item, t);
        } else {
            let w = ui.available_width();
            ui.add(egui::Label::new(inline(l, t.text, w)).wrap());
            ui.add_space(4.0);
        }
    }
}

/// `1. texte` → ("1", "texte").
fn numbered(l: &str) -> Option<(&str, &str)> {
    let (n, rest) = l.split_once(". ")?;
    (!n.is_empty() && n.chars().all(|c| c.is_ascii_digit())).then_some((n, rest))
}

/// A list item: a dot, or the step number in a circle, then the text, wrapped with an indent.
fn item_line(ui: &mut Ui, number: Option<&str>, text: &str, t: &Tokens) {
    ui.horizontal_top(|ui| {
        ui.add_space(4.0);
        let (r, _) = ui.allocate_exact_size(vec2(28.0, LINE), Sense::hover());
        let c = pos2(r.min.x + 10.0, r.center().y);
        match number {
            Some(n) => {
                ui.painter().circle_filled(c, 10.0, t.accent);
                ui.painter().text(c, Align2::CENTER_CENTER, n, theme::semibold(11.5), Color32::WHITE);
            }
            None => {
                ui.painter().circle_filled(c, 3.5, t.accent);
            }
        }
        let w = ui.available_width();
        ui.add(egui::Label::new(inline(text, t.text, w)).wrap());
    });
    ui.add_space(3.0);
}

/// A line with its `**bold**` runs, wrapped at `width`. `\\` in the text is a backslash.
fn inline(text: &str, color: Color32, width: f32) -> LayoutJob {
    let mut job = LayoutJob::default();
    job.wrap.max_width = width;
    let text = text.replace("\\\\", "\\");
    for (i, part) in text.split("**").enumerate() {
        if part.is_empty() {
            continue;
        }
        let font = if i % 2 == 1 { theme::semibold(TEXT) } else { theme::regular(TEXT) };
        job.append(part, 0.0, TextFormat { font_id: font, color, line_height: Some(LINE), ..Default::default() });
    }
    job
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_guide_has_a_title_and_balanced_bold() {
        for (title, _, text) in GUIDES {
            assert!(text.starts_with("# "), "{title}: starts with its title");
            for line in text.lines() {
                assert_eq!(line.matches("**").count() % 2, 0, "{title}: unbalanced ** in {line:?}");
            }
        }
    }

    #[test]
    fn the_privacy_page_is_the_privacy_guide() {
        assert_eq!(GUIDES[PRIVACY].0, "Confidentialité");
        assert!(GUIDES[PRIVACY].2.contains("GitHub"));
    }

    #[test]
    fn every_icon_exists() {
        for (title, icon, _) in GUIDES {
            assert!(icons::exists(icon), "{title}: icon {icon}");
        }
        assert!(icons::exists("circle-help"));
    }

    #[test]
    fn numbered_lines_are_steps() {
        assert_eq!(numbered("2. Ouvrez"), Some(("2", "Ouvrez")));
        assert_eq!(numbered("Ctrl. Z"), None);
    }

    #[test]
    fn help_and_privacy_open_from_the_help_menu() {
        let mut app = PdfCraftApp::new();
        app.execute("help.guide");
        assert_eq!(app.dialog, Some(crate::Dialog::Help));
        assert_eq!(app.help_page, 0);
        app.dialog = None;
        app.execute("help.privacy");
        assert_eq!((app.dialog, app.help_page), (Some(crate::Dialog::Help), PRIVACY));
    }
}
