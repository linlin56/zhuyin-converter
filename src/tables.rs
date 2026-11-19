//! Pinyin syllables (without tone) and their zhuyin.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Pinyin initials and their zhuyin.
const INITIALS: &[(&str, &str)] = &[
    ("b", "ㄅ"),
    ("p", "ㄆ"),
    ("m", "ㄇ"),
    ("f", "ㄈ"),
    ("d", "ㄉ"),
    ("t", "ㄊ"),
    ("n", "ㄋ"),
    ("l", "ㄌ"),
    ("g", "ㄍ"),
    ("k", "ㄎ"),
    ("h", "ㄏ"),
    ("j", "ㄐ"),
    ("q", "ㄑ"),
    ("x", "ㄒ"),
    ("zh", "ㄓ"),
    ("ch", "ㄔ"),
    ("sh", "ㄕ"),
    ("r", "ㄖ"),
    ("z", "ㄗ"),
    ("c", "ㄘ"),
    ("s", "ㄙ"),
];

/// Finals after an initial.
/// j, q and x write ü as u, so their u, ue, uan and un are the ü ones.
const FINALS: &[(&str, &str)] = &[
    ("a", "ㄚ"),
    ("o", "ㄛ"),
    ("e", "ㄜ"),
    ("ai", "ㄞ"),
    ("ei", "ㄟ"),
    ("ao", "ㄠ"),
    ("ou", "ㄡ"),
    ("an", "ㄢ"),
    ("en", "ㄣ"),
    ("ang", "ㄤ"),
    ("eng", "ㄥ"),
    ("ong", "ㄨㄥ"),
    ("i", "ㄧ"),
    ("ia", "ㄧㄚ"),
    ("ie", "ㄧㄝ"),
    ("iao", "ㄧㄠ"),
    ("iu", "ㄧㄡ"),
    ("ian", "ㄧㄢ"),
    ("in", "ㄧㄣ"),
    ("iang", "ㄧㄤ"),
    ("ing", "ㄧㄥ"),
    ("iong", "ㄩㄥ"),
    ("u", "ㄨ"),
    ("ua", "ㄨㄚ"),
    ("uo", "ㄨㄛ"),
    ("uai", "ㄨㄞ"),
    ("ui", "ㄨㄟ"),
    ("uan", "ㄨㄢ"),
    ("un", "ㄨㄣ"),
    ("uang", "ㄨㄤ"),
    ("ü", "ㄩ"),
    ("üe", "ㄩㄝ"),
];
const JQX_FINALS: &[(&str, &str)] = &[("u", "ㄩ"), ("ue", "ㄩㄝ"), ("uan", "ㄩㄢ"), ("un", "ㄩㄣ")];

/// The finals each initial takes.
const COMBINATIONS: &[(&str, &str)] = &[
    ("b", "a o ai ei ao an en ang eng i ie iao ian in ing u"),
    ("p", "a o ai ei ao ou an en ang eng i ie iao ian in ing u"),
    ("m", "a o e ai ei ao ou an en ang eng i ie iao iu ian in ing u"),
    ("f", "a o ei ou an en ang eng u"),
    (
        "d",
        "a e ai ei ao ou an en ang eng ong i ia ie iao iu ian ing u uo ui uan un",
    ),
    ("t", "a e ai ei ao ou an ang eng ong i ie iao ian ing u uo ui uan un"),
    (
        "n",
        "a e ai ei ao ou an en ang eng ong i ie iao iu ian in iang ing u uo uan un ü üe",
    ),
    (
        "l",
        "a o e ai ei ao ou an ang eng ong i ia ie iao iu ian in iang ing u uo uan un ü üe",
    ),
    ("g", "a e ai ei ao ou an en ang eng ong u ua uo uai ui uan un uang"),
    ("k", "a e ai ei ao ou an en ang eng ong u ua uo uai ui uan un uang"),
    ("h", "a e ai ei ao ou an en ang eng ong u ua uo uai ui uan un uang"),
    ("j", "i ia ie iao iu ian in iang ing iong u ue uan un"),
    ("q", "i ia ie iao iu ian in iang ing iong u ue uan un"),
    ("x", "i ia ie iao iu ian in iang ing iong u ue uan un"),
    ("zh", "a e ai ei ao ou an en ang eng ong i u ua uo uai ui uan un uang"),
    ("ch", "a e ai ao ou an en ang eng ong i u ua uo uai ui uan un uang"),
    ("sh", "a e ai ei ao ou an en ang eng i u ua uo uai ui uan un uang"),
    ("r", "e ao ou an en ang eng ong i u ua uo ui uan un"),
    ("z", "a e ai ei ao ou an en ang eng ong i u uo ui uan un"),
    ("c", "a e ai ao ou an en ang eng ong i u uo ui uan un"),
    ("s", "a e ai ao ou an en ang eng ong i u uo ui uan un"),
];

