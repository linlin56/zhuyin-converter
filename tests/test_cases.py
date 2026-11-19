# Runs the shared cases of tests/cases/*.tsv through the Python binding, like the Rust tests do.
# Also converts every syllable to zhuyin and back.
from pathlib import Path

import pytest

import zhuyin_converter as zc

CASES = Path(__file__).resolve().parent / "cases"


# Reads a cases file as pytest parameters (input, expected).
# Each line is input<TAB>expected<TAB>comment, and a line starting with # is a comment.
def cases(name):
    rows = []
    for n, line in enumerate((CASES / name).read_text(encoding="utf-8").split("\n"), start=1):
        if line and not line.startswith("#"):
            columns = line.split("\t")
            rows.append(pytest.param(columns[0], columns[1], id=f"{name}:{n}"))
    return rows


@pytest.mark.parametrize("pinyin, zhuyin", cases("pinyin_to_zhuyin.tsv"))
def test_pinyin_to_zhuyin(pinyin, zhuyin):
    assert zc.pinyin_to_zhuyin(pinyin) == zhuyin


@pytest.mark.parametrize("zhuyin, pinyin", cases("zhuyin_to_pinyin.tsv"))
def test_zhuyin_to_pinyin(zhuyin, pinyin):
    assert zc.zhuyin_to_pinyin(zhuyin) == pinyin


@pytest.mark.parametrize("reading, numbered", cases("numbered_pinyin.tsv"))
def test_numbered_pinyin(reading, numbered):
    assert zc.numbered_pinyin(reading) == numbered


@pytest.mark.parametrize("tone", [1, 2, 3, 4, 5])
def test_every_syllable_both_ways(tone):
    syllables = zc.pinyin_syllables()
    assert len(syllables) == 419
    for syllable in syllables:
        numbered = f"{syllable}{tone}"
        zhuyin = zc.pinyin_to_zhuyin(numbered)
        assert zhuyin, numbered
        assert zc.pinyin_to_zhuyin(zc.zhuyin_to_pinyin(zhuyin)) == zhuyin, numbered
        assert zc.numbered_pinyin(zhuyin) == numbered.replace("v", "ü"), numbered


def test_is_zhuyin():
    assert zc.is_zhuyin("ㄅㄚ") and not zc.is_zhuyin("ba")
