//! Pinyin to zhuyin.
//! The pinyin is cut into syllables, and each syllable is looked up in the tables.

use crate::common::{is_punctuation, is_zhuyin, split, NEUTRAL, VOWELS};
use crate::marks::{decompose, tone_of_mark, CIRCUMFLEX, DIAERESIS};
use crate::tables::tables;

/// Only ASCII digits are tone numbers, other digits (５, ٥) make a token that isn't pinyin anyway.
fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// A word character: a letter, a digit, or an underscore.
/// Other punctuation (including apostrophes) separates pinyin syllables.
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// Characters that separate pinyin words or syllables: spaces, apostrophes, middle dots and hyphens.
fn is_pinyin_separator(c: char) -> bool {
    c.is_whitespace() || matches!(c, '\'' | '’' | '·' | '・' | '-')
}

/// Zhuyin mark written after a syllable of this tone, nothing for the first tone.
fn zhuyin_mark(tone: u8) -> &'static str {
    match tone {
        2 => "ˊ",
        3 => "ˇ",
        4 => "ˋ",
        _ => "",
    }
}

/// Rewrites the spellings some common dictionaries use:
// - u for ü (lu:4)
// - r5 for an erhua in pinyin with tone marks (yīxiàr5),
// - the script ɡ (U+0261) of for g
fn dictionary_spellings(text: &str) -> String {
    let text = text.replace("u:", "ü").replace("U:", "Ü").replace('\u{261}', "g");
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        // r5 is an erhua only right after a letter and at the end of a word.
        if chars[i] == 'r'
            && chars.get(i + 1) == Some(&'5')
            && i > 0
            && !chars[i - 1].is_whitespace()
            && !is_digit(chars[i - 1])
            && chars.get(i + 2).map_or(true, |&c| !is_word_char(c))
        {
            out.push('r');
            i += 2;
            continue;
        }
        out.push(chars[i]);
        i += 1;
    }
    out
}

/// The letters of a pinyin word, each with the tone of the mark on it (0 when it has none).
/// ü can be written with a diaeresis or as v, and ê with a circumflex.
/// Returns None when the word has a character that isn't a pinyin letter.
fn letters(word: &str) -> Option<Vec<(char, u8)>> {
    let mut decomposed = Vec::new();
    for c in word.chars().flat_map(char::to_lowercase) {
        decompose(c, &mut decomposed);
    }
    let mut out: Vec<(char, u8)> = Vec::new();
    for c in decomposed {
        if let (Some(tone), Some(last)) = (tone_of_mark(c), out.last_mut()) {
            last.1 = tone;
        } else if c == DIAERESIS && out.last().is_some_and(|l| l.0 == 'u') {
            out.last_mut().unwrap().0 = 'ü';
        } else if c == CIRCUMFLEX && out.last().is_some_and(|l| l.0 == 'e') {
            out.last_mut().unwrap().0 = 'ê';
        } else if c.is_ascii_lowercase() {
            out.push((if c == 'v' { 'ü' } else { c }, 0));
        } else {
            return None;
        }
    }
    Some(out)
}