/// Syllables without an initial (y and w spell the medials), and the nasal interjections (呣 m, 嗯 ng, 哼 hng).
const STANDALONE: &[(&str, &str)] = &[
    ("a", "ㄚ"),
    ("o", "ㄛ"),
    ("e", "ㄜ"),
    ("ê", "ㄝ"),
    ("ai", "ㄞ"),
    ("ei", "ㄟ"),
    ("ao", "ㄠ"),
    ("ou", "ㄡ"),
    ("an", "ㄢ"),
    ("en", "ㄣ"),
    ("ang", "ㄤ"),
    ("eng", "ㄥ"),
    ("er", "ㄦ"),
    ("yi", "ㄧ"),
    ("ya", "ㄧㄚ"),
    ("yo", "ㄧㄛ"),
    ("ye", "ㄧㄝ"),
    ("yai", "ㄧㄞ"),
    ("yao", "ㄧㄠ"),
    ("you", "ㄧㄡ"),
    ("yan", "ㄧㄢ"),
    ("yin", "ㄧㄣ"),
    ("yang", "ㄧㄤ"),
    ("ying", "ㄧㄥ"),
    ("yong", "ㄩㄥ"),
    ("wu", "ㄨ"),
    ("wa", "ㄨㄚ"),
    ("wo", "ㄨㄛ"),
    ("wai", "ㄨㄞ"),
    ("wei", "ㄨㄟ"),
    ("wan", "ㄨㄢ"),
    ("wen", "ㄨㄣ"),
    ("wang", "ㄨㄤ"),
    ("weng", "ㄨㄥ"),
    ("yu", "ㄩ"),
    ("yue", "ㄩㄝ"),
    ("yuan", "ㄩㄢ"),
    ("yun", "ㄩㄣ"),
    ("m", "ㄇ"),
    ("n", "ㄋ"),
    ("ng", "ㄫ"),
    ("hm", "ㄏㄇ"),
    ("hng", "ㄏㄫ"),
];

/// zhi, chi, shi, ri, zi, ci, si: the initial alone.
const SYLLABIC: &[&str] = &["zh", "ch", "sh", "r", "z", "c", "s"];

/// Value of a key in one of the tables above.
fn lookup<'a>(table: &[(&str, &'a str)], key: &str) -> Option<&'a str> {
    table.iter().find(|(k, _)| *k == key).map(|(_, v)| *v)
}

/// The syllable tables, built once from the lists above.
pub(crate) struct Tables {
    /// Zhuyin of each pinyin syllable (without tone).
    pub zhuyin_of: HashMap<String, String>,
    /// Pinyin of each zhuyin syllable.
    pub pinyin_of: HashMap<String, String>,
    /// Length of the longest pinyin syllable, in characters.
    pub longest_pinyin: usize,
    /// Length of the longest zhuyin syllable, in characters.
    pub longest_zhuyin: usize,
    /// Pinyin syllables in table order, for the callers that list them.
    pub syllables: Vec<String>,
}

/// The tables, built on first use.
pub(crate) fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(|| {
        let mut entries: Vec<(String, String)> =
            STANDALONE.iter().map(|(p, z)| (p.to_string(), z.to_string())).collect();
        for (initial, finals) in COMBINATIONS {
            let initial_zhuyin = lookup(INITIALS, initial).expect("initial");
            for fin in finals.split(' ') {
                let zhuyin = if fin == "i" && SYLLABIC.contains(initial) {
                    initial_zhuyin.to_string()
                } else if matches!(*initial, "j" | "q" | "x") && lookup(JQX_FINALS, fin).is_some() {
                    format!("{initial_zhuyin}{}", lookup(JQX_FINALS, fin).unwrap())
                } else {
                    format!("{initial_zhuyin}{}", lookup(FINALS, fin).expect("final"))
                };
                entries.push((format!("{initial}{fin}"), zhuyin));
            }
        }
        let zhuyin_of: HashMap<String, String> = entries.iter().cloned().collect();
        let pinyin_of: HashMap<String, String> = entries.iter().map(|(p, z)| (z.clone(), p.clone())).collect();
        let longest_pinyin = zhuyin_of.keys().map(|k| k.chars().count()).max().unwrap_or(0);
        let longest_zhuyin = pinyin_of.keys().map(|k| k.chars().count()).max().unwrap_or(0);
        Tables {
            zhuyin_of,
            pinyin_of,
            longest_pinyin,
            longest_zhuyin,
            syllables: entries.into_iter().map(|e| e.0).collect(),
        }
    })
}
