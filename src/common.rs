//! What both directions of the conversion use.

use crate::tables::tables;

/// Tone number of the neutral tone.
pub(crate) const NEUTRAL: u8 = 5;

/// Pinyin vowels.
pub(crate) const VOWELS: &[char] = &['a', 'e', 'i', 'o', 'u', 'ü', 'ê'];

/// Punctuation a reading can have between its words.
pub(crate) const PUNCTUATION: &[char] = &[
    ',', '，', '。', '.', '!', '?', '！', '？', ';', '；', ':', '：', '…', '-', '–', '—',
];

/// Any bopomofo letter (even including the rarest ones from Taigi, Hakka...)
pub(crate) fn is_zhuyin_char(c: char) -> bool {
    ('\u{3105}'..='\u{312F}').contains(&c) || ('\u{31A0}'..='\u{31BF}').contains(&c)
}

/// True when the token is only punctuation.
pub(crate) fn is_punctuation(token: &str) -> bool {
    !token.is_empty() && token.chars().all(|c| PUNCTUATION.contains(&c))
}

/// True when the text has a bopomofo letter.
pub fn is_zhuyin(text: &str) -> bool {
    text.chars().any(is_zhuyin_char)
}

/// Pinyin syllables (without tone) that the conversions know, in table order.
pub fn pinyin_syllables() -> impl Iterator<Item = &'static str> {
    tables().syllables.iter().map(String::as_str)
}

/// Cuts a word of `n` letters into the fewest pieces accepted by `fits(start, length)`, none longer than `longest`.
/// Returns the length of each piece, or None when there is no way to cut the word.
pub(crate) fn split(n: usize, longest: usize, fits: impl Fn(usize, usize) -> bool) -> Option<Vec<usize>> {
    // best[i] is the best cut of the end of the word from letter i: (number of pieces, length of the first piece).
    let mut best: Vec<Option<(usize, usize)>> = vec![None; n + 1];
    best[n] = Some((0, 0));
    for i in (0..n).rev() {
        for length in (1..=longest.min(n - i)).rev() {
            if let Some((count, _)) = best[i + length] {
                if fits(i, length) {
                    let count = count + 1;
                    if best[i].map_or(true, |(c, _)| count < c) {
                        best[i] = Some((count, length));
                    }
                }
            }
        }
    }
    best[0]?;
    // Follow the best cut from the start of the word.
    let mut lengths = Vec::new();
    let mut i = 0;
    while i < n {
        let length = best[i].unwrap().1;
        lengths.push(length);
        i += length;
    }
    Some(lengths)
}
