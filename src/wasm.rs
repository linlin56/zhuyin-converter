//! The JavaScript binding (feature `wasm`), compiled to WebAssembly: the npm package that npm/build.sh makes.

use wasm_bindgen::prelude::*;

/// Zhuyin of a pinyin reading, with a space between syllables: "xíngdòng" or "xing2 dong4" gives "ㄒㄧㄥˊ ㄉㄨㄥˋ".
/// A reading already in zhuyin is returned as it is.
/// Returns "" when the text isn't pinyin.
#[wasm_bindgen(js_name = pinyinToZhuyin)]
pub fn pinyin_to_zhuyin(pinyin: &str) -> String {
    crate::pinyin_to_zhuyin(pinyin)
}

/// Pinyin with tone marks of a zhuyin reading: "ㄒㄧㄥˊ ㄉㄨㄥˋ" gives "xíng dòng".
/// Returns "" when the text isn't zhuyin.
#[wasm_bindgen(js_name = zhuyinToPinyin)]
pub fn zhuyin_to_pinyin(zhuyin: &str) -> String {
    crate::zhuyin_to_pinyin(zhuyin)
}

/// Pinyin with tone numbers of a zhuyin or pinyin reading, as CC-CEDICT writes it: "ㄈㄢˊㄊㄧˇㄗˋ" gives "fan2 ti3 zi4".
/// Returns "" when the text isn't a reading.
#[wasm_bindgen(js_name = numberedPinyin)]
pub fn numbered_pinyin(reading: &str) -> String {
    crate::numbered_pinyin(reading)
}

/// True when the text has a bopomofo letter.
#[wasm_bindgen(js_name = isZhuyin)]
pub fn is_zhuyin(text: &str) -> bool {
    crate::is_zhuyin(text)
}

/// Pinyin syllables (without tone) that the conversions know.
#[wasm_bindgen(js_name = pinyinSyllables)]
pub fn pinyin_syllables() -> Vec<String> {
    crate::pinyin_syllables().map(String::from).collect()
}
