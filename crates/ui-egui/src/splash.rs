//! L'écran d'accueil de Lusk PDF (comme ceux de Lusk Texte et de Lusk Tab) : une fenêtre
//! rectangulaire sans bordure, centrée, qui apparaît en fondu, montre le logo (en fondu lui
//! aussi), les mentions légales et une barre de chargement, puis disparaît en fondu. La fenêtre
//! de travail s'ouvre ensuite, à sa taille et à sa place habituelles.
//!
//! L'écran a sa propre petite fenêtre, ouverte par `apps/pdfcraft` avant celle de l'application
//! (voir `run_splash` dans `apps/pdfcraft/src/main.rs`) : la fenêtre de travail garde ainsi sa
//! taille et sa place enregistrées, sans détour.
//!
//! La barre avance par étapes et reste au moins [`MIN_SECS`] à l'écran pour qu'on la voie.

use egui::{Align2, Color32, CornerRadius, FontFamily, FontId, Rect, Sense, pos2, vec2};

/// Taille de l'écran d'accueil (points), au format de son image (16:9).
pub const SIZE: [f32; 2] = [720.0, 405.0];
/// Durée minimale d'affichage de la barre.
pub const MIN_SECS: f64 = 1.6;
const FADE_IN: f64 = 0.45;
const FADE_OUT: f64 = 0.45;
const LOGO_DELAY: f64 = 0.25;
const LOGO_FADE: f64 = 0.7;
/// Un court instant barre pleine avant le fondu de sortie.
const HOLD: f64 = 0.15;

// ─── Textes de l'écran d'accueil : à modifier ici ──────────────────────────────────────────
// `{version}` est remplacé par le numéro de version de Lusk PDF. Après une modification,
// recompilez (`cargo run --release -p pdfcraft`) pour voir le résultat.

/// Mentions légales, première ligne, à gauche.
pub const LEGAL_LEFT: &str = "Version {version}  ·  Portée par une association loi 1901";
/// Mentions légales, première ligne, à droite.
pub const LEGAL_RIGHT: &str = "© 2026 Lusk";
/// Mentions légales, seconde ligne, à gauche (laisser vide pour n'avoir qu'une ligne).
pub const LEGAL_SECOND: &str = "Logiciel libre sous licence MIT ou Apache 2.0";

/// Les étapes de la barre : à partir de quelle part du chargement chacune s'affiche.
const STEPS: &[(f32, &str)] =
    &[(0.0, "Chargement de l'interface…"), (0.35, "Chargement des polices…"), (0.7, "Préparation de l'espace de travail…"), (1.0, "Prêt")];

// ──────────────────────────────────────────────────────────────────────────────────────────

/// Où en est l'écran d'accueil.
#[derive(Debug, Default)]
pub struct Splash {
    start: Option<f64>,
    frames: u32,
    shown: f32,
    full_at: Option<f64>,
    /// Effets d'animation coupés : pas de fondu, la barre seule.
    still: bool,
}

impl Splash {
    pub fn new(animations: bool) -> Self {
        Splash { still: !animations, ..Splash::default() }
    }

    /// Les polices de l'écran : Inter, la police de l'interface, devant celles d'egui.
    pub fn install_fonts(ctx: &egui::Context) {
        let mut fonts = egui::FontDefinitions::default();
        if let Some(inter) = pdfcraft_fonts::document_latin_fonts(false).first() {
            fonts.font_data.insert("lusk-inter".into(), std::sync::Arc::new(egui::FontData::from_static(inter.bytes)));
            if let Some(family) = fonts.families.get_mut(&FontFamily::Proportional) {
                family.insert(0, "lusk-inter".into());
            }
        }
        ctx.set_fonts(fonts);
    }

    /// Dessine l'écran d'accueil pour cette image. `true` quand il a fini (fondu de sortie
    /// terminé) : la fenêtre de l'écran se ferme alors.
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let ctx = ui.ctx().clone();
        let now = ctx.input(|i| i.time);
        let start = *self.start.get_or_insert_with(|| {
            log::info!("écran d'accueil : première image, {:?} points à l'échelle {}", ctx.content_rect().size(), ctx.pixels_per_point());
            now
        });
        let t = now - start;
        // La barre se remplit à son rythme, jamais en arrière.
        let paced = if self.frames < 2 { 0.05 } else { ((t / MIN_SECS) as f32).min(1.0) };
        self.shown = self.shown.max(paced).min(1.0);
        if self.shown >= 1.0 && self.full_at.is_none() {
            self.full_at = Some(now);
        }
        let status = STEPS.iter().rev().find(|(from, _)| self.shown >= *from).map_or("Chargement…", |(_, s)| s);
        let out = self.full_at.map_or(0.0, |f| ((now - f - HOLD) / FADE_OUT).clamp(0.0, 1.0));
        let alpha = if self.still {
            1.0
        } else {
            let inn = (t / FADE_IN).clamp(0.0, 1.0);
            (ease(inn) * (1.0 - ease(out))) as f32
        };
        let logo = if self.still { 1.0 } else { ease(((t - LOGO_DELAY) / LOGO_FADE).clamp(0.0, 1.0)) as f32 * alpha };
        paint(ui, Paint { alpha, logo, progress: self.shown, status, text: self.frames >= 1 });
        self.frames = self.frames.saturating_add(1);
        let done = self.full_at.is_some_and(|f| now - f >= HOLD + if self.still { 0.0 } else { FADE_OUT });
        if done {
            log::info!("écran d'accueil : terminé après {t:.2} s et {} images", self.frames);
        } else {
            ctx.request_repaint();
        }
        done
    }
}

