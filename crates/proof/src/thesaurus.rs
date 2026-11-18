//! Lusk PDF : le dictionnaire des synonymes, repris de Lusk Diapo (`crates/engine/src/thesaurus.rs`
//! de declic29/Lusk-Diapo). Synonymes cherchés sans connexion : le français vient du « Dictionnaire
//! des synonymes français » myThes 2.3 de Dicollecte (`assets/thesaurus/thesaurus_fr.txt`, LGPL 2.1
//! ou ultérieure), l'anglais d'une petite liste écrite pour DeckCraft
//! (`assets/thesaurus/thesaurus_en.txt`).

use std::collections::HashMap;
use std::sync::OnceLock;

const DATA: &str = include_str!("../../../assets/thesaurus/thesaurus_en.txt");
const DATA_FR: &str = include_str!("../../../assets/thesaurus/thesaurus_fr.txt");

/// One sense of a word: its headword, its part of speech (French list: « nom », « verbe »…;
/// empty in the English one) and synonyms.
#[derive(Clone, Debug, PartialEq)]
pub struct Sense {
    pub word: String,
    pub pos: String,
    pub synonyms: Vec<String>,
}

fn english_senses(head: &str) -> Vec<Sense> {
    DATA.lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| l.split_once('|'))
        .filter(|(h, _)| h.trim() == head)
        .map(|(h, s)| Sense {
            word: h.trim().to_string(),
            pos: String::new(),
            synonyms: s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
        })
        .collect()
}

/// The French list by headword (lower case): its lines `mot|nature|synonymes`, read once.
fn french_index() -> &'static HashMap<String, Vec<&'static str>> {
    static INDEX: OnceLock<HashMap<String, Vec<&'static str>>> = OnceLock::new();
    INDEX.get_or_init(|| {
        let mut m: HashMap<String, Vec<&'static str>> = HashMap::new();
        for l in DATA_FR.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            if let Some((h, _)) = l.split_once('|') {
                m.entry(h.to_lowercase()).or_default().push(l);
            }
        }
        m
    })
}

fn french_senses(head: &str) -> Vec<Sense> {
    french_index()
        .get(head)
        .into_iter()
        .flatten()
        .filter_map(|l| {
            let mut parts = l.splitn(3, '|');
            let (word, pos, syn) = (parts.next()?, parts.next()?, parts.next()?);
            Some(Sense {
                word: word.to_string(),
                pos: pos.to_string(),
                synonyms: syn.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect(),
            })
        })
        .collect()
}

/// Forms of a French word to try after the word itself: without an elided article (« l'idée »
/// → « idée »), singular (« projets », « chevaux »), masculine (« grande », « heureuse ») and the
/// infinitive of a few common verb endings (« présentons » → « présenter »).
fn french_forms(w: &str) -> Vec<String> {
    let mut out = Vec::new();
    let w = ["l'", "l’", "d'", "d’", "qu'", "qu’", "j'", "j’", "n'", "n’", "s'", "s’", "c'", "c’", "m'", "m’", "t'", "t’"]
        .iter()
        .find_map(|p| w.strip_prefix(p))
        .unwrap_or(w);
    out.push(w.to_string());
    let mut bases = vec![w.to_string()];
    if let Some(s) = w.strip_suffix("aux") {
        bases.push(format!("{s}al"));
    }
    for suf in ["s", "x"] {
        if let Some(s) = w.strip_suffix(suf) {
            bases.push(s.to_string());
        }
    }
    for b in bases.clone() {
        for (from, to) in [("euse", "eux"), ("trice", "teur"), ("ive", "if"), ("ienne", "ien"), ("onne", "on"), ("ère", "er"), ("e", "")] {
            if let Some(s) = b.strip_suffix(from)
                && s.chars().count() >= 2
            {
                bases.push(format!("{s}{to}"));
            }
        }
    }
    out.extend(bases);
    for (suf, inf) in [
        ("ons", "er"),
        ("ez", "er"),
        ("ent", "er"),
        ("ait", "er"),
        ("ais", "er"),
        ("aient", "er"),
        ("é", "er"),
        ("ée", "er"),
        ("és", "er"),
        ("ées", "er"),
        ("ant", "er"),
        ("era", "er"),
        ("issons", "ir"),
        ("issez", "ir"),
        ("issent", "ir"),
        ("it", "ir"),
        ("i", "ir"),
        ("is", "ir"),
    ] {
        if let Some(s) = w.strip_suffix(suf)
            && s.chars().count() >= 2
        {
            out.push(format!("{s}{inf}"));
        }
    }
    out
}