/// Cuts a run of pinyin letters into syllables, each with its tone and whether it ends with an erhua r.
/// `tone` is the tone number written after the run, and it goes to the last syllable.
/// A syllable with no tone mark or number has the neutral tone.
fn pinyin_syllables_of(word: &str, tone: u8) -> Option<Vec<(String, u8, bool)>> {
    let letters = letters(word)?;
    if letters.is_empty() {
        return None;
    }
    let t = tables();
    let text: Vec<char> = letters.iter().map(|l| l.0).collect();
    let piece = |i: usize, length: usize| -> String { text[i..i + length].iter().collect() };

    // A piece fits when it is a known syllable, alone or followed by an erhua r.
    // With `strict`, a syllable can't start with a, o or e inside the word, because it would need an apostrophe.
    let fits = |strict: bool| {
        let text = &text;
        let letters = &letters;
        move |i: usize, length: usize| -> bool {
            // A syllable has at most one tone mark, so xīān is two syllables.
            if letters[i..i + length].iter().filter(|l| l.1 != 0).count() > 1 {
                return false;
            }
            if strict && i > 0 && matches!(text[i], 'a' | 'o' | 'e') {
                return false;
            }
            let p = piece(i, length);
            if t.zhuyin_of.contains_key(&p) {
                return true;
            }
            // An erhua r has no vowel after it, otherwise it is the initial of the next syllable.
            let stem: String = text[i..i + length - 1].iter().collect();
            p.ends_with('r')
                && t.zhuyin_of.contains_key(&stem)
                && p != "er"
                && (i + length == text.len() || !VOWELS.contains(&text[i + length]))
        }
    };

    // An erhua r makes the longest piece one letter longer.
    let longest = t.longest_pinyin + 1;
    // The strict cut is tried first (fangan is fan gan), and the other one only when it fails (Xīān is Xī ān).
    let lengths = split(text.len(), longest, fits(true)).or_else(|| split(text.len(), longest, fits(false)))?;
    let mut syllables = Vec::new();
    let mut i = 0;
    for (k, &length) in lengths.iter().enumerate() {
        let p = piece(i, length);
        let marked = letters[i..i + length]
            .iter()
            .map(|l| l.1)
            .find(|&m| m != 0)
            .unwrap_or(0);
        let erhua = !t.zhuyin_of.contains_key(&p);
        let syllable = if erhua {
            text[i..i + length - 1].iter().collect()
        } else {
            p
        };
        let last = k == lengths.len() - 1;
        let tone = if marked != 0 {
            marked
        } else if last && tone != 0 {
            tone
        } else {
            NEUTRAL
        };
        syllables.push((syllable, tone, erhua));
        i += length;
    }
    Some(syllables)
}

/// Zhuyin of one syllable, with the neutral tone's dot before it and the other tone marks after it.
/// An erhua adds ㄦ at the end.
fn zhuyin_syllable(syllable: &str, tone: u8, erhua: bool) -> String {
    let zhuyin = &tables().zhuyin_of[syllable];
    let mut out = if tone == NEUTRAL {
        format!("˙{zhuyin}")
    } else {
        format!("{zhuyin}{}", zhuyin_mark(tone))
    };
    if erhua {
        out.push('ㄦ');
    }
    out
}

/// Zhuyin of a pinyin reading, with a space between syllables: "xíngdòng" or "xing2 dong4" gives "ㄒㄧㄥˊ ㄉㄨㄥˋ".
/// A reading already in zhuyin is returned as it is.
/// Returns "" when the text isn't pinyin.
pub fn pinyin_to_zhuyin(pinyin: &str) -> String {
    let text = pinyin.trim();
    if text.is_empty() {
        return String::new();
    }
    if is_zhuyin(text) {
        return text.to_string();
    }
    let text = dictionary_spellings(text);
    let mut out: Vec<String> = Vec::new();
    for token in text.split(is_pinyin_separator) {
        if token.is_empty() {
            continue;
        }
        if is_punctuation(token) {
            out.push(token.to_string());
            continue;
        }
        // A tone number ends a syllable, so xing2dong4 is xing with tone 2 then dong with tone 4.
        let chars: Vec<char> = token.chars().collect();
        let mut parts: Vec<(String, u8)> = Vec::new();
        let mut i = 0;
        while i < chars.len() {
            let start = i;
            while i < chars.len() && !is_digit(chars[i]) {
                i += 1;
            }
            // A digit with no letters before it isn't pinyin.
            if i == start {
                return String::new();
            }
            let word: String = chars[start..i].iter().collect();
            let mut number = 0;
            if i < chars.len() && ('1'..='5').contains(&chars[i]) {
                number = chars[i] as u8 - b'0';
                i += 1;
            }
            parts.push((word, number));
        }
        for (word, number) in parts {
            // An r alone after a tone number is the erhua of the syllable before it (hua1r).
            if word.to_lowercase() == "r" && out.last().is_some_and(|last| !is_punctuation(last)) {
                out.last_mut().unwrap().push('ㄦ');
                continue;
            }
            match pinyin_syllables_of(&word, number) {
                Some(syllables) => out.extend(
                    syllables
                        .iter()
                        .map(|(s, tone, erhua)| zhuyin_syllable(s, *tone, *erhua)),
                ),
                None => return String::new(),
            }
        }
    }
    out.join(" ")
}