/// Un texte de l'écran avec le numéro de version à la place de `{version}`.
fn with_version(text: &str) -> String {
    text.replace("{version}", env!("CARGO_PKG_VERSION"))
}

fn ease(p: f64) -> f64 {
    // Doux au départ et à l'arrivée.
    p * p * (3.0 - 2.0 * p)
}

struct Paint<'a> {
    alpha: f32,
    logo: f32,
    progress: f32,
    status: &'a str,
    /// Le texte attend la deuxième image, quand les polices sont installées.
    text: bool,
}

/// Une image intégrée, chargée une fois en texture ; une image illisible (impossible avec les
/// fichiers intégrés) donne un carré de `fallback`, jamais un plantage.
fn texture(ctx: &egui::Context, name: &'static str, bytes: &[u8], fallback: [u8; 4]) -> egui::TextureHandle {
    let id = egui::Id::unique(("lusk-splash", name));
    if let Some(h) = ctx.data(|d| d.get_temp::<egui::TextureHandle>(id)) {
        return h;
    }
    // Au plus 1440 pixels de côté (deux fois la taille affichée), et jamais plus que ce que la
    // carte graphique accepte.
    let side = ctx.input(|i| i.max_texture_side).clamp(1, 1440) as u32;
    let img = image::load_from_memory(bytes)
        .map(|i| if i.width() > side || i.height() > side { i.resize(side, side, image::imageops::FilterType::Triangle) } else { i })
        .map(|i| i.to_rgba8())
        .unwrap_or_else(|_| image::RgbaImage::from_pixel(1, 1, image::Rgba(fallback)));
    let h = ctx.load_texture(
        name,
        egui::ColorImage::from_rgba_unmultiplied([img.width() as usize, img.height() as usize], img.as_raw()),
        egui::TextureOptions::LINEAR,
    );
    ctx.data_mut(|d| d.insert_temp(id, h.clone()));
    h
}

