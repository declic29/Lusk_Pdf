//! Lusk PDF : le correcteur d'orthographe des zones de texte, repris de Lusk Texte (crate
//! `lusk-proof`, dictionnaires français, breton et anglais intégrés au programme).
//!
//! Les mots inconnus des commentaires, des réponses et du texte ajouté par « Remplir et signer »
//! sont soulignés d'une vague rouge ; un clic droit sur l'un d'eux propose des corrections,
//! « Tout ignorer » (jusqu'à la fermeture) et « Ajouter au dictionnaire » (le dictionnaire
//! personnel, gardé dans le dossier de données). La langue se choisit dans Préférences ›
//! Documents et affichage › Orthographe ; le français est la langue par défaut.
//!
//! Le dictionnaire de la langue choisie se charge en arrière-plan au démarrage ([`start`]) :
//! tant qu'il n'est pas prêt, rien n'est souligné, et l'interface ne se fige jamais.

use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::sync::atomic::{AtomicU8, Ordering};

use egui::text::CCursor;
use egui::{Color32, Id, Pos2, Stroke, Ui};
use lusk_proof::ProofLang;

/// The language text fields are checked in (Preferences ▸ Spelling).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum SpellLang {
    French,
    Breton,
    English,
    Off,
}

impl SpellLang {
    /// In the order Preferences lists them.
    pub const ALL: [SpellLang; 4] = [SpellLang::French, SpellLang::Breton, SpellLang::English, SpellLang::Off];

    /// How the settings file stores it.
    pub fn key(self) -> &'static str {
        match self {
            SpellLang::French => "fr",
            SpellLang::Breton => "br",
            SpellLang::English => "en",
            SpellLang::Off => "off",
        }
    }

    /// The language a settings key names; `None` for anything else.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|l| l.key() == key)
    }

    /// Its name in the interface language.
    pub fn label(self) -> &'static str {
        match self {
            SpellLang::French => tl!("French"),
            SpellLang::Breton => tl!("Breton"),
            SpellLang::English => tl!("English"),
            SpellLang::Off => tl!("Off"),
        }
    }

    fn proof(self) -> Option<ProofLang> {
        match self {
            SpellLang::French => Some(ProofLang::Fr),
            SpellLang::Breton => Some(ProofLang::Br),
            SpellLang::English => Some(ProofLang::En),
            SpellLang::Off => None,
        }
    }

    fn bit(self) -> u8 {
        match self {
            SpellLang::French => 1,
            SpellLang::Breton => 2,
            SpellLang::English => 4,
            SpellLang::Off => 0,
        }
    }

    fn index(self) -> u8 {
        match self {
            SpellLang::French => 0,
            SpellLang::Breton => 1,
            SpellLang::English => 2,
            SpellLang::Off => 3,
        }
    }
}

/// The chosen language ([`SpellLang::index`]); French until the settings say otherwise.
static LANG: AtomicU8 = AtomicU8::new(0);
/// The languages whose dictionary has loaded ([`SpellLang::bit`]).
static READY: AtomicU8 = AtomicU8::new(0);

/// The language text fields are checked in.
pub fn lang() -> SpellLang {
    let i = LANG.load(Ordering::Relaxed);
    SpellLang::ALL.into_iter().find(|l| l.index() == i).unwrap_or(SpellLang::French)
}

/// Choose the language (Preferences, or the settings file), and load its dictionary in the
/// background once [`start`] has run.
pub fn set_lang(l: SpellLang) {
    let before = LANG.swap(l.index(), Ordering::Relaxed);
    if before != l.index() && STARTED.load(Ordering::Relaxed) != 0 {
        warm_up(l);
    }
}

static STARTED: AtomicU8 = AtomicU8::new(0);

