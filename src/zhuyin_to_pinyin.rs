//! Zhuyin to pinyin.
//! Each zhuyin word is cut into syllables, which are looked up in the tables and given their tone mark.

use crate::common::{is_punctuation, is_zhuyin, is_zhuyin_char, split, NEUTRAL, VOWELS};
use crate::marks::{compose, mark_of_tone};
use crate::pinyin_to_zhuyin::pinyin_to_zhuyin;
use crate::tables::tables;

/// Zhuyin tone marks and their tone, the neutral tone's dot ˙ being read apart.
const ZHUYIN_TONES: &[(char, u8)] = &[('ˉ', 1), ('ˊ', 2), ('ˇ', 3), ('ˋ', 4)];

/// Tone of a zhuyin tone mark, None for any other character.
fn zhuyin_tone(c: char) -> Option<u8> {
    ZHUYIN_TONES.iter().find(|(m, _)| *m == c).map(|(_, t)| *t)
}

/// Characters that separate zhuyin words: spaces and middle dots.
fn is_zhuyin_separator(c: char) -> bool {
    c.is_whitespace() || matches!(c, '・' | '·')
}

/// Puts the tone mark on a pinyin syllable: on a or e, on the o of ou, else on the last vowel (liú, guǐ).
/// A syllable with no vowel (m, ng, hm) takes it on its nasal.
fn mark(syllable: &str, tone: u8) -> String {
    let Some(tone_mark) = mark_of_tone(tone) else {
        return syllable.to_string();
    };
    let chars: Vec<char> = syllable.chars().collect();
    let position = |c: char| chars.iter().position(|&x| x == c);
    let at = if !chars.iter().any(|c| VOWELS.contains(c)) {
        position('m').or_else(|| position('n')).unwrap_or(0)
    } else if let Some(at) = position('a').or_else(|| position('e')) {
        at
    } else if syllable.contains("ou") {
        position('o').unwrap()
    } else {
        chars.iter().rposition(|c| VOWELS.contains(c)).unwrap()
    };
    let mut out = String::new();
    for (i, &c) in chars.iter().enumerate() {
        if i == at {
            compose(c, tone_mark, &mut out);
        } else {
            out.push(c);
        }
    }
    out
}

/// A run of zhuyin letters between two tone marks, which can hold several syllables.
/// `first` is the tone of its first syllable (a ˙ before it), and `last` the tone of its last one (a mark after it).
struct Piece {
    letters: Vec<char>,
    first: u8,
    last: u8,
}