fn paint(ui: &mut egui::Ui, p: Paint<'_>) {
    let ctx = ui.ctx().clone();
    let r = ctx.content_rect();
    let (_, resp) = ui.allocate_exact_size(r.size(), Sense::hover());
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Image, true, format!("Lusk PDF — {}", p.status)));
    let painter = ui.painter();
    let fade = |c: Color32, a: f32| c.gamma_multiply(a.clamp(0.0, 1.0));
    let corner = CornerRadius::same(10);
    let bg = texture(&ctx, "lusk_splash_background", include_bytes!("../../../assets/splash/accueil-lusk-pdf.jpg"), [0xA3, 0x1B, 0x1F, 255]);
    let uv = Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0));
    painter.add(egui::epaint::RectShape::filled(r, corner, fade(Color32::WHITE, p.alpha)).with_texture(bg.id(), uv));
    if !p.text {
        return;
    }
    let (w, h) = (r.width(), r.height());
    // Le logo seul, sur la partie claire de l'image, au centre.
    let logo = texture(&ctx, "lusk_splash_logo", include_bytes!("../../../assets/app-icon/lusk-pdf-logo.png"), [0, 0, 0, 0]);
    let size = logo.size_vec2();
    let lh = 150.0;
    let lw = lh * size.x / size.y.max(1.0);
    let logo_rect = Rect::from_center_size(pos2(r.min.x + w * 0.5, r.min.y + h * 0.40), vec2(lw, lh));
    painter.image(logo.id(), logo_rect, uv, fade(Color32::WHITE, p.logo));
    // Un voile sombre en dégradé sous le bas de l'image, pour lire le texte blanc sur les
    // vagues (le clair au milieu).
    let veil_top = r.max.y - 118.0;
    let mut veil = egui::Mesh::default();
    let clear = Color32::from_black_alpha(0);
    let dark = fade(Color32::from_black_alpha(130), p.alpha);
    veil.colored_vertex(pos2(r.min.x, veil_top), clear);
    veil.colored_vertex(pos2(r.max.x, veil_top), clear);
    veil.colored_vertex(pos2(r.max.x, r.max.y), dark);
    veil.colored_vertex(pos2(r.min.x, r.max.y), dark);
    veil.add_triangle(0, 1, 2);
    veil.add_triangle(0, 2, 3);
    painter.add(veil);
    // La barre de chargement et l'étape en cours, sur les vagues du bas.
    let white = |a: f32| fade(Color32::WHITE, a * p.alpha);
    let text = |pos: egui::Pos2, anchor: Align2, s: &str, size: f32, c: Color32| {
        painter.text(pos, anchor, s, FontId::proportional(size), c);
    };
    let bar = Rect::from_min_size(pos2(r.min.x + 40.0, r.max.y - 66.0), vec2(w - 80.0, 4.0));
    painter.rect_filled(bar, CornerRadius::same(2), white(0.3));
    let fill = Rect::from_min_size(bar.min, vec2(bar.width() * p.progress.clamp(0.0, 1.0), bar.height()));
    painter.rect_filled(fill, CornerRadius::same(2), white(0.95));
    text(pos2(bar.min.x, bar.min.y - 10.0), Align2::LEFT_BOTTOM, p.status, 12.0, white(0.95));
    // Les mentions légales, discrètes, sur deux lignes.
    let (y1, y2) = (r.max.y - 34.0, r.max.y - 18.0);
    text(pos2(bar.min.x, y1), Align2::LEFT_CENTER, &with_version(LEGAL_LEFT), 10.5, white(0.9));
    text(pos2(bar.max.x, y1), Align2::RIGHT_CENTER, &with_version(LEGAL_RIGHT), 10.5, white(0.9));
    text(pos2(bar.min.x, y2), Align2::LEFT_CENTER, &with_version(LEGAL_SECOND), 10.5, white(0.8));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the screen at 60 images a second; the time it says it has finished.
    fn run(splash: &mut Splash) -> Option<f64> {
        let ctx = egui::Context::default();
        Splash::install_fonts(&ctx);
        for frame in 0..900 {
            let time = f64::from(frame) / 60.0;
            let input = egui::RawInput {
                time: Some(time),
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(SIZE[0], SIZE[1]))),
                ..Default::default()
            };
            let mut done = false;
            let mut out = ctx.run_ui(input, |ui| {
                done = splash.ui(ui);
            });
            out.textures_delta.clear();
            if done {
                return Some(time);
            }
        }
        None
    }

    /// The screen runs its course: it fills, fades out and says it has finished, after at least
    /// the minimum time, and soon after.
    #[test]
    fn the_start_screen_fills_then_finishes() {
        let mut splash = Splash::new(true);
        let at = run(&mut splash).expect("the start screen finishes");
        assert!(at >= MIN_SECS + FADE_OUT, "shown at least {MIN_SECS} s then faded, finished at {at}");
        assert!(at < MIN_SECS + HOLD + FADE_OUT + 0.5, "finished at {at}");
        assert!((splash.shown - 1.0).abs() < f32::EPSILON);
    }

    /// The texts show the version and don't name PdfCraft or another Lusk app.
    #[test]
    fn the_legal_lines_show_the_version() {
        assert_eq!(with_version(LEGAL_LEFT), format!("Version {}  ·  Portée par une association loi 1901", env!("CARGO_PKG_VERSION")));
        for t in [LEGAL_LEFT, LEGAL_RIGHT, LEGAL_SECOND] {
            assert!(!t.contains("PdfCraft") && !t.contains("Texte") && !t.contains("Tab"), "{t}");
        }
    }

    /// With animation effects off there is no fade: it ends as soon as the bar has been full a
    /// moment.
    #[test]
    fn without_animations_it_does_not_fade() {
        let mut splash = Splash::new(false);
        let at = run(&mut splash).expect("the start screen finishes");
        let full = splash.full_at.unwrap_or_default();
        assert!(at - full < FADE_OUT, "no fade-out: {at} after full at {full}");
    }

    /// `LUSK_SPLASH_SHOT=<file.png>` saves a picture of the screen halfway through, to look at.
    #[test]
    fn the_start_screen_can_be_pictured() {
        let Ok(path) = std::env::var("LUSK_SPLASH_SHOT") else { return };
        let mut h = egui_kittest::Harness::builder().with_size(vec2(SIZE[0], SIZE[1])).build_ui_state(
            |ui, splash: &mut Splash| {
                splash.ui(ui);
            },
            // Without the fades, so the picture doesn't depend on the harness's clock.
            Splash::new(false),
        );
        Splash::install_fonts(&h.ctx);
        h.run_steps(3);
        h.render().expect("rendered").save(path).expect("saved");
    }

    /// The embedded pictures decode: the screen never shows the fallback square.
    #[test]
    fn the_pictures_decode() {
        for bytes in
            [&include_bytes!("../../../assets/splash/accueil-lusk-pdf.jpg")[..], &include_bytes!("../../../assets/app-icon/lusk-pdf-logo.png")[..]]
        {
            let img = image::load_from_memory(bytes).expect("decodes");
            assert!(img.width() > 100 && img.height() > 100);
        }
    }
}