/// At start-up, once the settings are read: load the personal dictionary and, in the
/// background, the chosen language's dictionary.
pub fn start() {
    STARTED.store(1, Ordering::Relaxed);
    if let Some(path) = user_dictionary_path() {
        let added = lusk_proof::load_user_dictionary(&path);
        if !added.is_empty() {
            log::info!("orthographe : {} mot(s) du dictionnaire personnel", added.len());
        }
    }
    warm_up(lang());
}

/// Load `l`'s dictionary off the interface thread, then let fields be checked in it.
fn warm_up(l: SpellLang) {
    let Some(proof) = l.proof() else { return };
    if READY.load(Ordering::Relaxed) & l.bit() != 0 {
        return;
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let spawned = std::thread::Builder::new().name("lusk-orthographe".into()).spawn(move || {
            let _ = lusk_proof::check_spelling_in("Lusk", proof);
            READY.fetch_or(l.bit(), Ordering::Relaxed);
        });
        if let Err(e) = spawned {
            log::warn!("orthographe : dictionnaire non chargé : {e}");
        }
    }
    #[cfg(target_arch = "wasm32")]
    {
        let _ = lusk_proof::check_spelling_in("Lusk", proof);
        READY.fetch_or(l.bit(), Ordering::Relaxed);
    }
}

/// The language fields are checked in now: `None` when spelling is off or its dictionary is
/// still loading.
fn active() -> Option<(SpellLang, ProofLang)> {
    let l = lang();
    let proof = l.proof()?;
    (READY.load(Ordering::Relaxed) & l.bit() != 0).then_some((l, proof))
}

/// The personal dictionary: `orthographe.txt` beside the OCR dictionaries (the portable data
/// folder in portable mode). `None` on the web.
fn user_dictionary_path() -> Option<std::path::PathBuf> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        crate::persist::recovery::default_dictionary_dir().map(|d| d.join("orthographe.txt"))
    }
    #[cfg(target_arch = "wasm32")]
    {
        None
    }
}

fn save_user_dictionary() {
    let Some(path) = user_dictionary_path() else { return };
    if let Some(dir) = path.parent()
        && let Err(e) = std::fs::create_dir_all(dir)
    {
        log::warn!("orthographe : {} : {e}", dir.display());
        return;
    }
    if let Err(e) = lusk_proof::save_user_dictionary(&path, &[]) {
        log::warn!("orthographe : {} : {e}", path.display());
    }
}

/// The misspelled words of `text` in `lang`, as byte ranges.
pub fn misspelled(text: &str, lang: ProofLang) -> Vec<(usize, usize)> {
    lusk_proof::check_spelling_in(text, lang).into_iter().map(|i| (i.start, i.end)).filter(|(a, b)| a < b && text.get(*a..*b).is_some()).collect()
}

/// A field's last check, kept while its text, the language and the dictionaries stay the same.
#[derive(Clone)]
struct Checked {
    key: u64,
    words: Arc<Vec<(usize, usize)>>,
}

/// The word a right click landed on, while its menu is open.
#[derive(Clone)]
struct Target {
    start: usize,
    end: usize,
    word: String,
    suggestions: Vec<String>,
}

/// The longest text checked: a pasted book is not checked on every frame.
const MAX_CHECKED_BYTES: usize = 20_000;