/// Cuts a zhuyin word (no spaces) into pinyin syllables, each with its tone and whether it has an erhua.
fn zhuyin_word(word: &str) -> Option<Vec<(String, u8, bool)>> {
    // First the word is cut into pieces on its tone marks.
    let chars: Vec<char> = word.chars().collect();
    let mut pieces: Vec<Piece> = Vec::new();
    // The piece being read, None when the next letter starts a new one.
    let mut current: Option<usize> = None;
    for (i, &c) in chars.iter().enumerate() {
        if c == '˙' {
            if chars.get(i + 1).is_some_and(|&n| is_zhuyin_char(n)) {
                // A dot before a letter is Taiwan's way to write the neutral tone, and it starts a new piece.
                pieces.push(Piece {
                    letters: Vec::new(),
                    first: NEUTRAL,
                    last: 0,
                });
                current = Some(pieces.len() - 1);
            } else {
                // Otherwise the dot ends the word and is the tone of the syllable before it (ㄕ˙).
                let k = current.filter(|&k| pieces[k].last == 0)?;
                pieces[k].last = NEUTRAL;
                current = None;
            }
        } else if let Some(tone) = zhuyin_tone(c) {
            // A tone mark ends the piece and gives the tone of its last syllable.
            match current {
                Some(k) if !pieces[k].letters.is_empty() => pieces[k].last = tone,
                _ => return None,
            }
            current = None;
        } else if is_zhuyin_char(c) {
            // A letter goes into the piece being read, or starts a new one.
            let k = match current {
                Some(k) => k,
                None => {
                    pieces.push(Piece {
                        letters: Vec::new(),
                        first: 0,
                        last: 0,
                    });
                    pieces.len() - 1
                }
            };
            current = Some(k);
            pieces[k].letters.push(c);
        } else {
            return None;
        }
    }

    // Then each piece is cut into syllables.
    let t = tables();
    let mut syllables: Vec<(String, u8, bool)> = Vec::new();
    for Piece { letters, first, last } in pieces {
        // A bare ㄦ after a syllable is its erhua (ㄓㄜˋㄦ), while a toned one (ㄦˊ) is the syllable 兒 itself.
        if letters == ['ㄦ'] && first == 0 && last == 0 && syllables.last().is_some_and(|s| !s.2) {
            syllables.last_mut().unwrap().2 = true;
            continue;
        }
        let piece = |i: usize, length: usize| -> String { letters[i..i + length].iter().collect() };
        let fits = |i: usize, length: usize| -> bool {
            let p = piece(i, length);
            if t.pinyin_of.contains_key(&p) {
                return true;
            }
            // An erhua is a ㄦ right after a syllable, with no tone mark on it (ㄓㄜˋㄦ).
            // A toned ㄦ is the syllable 兒 or 爾 itself, so ㄅㄚㄦˇ is bā'ěr.
            let toned = last != 0 && i + length == letters.len();
            length > 1 && p.ends_with('ㄦ') && t.pinyin_of.contains_key(&piece(i, length - 1)) && !toned
        };
        let lengths = split(letters.len(), t.longest_zhuyin + 1, fits)?;
        let mut i = 0;
        for (k, &length) in lengths.iter().enumerate() {
            let p = piece(i, length);
            let erhua = !t.pinyin_of.contains_key(&p);
            let key = if erhua { piece(i, length - 1) } else { p };
            // The first syllable takes the dot before the piece, the last one the mark after it, and the others are first tone.
            let mut tone = 0;
            if k == 0 && first != 0 {
                tone = first;
            }
            if tone == 0 && k == lengths.len() - 1 && last != 0 {
                tone = last;
            }
            if tone == 0 {
                tone = 1;
            }
            syllables.push((t.pinyin_of[&key].clone(), tone, erhua));
            i += length;
        }
    }
    Some(syllables)
}

/// Pinyin with tone numbers of a zhuyin or pinyin reading, as CC-CEDICT writes it: "ㄈㄢˊㄊㄧˇㄗˋ" or "fántǐzì" gives "fan2 ti3 zi4".
/// The neutral tone is 5, and an erhua stays on its syllable (huar1).
/// Punctuation is dropped, and it returns "" when the text isn't a reading.
pub fn numbered_pinyin(reading: &str) -> String {
    // Pinyin goes through zhuyin first, so that both are read the same way.
    let zhuyin = pinyin_to_zhuyin(reading);
    let mut syllables = Vec::new();
    for token in zhuyin.split(is_zhuyin_separator) {
        if token.is_empty() || is_punctuation(token) {
            continue;
        }
        let Some(parsed) = zhuyin_word(token) else {
            return String::new();
        };
        for (syllable, tone, erhua) in parsed {
            syllables.push(format!("{syllable}{}{tone}", if erhua { "r" } else { "" }));
        }
    }
    syllables.join(" ")
}

/// Pinyin with tone marks of a zhuyin reading: "ㄒㄧㄥˊ ㄉㄨㄥˋ" gives "xíng dòng", and "ㄌㄠˇㄖㄣˊ" gives "lǎorén".
/// Returns "" when the text isn't zhuyin.
pub fn zhuyin_to_pinyin(zhuyin: &str) -> String {
    let text = zhuyin.trim();
    if !is_zhuyin(text) {
        return String::new();
    }
    let mut words = Vec::new();
    for token in text.split(is_zhuyin_separator) {
        if token.is_empty() {
            continue;
        }
        if is_punctuation(token) {
            words.push(token.to_string());
            continue;
        }
        let Some(syllables) = zhuyin_word(token) else {
            return String::new();
        };
        let mut word = String::new();
        for (syllable, tone, erhua) in syllables {
            // Inside a word, a syllable starting with a, o or e needs an apostrophe before it (dà'ān).
            if !word.is_empty() && syllable.starts_with(['a', 'o', 'e']) {
                word.push('\'');
            }
            word.push_str(&mark(&syllable, tone));
            if erhua {
                word.push('r');
            }
        }
        words.push(word);
    }
    words.join(" ")
}
