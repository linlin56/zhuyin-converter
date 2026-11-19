# zhuyin-converter

Pinyin ↔ zhuyin (bopomofo, 注音符號) conversion, following Taiwan's conventions: the neutral tone's dot goes before
its syllable (˙ㄇㄚ), the first tone has no mark. Written in Rust, with no dependencies, and usable from Python.

| | |
|---|---|
| `pinyin_to_zhuyin("wǒmen")` | `ㄨㄛˇ ˙ㄇㄣ` |
| `pinyin_to_zhuyin("xing2dong4")` | `ㄒㄧㄥˊ ㄉㄨㄥˋ` |
| `pinyin_to_zhuyin("yīxiàr5")` | `ㄧ ㄒㄧㄚˋㄦ` (erhua) |
| `zhuyin_to_pinyin("ㄉㄚˋㄢ")` | `dà'ān` |
| `zhuyin_to_pinyin("ㄖㄣˋㄕ˙ ㄇㄚ˙")` | `rènshi ma` (dots after their syllables too) |
| `numbered_pinyin("ㄈㄢˊㄊㄧˇㄗˋ")` | `fan2 ti3 zi4` |

Every function returns an empty string when its input isn't a reading it can convert.

It reads tone marks, tone numbers (`xing2dong4`), toneless syllables (neutral tone), `u:` for ü and `r5` for erhua,
apostrophes and their absence (`Xīān`, `fang'an`), erhua (`huār`, `hua1r`, `ㄓㄜˋㄦ`), and the nasal interjections
(`hm`, `ng`).

## Python

```bash
pip install zhuyin-converter
```

```python
from zhuyin_converter import pinyin_to_zhuyin, zhuyin_to_pinyin, numbered_pinyin, is_zhuyin

pinyin_to_zhuyin("shuo1 hua4")  # 'ㄕㄨㄛ ㄏㄨㄚˋ'
```

## Rust

```bash
cargo add zhuyin-converter
```

```rust
use zhuyin_converter::pinyin_to_zhuyin;

assert_eq!(pinyin_to_zhuyin("lǚyóu"), "ㄌㄩˇ ㄧㄡˊ");
```

## Repository

One crate, published to crates.io, and to PyPI as a Python module built by maturin from `pyproject.toml`.

- `src/`: the conversion, and the Python binding in `python.rs` (feature `python`, off for Rust users)
- `zhuyin_converter.pyi`: the types of the Python module, for editors
- `tests/`: the shared cases in `tests/cases/*.tsv` (`input<TAB>expected<TAB>comment`), read by the Rust tests
  (`cases.rs`) and the Python ones (`test_cases.py`)
- `.github/workflows/`: CI, and the release: push a tag `vX.Y.Z` to publish both

The conversion comes from [MiningCat](https://github.com/linlin56/mining-cat)'s `zhuyin.py`, and gives the same
results on its tests and on a corpus of about 190,000 generated readings.

### Development

```bash
cargo test                              # Rust tests and the shared cases

python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --release               # builds the binding into the venv
pytest tests                            # the same cases, through Python
```

## License

Copyright (C) 2026 linlin56

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General
Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option)
any later version. See [LICENSE](LICENSE).