/// Underline the misspelled words of a text field just shown, and offer corrections on a right
/// click on one of them. Call it right after `TextEdit::show`, with the field's text. Returns
/// whether a correction changed `text`.
pub fn check(ui: &Ui, out: &egui::text_edit::TextEditOutput, text: &mut String) -> bool {
    let Some((l, proof)) = active() else { return false };
    if text.is_empty() || text.len() > MAX_CHECKED_BYTES {
        return false;
    }
    let id = out.response.response.id;
    let words = cached_check(ui, id, text, l, proof);
    let galley = &out.galley;
    // The word being typed is left alone until the cursor leaves it.
    let typing_at = out.response.response.has_focus().then(|| out.cursor_range.map(|r| r.primary.index.0)).flatten();
    let painter = ui.painter().with_clip_rect(out.text_clip_rect);
    let colour = Color32::from_rgb(0xD9, 0x2D, 0x20);
    for &(a, b) in words.iter() {
        let (Some(ca), Some(cb)) = (char_index(text, a), char_index(text, b)) else { continue };
        if typing_at == Some(cb) {
            continue;
        }
        let ra = galley.pos_from_cursor(CCursor::new(ca)).translate(out.galley_pos.to_vec2());
        let rb = galley.pos_from_cursor(CCursor::new(cb)).translate(out.galley_pos.to_vec2());
        if (ra.bottom() - rb.bottom()).abs() < 1.0 {
            wavy(&painter, ra.left(), rb.left(), ra.bottom() - 1.0, colour);
        } else {
            // A word longer than the field wraps: underline both of its rows.
            let rect = galley.rect.translate(out.galley_pos.to_vec2());
            wavy(&painter, ra.left(), rect.right(), ra.bottom() - 1.0, colour);
            wavy(&painter, rect.left(), rb.left(), rb.bottom() - 1.0, colour);
        }
    }

    let response = &out.response.response;
    let target_id = id.with("lusk-spelling-target");
    if response.secondary_clicked() {
        let target = response.interact_pointer_pos().and_then(|p| word_under(text, galley, out.galley_pos, p, &words, proof));
        ui.data_mut(|d| match target {
            Some(t) => {
                d.insert_temp(target_id, t);
            }
            None => d.remove::<Target>(target_id),
        });
    }
    let Some(target) = ui.data(|d| d.get_temp::<Target>(target_id)) else { return false };
    let mut changed = false;
    let shown = response.context_menu(|ui| {
        changed = menu(ui, &target, text);
    });
    if shown.is_none() || changed {
        ui.data_mut(|d| d.remove::<Target>(target_id));
    }
    changed
}

fn cached_check(ui: &Ui, id: Id, text: &str, l: SpellLang, proof: ProofLang) -> Arc<Vec<(usize, usize)>> {
    let mut h = std::collections::hash_map::DefaultHasher::new();
    text.hash(&mut h);
    l.hash(&mut h);
    lusk_proof::dictionary_generation().hash(&mut h);
    let key = h.finish();
    let cache_id = id.with("lusk-spelling");
    if let Some(c) = ui.data(|d| d.get_temp::<Checked>(cache_id)).filter(|c| c.key == key) {
        return c.words;
    }
    let words = Arc::new(misspelled(text, proof));
    ui.data_mut(|d| d.insert_temp(cache_id, Checked { key, words: words.clone() }));
    words
}

/// The misspelled word under `pointer`, with its suggestions.
fn word_under(text: &str, galley: &egui::Galley, galley_pos: Pos2, pointer: Pos2, words: &[(usize, usize)], proof: ProofLang) -> Option<Target> {
    let at = galley.cursor_from_pos(pointer - galley_pos).index.0;
    let byte = text.char_indices().nth(at).map_or(text.len(), |(i, _)| i);
    let &(start, end) = words.iter().find(|(a, b)| *a <= byte && byte <= *b)?;
    let word = text.get(start..end)?.to_owned();
    let suggestions = lusk_proof::suggest_in(&word, 6, proof);
    Some(Target { start, end, word, suggestions })
}

/// The right-click menu on a misspelled word. Returns whether it changed `text`.
fn menu(ui: &mut Ui, t: &Target, text: &mut String) -> bool {
    let mut changed = false;
    if t.suggestions.is_empty() {
        ui.add_enabled(false, egui::Button::new(tl!("No suggestions")));
    }
    for s in &t.suggestions {
        if ui.button(egui::RichText::new(s).strong()).clicked() {
            // Only if the word is still where it was clicked.
            if text.get(t.start..t.end) == Some(t.word.as_str()) {
                text.replace_range(t.start..t.end, s);
                changed = true;
            }
            ui.close();
        }
    }
    ui.separator();
    if ui.button(tl!("Ignore All")).clicked() {
        lusk_proof::ignore_word(&t.word);
        ui.close();
    }
    if ui.button(tl!("Add to Dictionary")).clicked() {
        lusk_proof::add_word(&t.word);
        save_user_dictionary();
        ui.close();
    }
    changed
}

