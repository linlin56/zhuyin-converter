//! The Python binding (feature `python`): `import zhuyin_converter`.

use pyo3::prelude::*;

/// Zhuyin of a pinyin reading, with a space between syllables: "xíngdòng" or "xing2 dong4" gives "ㄒㄧㄥˊ ㄉㄨㄥˋ".
/// A reading already in zhuyin is returned as it is.
/// Returns "" when the text isn't pinyin.
#[pyfunction]
fn pinyin_to_zhuyin(pinyin: &str) -> String {
    crate::pinyin_to_zhuyin(pinyin)
}

/// Pinyin with tone marks of a zhuyin reading: "ㄒㄧㄥˊ ㄉㄨㄥˋ" gives "xíng dòng".
/// Returns "" when the text isn't zhuyin.
#[pyfunction]
fn zhuyin_to_pinyin(zhuyin: &str) -> String {
    crate::zhuyin_to_pinyin(zhuyin)
}

/// Pinyin with tone numbers of a zhuyin or pinyin reading, as CC-CEDICT writes it: "ㄈㄢˊㄊㄧˇㄗˋ" gives "fan2 ti3 zi4".
/// Returns "" when the text isn't a reading.
#[pyfunction]
fn numbered_pinyin(reading: &str) -> String {
    crate::numbered_pinyin(reading)
}

/// True when the text has a bopomofo letter.
#[pyfunction]
fn is_zhuyin(text: &str) -> bool {
    crate::is_zhuyin(text)
}

/// Pinyin syllables (without tone) that the conversions know.
#[pyfunction]
fn pinyin_syllables() -> Vec<&'static str> {
    crate::pinyin_syllables().collect()
}

#[pymodule]
#[pyo3(name = "zhuyin_converter")]
fn zhuyin_converter_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pinyin_to_zhuyin, m)?)?;
    m.add_function(wrap_pyfunction!(zhuyin_to_pinyin, m)?)?;
    m.add_function(wrap_pyfunction!(numbered_pinyin, m)?)?;
    m.add_function(wrap_pyfunction!(is_zhuyin, m)?)?;
    m.add_function(wrap_pyfunction!(pinyin_syllables, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
