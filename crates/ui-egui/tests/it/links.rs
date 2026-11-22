//! Community links (Lusk PDF): the top bar's button opens lusk.bzh from everywhere; the Help menu
//! and the About dialog open the Lusk association's pages.

use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use pdfcraft_engine::links;
use pdfcraft_ui_egui::{Dialog, PdfCraftApp};

fn harness(setup: impl FnOnce(&mut PdfCraftApp) + 'static) -> Harness<'static, PdfCraftApp> {
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_eframe(move |_cc| {
        let mut app = PdfCraftApp::new();
        app.set_option("language", "en").unwrap();
        setup(&mut app);
        app
    });
    h.run_steps(4);
    h
}

#[test]
fn the_top_bar_button_opens_the_lusk_website() {
    let mut h = harness(|_| {});
    h.get_by_label("Search on lusk.bzh").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some("https://www.lusk.bzh"));
    assert_eq!(links::WEBSITE, "https://www.lusk.bzh");
}

/// Lusk PDF : l'accueil n'a plus de carte de l'association (ses liens restent dans À propos et le
/// menu Aide).
#[test]
fn the_home_screen_has_no_community_card() {
    let h = harness(|_| {});
    assert!(h.query_by_label("Lusk PDF, by the Lusk association").is_none());
    assert!(h.query_by_label("Write to the Lusk association").is_none());
}

#[test]
fn about_dialog_shows_the_brand_and_links() {
    let pdf = b"%PDF-1.7\n1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj\n2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj\n3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 200 200] >> endobj\ntrailer << /Root 1 0 R >>\n%%EOF";
    // With a document open, so the home screen's own links are not on screen.
    let mut h = harness(move |app| {
        app.open_bytes("one.pdf", None, pdf.to_vec()).unwrap();
        app.dialog = Some(Dialog::About);
    });
    assert!(h.query_all_by_label("Lusk PDF").count() >= 1, "the Lusk PDF mark (alt text)");
    assert!(h.query_by_label("ArtCraft").is_none(), "no ArtCraft logo");
    h.get_by_label("Write to the Lusk association").click();
    h.run_steps(2);
    assert_eq!(h.state().last_opened_url.as_deref(), Some(links::DISCORD));
}

#[test]
fn help_commands_open_each_link() {
    for l in links::LINKS {
        let mut h = harness(|_| {});
        assert!(h.state_mut().execute(l.command), "{}", l.command);
        assert_eq!(h.state().last_opened_url.as_deref(), Some(l.url));
        assert_eq!(pdfcraft_engine::commands::command(l.command).unwrap().menu, Some("Help"));
    }
}

/// Runs frames until the node labelled `label` exists and has stopped moving, then returns it.
///
/// The About dialog is a modal centred on its own size, and switching tabs changes that size, so
/// for a few frames afterwards the modal grows and recentres and every control in it moves. A
/// fixed number of frames is a guess at how long that takes: it was too few after the 0.5.0
/// contributors refresh (78 names instead of 5) on some machines, the click on Table landed where
/// the button had been, and the table never opened (#671).
fn settled<'a>(h: &'a mut Harness<'static, PdfCraftApp>, label: &'a str) -> egui_kittest::Node<'a> {
    let mut last = None;
    for _ in 0..60 {
        h.run_steps(1);
        let now = h.query_by_label(label).map(|n| n.rect());
        if now.is_some() && now == last {
            return h.get_by_label(label);
        }
        last = now;
    }
    panic!("{label:?} never appeared or never stopped moving");
}

/// Lusk PDF : la fenêtre À propos n'a que deux onglets, À propos et Mises à jour.
#[test]
fn about_dialog_has_only_about_and_updates_tabs() {
    let mut h = harness(|app| app.dialog = Some(Dialog::About));
    settled(&mut h, "Updates");
    assert!(h.query_all_by_label("About").count() >= 1);
    for gone in ["Contributors", "Models", "Display name", "Real name"] {
        assert!(h.query_by_label(gone).is_none(), "{gone} is still shown");
    }
}

/// Lusk PDF : « Raccourcis clavier », dans la fenêtre d'aide, ouvre bien la liste des raccourcis
/// (avant, la fenêtre d'aide se remettait aussitôt à sa place).
#[test]
fn the_help_window_opens_the_keyboard_shortcuts() {
    let mut h = harness(|app| {
        app.execute("help.guide");
    });
    h.run_steps(2);
    assert_eq!(h.state().dialog, Some(Dialog::Help));
    // The top bar has a "Keyboard shortcuts" button too; the help window's entry comes after it.
    h.get_all_by_label("Keyboard shortcuts").last().expect("the help window lists the shortcuts").click();
    h.run_steps(3);
    assert_eq!(h.state().dialog, Some(Dialog::Shortcuts));
}