/// The character index of byte `b` of `text` (`None` off a character boundary).
fn char_index(text: &str, b: usize) -> Option<usize> {
    text.get(..b).map(|s| s.chars().count())
}

/// A red wavy line from `x0` to `x1` at `y`, as word processors underline spelling.
fn wavy(painter: &egui::Painter, x0: f32, x1: f32, y: f32, colour: Color32) {
    if !(x0.is_finite() && x1.is_finite() && y.is_finite()) || x1 - x0 < 2.0 {
        return;
    }
    let step = 2.0;
    let mut points = Vec::new();
    let mut x = x0;
    let mut up = false;
    while x < x1 && points.len() < 4096 {
        points.push(Pos2::new(x, if up { y - 1.5 } else { y }));
        up = !up;
        x += step;
    }
    points.push(Pos2::new(x1, if up { y - 1.5 } else { y }));
    painter.add(egui::Shape::line(points, Stroke::new(1.0, colour)));
}

/// Lets tests check fields without waiting for the background load.
#[cfg(test)]
pub(crate) fn ready_for_tests(l: SpellLang) {
    let _ = l.proof().map(|p| lusk_proof::check_spelling_in("Lusk", p));
    READY.fetch_or(l.bit(), Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn languages_round_trip_through_the_settings() {
        for l in SpellLang::ALL {
            assert_eq!(SpellLang::from_key(l.key()), Some(l));
        }
        assert_eq!(SpellLang::from_key("klingon"), None);
    }

    #[test]
    fn french_is_the_default_and_finds_mistakes() {
        assert_eq!(lang(), SpellLang::French);
        let text = "Bonjour, voici ma maisson.";
        let words = misspelled(text, ProofLang::Fr);
        assert_eq!(words.iter().map(|&(a, b)| &text[a..b]).collect::<Vec<_>>(), ["maisson"]);
        assert!(misspelled("Le renard traverse la rivière.", ProofLang::Fr).is_empty());
    }

    #[test]
    fn breton_is_checked_too() {
        assert!(misspelled("Demat d'an holl", ProofLang::Br).is_empty());
        assert!(!misspelled("Demat zqxwv", ProofLang::Br).is_empty());
    }

    #[test]
    fn suggestions_offer_the_right_word() {
        assert!(lusk_proof::suggest_in("maisson", 6, ProofLang::Fr).iter().any(|s| s == "maison"));
    }

    #[test]
    fn char_index_counts_characters() {
        assert_eq!(char_index("été là", 5), Some(3));
        assert_eq!(char_index("été", 1), None);
    }

    /// A field shows its mistake, and a right click on it offers the correction, which replaces it.
    #[test]
    fn a_right_click_corrects_the_word() {
        use egui_kittest::kittest::Queryable;
        ready_for_tests(SpellLang::French);
        let mut h = egui_kittest::Harness::builder().with_size(egui::vec2(320.0, 120.0)).build_ui_state(
            |ui, text: &mut String| {
                let out = egui::TextEdit::multiline(text).desired_width(f32::INFINITY).id_salt("spell-test").show(ui);
                check(ui, &out, text);
                let _ = out.response.response.accessible_name("Champ");
            },
            "maisson".to_owned(),
        );
        h.run_steps(2);
        if let Ok(path) = std::env::var("LUSK_SPELL_SHOT") {
            h.get_by_label("Champ").click_secondary();
            h.run_steps(2);
            h.render().expect("rendered").save(path).expect("saved");
        } else {
            h.get_by_label("Champ").click_secondary();
            h.run_steps(2);
        }
        h.get_by_label("maison").click();
        h.run_steps(2);
        assert_eq!(h.state(), "maison");
    }
}
