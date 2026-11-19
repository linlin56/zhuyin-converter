//! Pinyin / zhuyin conversion
//! We use Taiwanese conventions: the neutral tone's dot goes before its syllable (˙ㄇㄚ), the first tone has no mark.

mod common;
mod marks;
mod pinyin_to_zhuyin;
#[cfg(feature = "python")]
mod python;
mod tables;
mod zhuyin_to_pinyin;

pub use common::{is_zhuyin, pinyin_syllables};
pub use pinyin_to_zhuyin::pinyin_to_zhuyin;
pub use zhuyin_to_pinyin::{numbered_pinyin, zhuyin_to_pinyin};