/// The senses for `word`, trying simple English endings when the word itself isn't listed
/// ("goals" → "goal", "improved" → "improve", "growing" → "grow"). Synonyms take the word's
/// capitalisation ("Goal" → "Aim", "GOAL" → "AIM").
/// French senses come first when `french_first` (French spelling), English ones otherwise.
pub fn lookup(word: &str, french_first: bool) -> Vec<Sense> {
    let w: String = word.trim().trim_matches(|c: char| !c.is_alphanumeric() && c != '-').chars().take(64).collect();
    let lower = w.to_lowercase();
    if lower.is_empty() {
        return vec![];
    }
    let mut tries = vec![lower.clone()];
    for (suffix, add) in [("ies", "y"), ("es", ""), ("s", ""), ("ied", "y"), ("ed", "e"), ("ed", ""), ("ing", "e"), ("ing", "")] {
        if let Some(stem) = lower.strip_suffix(suffix)
            && stem.chars().count() >= 2
        {
            tries.push(format!("{stem}{add}"));
        }
    }
    let french = french_forms(&lower).iter().map(|t| french_senses(t)).find(|v| !v.is_empty()).unwrap_or_default();
    let english = tries.iter().map(|t| english_senses(t)).find(|v| !v.is_empty()).unwrap_or_default();
    let found: Vec<Sense> = if french_first { french.into_iter().chain(english).collect() } else { english.into_iter().chain(french).collect() };
    if found.is_empty() {
        return vec![];
    }
    let upper = w.chars().count() > 1 && w.chars().all(|c| !c.is_lowercase());
    let title = !upper && w.chars().next().is_some_and(char::is_uppercase);
    found
        .into_iter()
        .map(|s| Sense {
            word: s.word,
            pos: s.pos,
            synonyms: s
                .synonyms
                .into_iter()
                .map(|x| {
                    if upper {
                        x.to_uppercase()
                    } else if title {
                        let mut c = x.chars();
                        c.next().map(|f| f.to_uppercase().chain(c).collect()).unwrap_or_default()
                    } else {
                        x
                    }
                })
                .collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn english_senses_endings_and_case() {
        let g = lookup("goal", false);
        assert!(g[0].synonyms.contains(&"objective".to_string()));
        assert_eq!(lookup("Goals", false)[0].synonyms[0], "Aim");
        assert_eq!(lookup("GOAL", false)[0].synonyms[0], "AIM");
        assert_eq!(lookup("improved", false)[0].word, "improve");
        assert_eq!(lookup("“goal,”", false)[0].word, "goal", "punctuation around the word");
        assert!(lookup("", true).is_empty() && lookup("zzxq", true).is_empty() && lookup("   ", true).is_empty());
        assert!(lookup(&"a".repeat(10_000), true).is_empty());
    }

    /// French words are found from their plural, feminine, elided or conjugated forms.
    #[test]
    fn french_words_have_synonyms() {
        let m = lookup("maison", true);
        assert!(m.iter().any(|s| s.synonyms.iter().any(|x| x == "foyer")), "{m:?}");
        assert!(m.iter().all(|s| !s.pos.is_empty()));
        assert_eq!(lookup("projets", true)[0].word, "projet");
        assert_eq!(lookup("l'idée", true)[0].word, "idée");
        assert_eq!(lookup("chevaux", true)[0].word, "cheval");
        assert_eq!(lookup("présentons", true)[0].word, "présenter");
        assert_eq!(lookup("Maison", true)[0].synonyms[0].chars().next().map(char::is_uppercase), Some(true));
    }

    #[test]
    fn data_is_well_formed() {
        for l in DATA_FR.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            assert_eq!(l.splitn(3, '|').count(), 3, "{l}");
        }
        for l in DATA.lines().filter(|l| !l.starts_with('#') && !l.trim().is_empty()) {
            assert!(l.contains('|'), "{l}");
        }
    }
}
