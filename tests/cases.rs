//! Runs the shared cases of tests/cases/*.tsv, which the Python tests read too.
//! Also converts every syllable to zhuyin and back.

use zhuyin_converter::{numbered_pinyin, pinyin_syllables, pinyin_to_zhuyin, zhuyin_to_pinyin};

/// Reads a cases file as (line number, input, expected).
/// Each line is input<TAB>expected<TAB>comment, and a line starting with # is a comment.
fn cases(text: &str) -> Vec<(usize, &str, &str)> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty() && !line.starts_with('#'))
        .map(|(n, line)| {
            let mut columns = line.split('\t');
            let input = columns.next().unwrap();
            let expected = columns
                .next()
                .unwrap_or_else(|| panic!("line {}: no expected column", n + 1));
            (n + 1, input, expected)
        })
        .collect()
}

/// Runs one conversion on every case of a file and lists all the failures.
fn check(file: &str, text: &str, convert: fn(&str) -> String) {
    let cases = cases(text);
    assert!(!cases.is_empty(), "{file} has no cases");
    let failures: Vec<String> = cases
        .iter()
        .filter_map(|&(line, input, expected)| {
            let got = convert(input);
            (got != expected).then(|| format!("{file}:{line}: {input:?} -> {got:?}, expected {expected:?}"))
        })
        .collect();
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn pinyin_to_zhuyin_cases() {
    check(
        "pinyin_to_zhuyin.tsv",
        include_str!("cases/pinyin_to_zhuyin.tsv"),
        pinyin_to_zhuyin,
    );
}

#[test]
fn zhuyin_to_pinyin_cases() {
    check(
        "zhuyin_to_pinyin.tsv",
        include_str!("cases/zhuyin_to_pinyin.tsv"),
        zhuyin_to_pinyin,
    );
}

#[test]
fn numbered_pinyin_cases() {
    check(
        "numbered_pinyin.tsv",
        include_str!("cases/numbered_pinyin.tsv"),
        numbered_pinyin,
    );
}

#[test]
fn every_syllable_both_ways() {
    let mut count = 0;
    for tone in 1..=5 {
        for syllable in pinyin_syllables() {
            let numbered = format!("{syllable}{tone}");
            let zhuyin = pinyin_to_zhuyin(&numbered);
            assert!(!zhuyin.is_empty(), "{numbered}");
            assert_eq!(pinyin_to_zhuyin(&zhuyin_to_pinyin(&zhuyin)), zhuyin, "{numbered}");
            assert_eq!(numbered_pinyin(&zhuyin), numbered.replace('v', "ü"), "{numbered}");
            count += 1;
        }
    }
    assert_eq!(count, 419 * 5);
}
