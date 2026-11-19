//! Pinyin tone marks, taken off their letter and put back on it.
//! It does what Unicode NFD and NFC do, only for the letters of pinyin, so the crate needs no Unicode tables.

/// Combining marks of the four tones, and of ü and ê.
pub(crate) const MACRON: char = '\u{304}';
pub(crate) const ACUTE: char = '\u{301}';
pub(crate) const CARON: char = '\u{30C}';
pub(crate) const GRAVE: char = '\u{300}';
pub(crate) const DIAERESIS: char = '\u{308}';
pub(crate) const CIRCUMFLEX: char = '\u{302}';

/// Tone of a combining mark: macron 1, acute 2, caron 3, grave 4.
pub(crate) fn tone_of_mark(c: char) -> Option<u8> {
    match c {
        MACRON => Some(1),
        ACUTE => Some(2),
        CARON => Some(3),
        GRAVE => Some(4),
        _ => None,
    }
}

/// Combining mark of a tone, None for the neutral tone.
pub(crate) fn mark_of_tone(tone: u8) -> Option<char> {
    match tone {
        1 => Some(MACRON),
        2 => Some(ACUTE),
        3 => Some(CARON),
        4 => Some(GRAVE),
        _ => None,
    }
}

/// Vowels and their precomposed letters for tones 1 to 4.
const TONED: &[(char, [char; 4])] = &[
    ('a', ['ā', 'á', 'ǎ', 'à']),
    ('e', ['ē', 'é', 'ě', 'è']),
    ('i', ['ī', 'í', 'ǐ', 'ì']),
    ('o', ['ō', 'ó', 'ǒ', 'ò']),
    ('u', ['ū', 'ú', 'ǔ', 'ù']),
    ('ü', ['ǖ', 'ǘ', 'ǚ', 'ǜ']),
];
/// The other precomposed letters with a tone mark (Unicode has no ê̄, ê̌, m̄, m̀, m̌ or n̄).
const OTHER_TONED: &[(char, char, char)] = &[
    ('ê', ACUTE, 'ế'),
    ('ê', GRAVE, 'ề'),
    ('m', ACUTE, 'ḿ'),
    ('n', ACUTE, 'ń'),
    ('n', CARON, 'ň'),
    ('n', GRAVE, 'ǹ'),
];

/// Splits a lowercase character into its base letter and its combining marks, in the order NFD gives them.
pub(crate) fn decompose(c: char, out: &mut Vec<char>) {
    let push_base = |base: char, out: &mut Vec<char>| match base {
        'ü' => out.extend(['u', DIAERESIS]),
        'ê' => out.extend(['e', CIRCUMFLEX]),
        b => out.push(b),
    };
    for (base, toned) in TONED {
        if let Some(i) = toned.iter().position(|&t| t == c) {
            push_base(*base, out);
            out.push(mark_of_tone(i as u8 + 1).unwrap());
            return;
        }
    }
    for (base, mark, toned) in OTHER_TONED {
        if *toned == c {
            push_base(*base, out);
            out.push(*mark);
            return;
        }
    }
    match c {
        'ü' | 'ê' => push_base(c, out),
        _ => out.push(c),
    }
}

/// Writes `base` with the tone mark `mark` on it, as one character when Unicode has it, else as the letter followed by the mark.
pub(crate) fn compose(base: char, mark: char, out: &mut String) {
    if let Some(tone) = tone_of_mark(mark) {
        if let Some((_, toned)) = TONED.iter().find(|(b, _)| *b == base) {
            out.push(toned[tone as usize - 1]);
            return;
        }
    }
    if let Some((_, _, toned)) = OTHER_TONED.iter().find(|(b, m, _)| *b == base && *m == mark) {
        out.push(*toned);
        return;
    }
    out.push(base);
    out.push(mark);
}
