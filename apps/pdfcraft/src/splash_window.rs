//! Lusk PDF : la petite fenêtre de l'écran d'accueil (`pdfcraft_ui_egui::splash`), ouverte avant
//! la fenêtre de travail et refermée quand l'écran a fini.
//!
//! Sans bordure, transparente (pour ses coins arrondis et ses fondus), centrée, au premier plan.
//! Elle n'utilise pas les préférences : la fenêtre de travail garde sa taille et sa place.
//! Si elle ne peut pas s'ouvrir (pas de carte graphique, pas d'écran), l'application démarre
//! quand même, sans écran d'accueil.

use pdfcraft_ui_egui::splash::{SIZE, Splash};

/// Whether this start shows the start-up screen: not on macOS (the app's window there has the
/// Dock and the menu bar), not with the UI control channel (agents and screenshots), and not when
/// `LUSK_NO_SPLASH` is set.
pub fn wanted(control: bool, no_splash_env: bool) -> bool {
    !cfg!(target_os = "macos") && !control && !no_splash_env
}

struct SplashApp {
    splash: Splash,
    done: bool,
}

impl eframe::App for SplashApp {
    fn persist_egui_memory(&self) -> bool {
        false
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if !self.done && self.splash.ui(ui) {
            self.done = true;
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    /// Transparent: the screen draws its own rounded picture.
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0; 4]
    }
}

/// Shows the start-up screen and returns once it has finished (or couldn't show).
pub fn run(renderer: eframe::Renderer, app_id: &str, icon: &[u8]) {
    let mut viewport = egui::ViewportBuilder::default()
        .with_title("Lusk PDF")
        .with_decorations(false)
        .with_transparent(true)
        .with_resizable(false)
        .with_inner_size(SIZE)
        .with_min_inner_size(SIZE)
        .with_window_level(egui::WindowLevel::AlwaysOnTop)
        .with_taskbar(false)
        .with_app_id(app_id);
    match eframe::icon_data::from_png_bytes(icon) {
        Ok(icon) => viewport = viewport.with_icon(icon),
        Err(e) => log::warn!("écran d'accueil : icône : {e}"),
    }
    // Its own throwaway settings file, never the app's: nothing of this window is kept.
    let persistence_path = Some(std::env::temp_dir().join("lusk-pdf-accueil.ron"));
    let mut options = eframe::NativeOptions { viewport, centered: true, renderer, persist_window: false, persistence_path, ..Default::default() };
    // DirectX 12 shows a transparent window only through DirectComposition.
    #[cfg(target_os = "windows")]
    if std::env::var_os("WGPU_DX12_PRESENTATION_SYSTEM").is_none()
        && let eframe::egui_wgpu::WgpuSetup::CreateNew(create) = &mut options.wgpu_options.wgpu_setup
    {
        create.instance_descriptor.backend_options.dx12.presentation_system = eframe::wgpu::Dx12SwapchainKind::DxgiFromVisual;
    }
    options.system_font_fallback = false;
    let result = eframe::run_native(
        "Lusk PDF (accueil)",
        options,
        Box::new(|cc| {
            Splash::install_fonts(&cc.egui_ctx);
            Ok(Box::new(SplashApp { splash: Splash::new(true), done: false }))
        }),
    );
    if let Err(e) = result {
        log::warn!("écran d'accueil non affiché : {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_screen_is_skipped_for_agents_and_on_request() {
        assert!(!wanted(true, false), "never with the control channel");
        assert!(!wanted(false, true), "never with LUSK_NO_SPLASH");
        assert_eq!(wanted(false, false), !cfg!(target_os = "macos"));
    }
}
