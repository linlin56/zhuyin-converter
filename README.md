# zhuyin-converter

Pinyin ↔ zhuyin (bopomofo, 注音符號) conversion, following Taiwan's conventions: the neutral tone's dot goes before
its syllable (˙ㄇㄚ), the first tone has no mark. Written in Rust, with no dependencies, and usable from Python and JavaScript.

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

## JavaScript

```bash
npm install zhuyin-converter
```

```js
import { pinyinToZhuyin, zhuyinToPinyin, numberedPinyin, isZhuyin } from "zhuyin-converter";

pinyinToZhuyin("shuo1 hua4"); // "ㄕㄨㄛ ㄏㄨㄚˋ"
```

The functions are those of Python, in camelCase, and come with their TypeScript types. The package is WebAssembly, so it
works the same on every platform: in Node (`import` or `require`) and in browsers through a bundler (Vite, webpack...),
which loads the module before the first call.

## Rust

```bash
cargo add zhuyin-converter
```

```rust
use zhuyin_converter::pinyin_to_zhuyin;

assert_eq!(pinyin_to_zhuyin("lǚyóu"), "ㄌㄩˇ ㄧㄡˊ");
```

## Repository

One crate, published to crates.io, to PyPI as a Python module built by maturin from `pyproject.toml`, and to npm as
a WebAssembly package built by `npm/build.sh`.

- `src/`: the conversion, the Python binding in `python.rs` (feature `python`) and the JavaScript one in `wasm.rs`
  (feature `wasm`), both off for Rust users
- `zhuyin_converter.pyi`: the types of the Python module, for editors
- `npm/`: the npm package's `package.json`, its browser entry (`web.js`) and `build.sh`, which builds it into `npm/pkg`
- `tests/`: the shared cases in `tests/cases/*.tsv` (`input<TAB>expected<TAB>comment`), read by the Rust tests
  (`cases.rs`), the Python ones (`test_cases.py`) and the JavaScript ones (`test_cases.mjs`)
- `.github/workflows/`: CI, and the release: push a tag `vX.Y.Z` to publish all three

The conversion comes from [MiningCat](https://github.com/linlin56/mining-cat)'s `zhuyin.py`, and gives the same
results on its tests and on a corpus of about 190,000 generated readings.

### Development

```bash
cargo test                              # Rust tests and the shared cases

python -m venv .venv && source .venv/bin/activate
pip install maturin pytest
maturin develop --release               # builds the binding into the venv
pytest tests                            # the same cases, through Python

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version <the wasm-bindgen version of Cargo.lock>
npm/build.sh                            # builds the npm package into npm/pkg
node --test tests/test_cases.mjs        # the same cases, through JavaScript
```

## License

Copyright (C) 2026 linlin56

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General
Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option)
any later version. See [LICENSE](LICENSE).
