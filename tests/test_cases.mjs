// Runs the shared cases of tests/cases/*.tsv through the npm package (npm/pkg, built by npm/build.sh),
// like the Rust and Python tests do. Also converts every syllable to zhuyin and back.
// Run with: node --test tests/test_cases.mjs
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { test } from "node:test";

const pkg = new URL("../npm/pkg/", import.meta.url);
const zc = await import(new URL("node/zhuyin_converter.js", pkg));

// Reads a cases file as [line number, input, expected].
// Each line is input<TAB>expected<TAB>comment, and a line starting with # is a comment.
function cases(name) {
    const text = readFileSync(new URL(`cases/${name}`, import.meta.url), "utf8");
    return text
        .split("\n")
        .map((line, i) => [i + 1, ...line.split("\t")])
        .filter(([, input]) => input && !input.startsWith("#"));
}

// Runs one conversion on every case of a file and lists all the failures.
function check(name, convert) {
    const rows = cases(name);
    assert.ok(rows.length > 0, `${name} has no cases`);
    const failures = rows
        .filter(([, input, expected]) => convert(input) !== expected)
        .map(([line, input, expected]) => `${name}:${line}: ${input} gave ${convert(input)}, expected ${expected}`);
    assert.deepEqual(failures, []);
}

test("pinyin to zhuyin", () => check("pinyin_to_zhuyin.tsv", zc.pinyinToZhuyin));
test("zhuyin to pinyin", () => check("zhuyin_to_pinyin.tsv", zc.zhuyinToPinyin));
test("numbered pinyin", () => check("numbered_pinyin.tsv", zc.numberedPinyin));

test("every syllable both ways", () => {
    const syllables = zc.pinyinSyllables();
    assert.equal(syllables.length, 419);
    for (const tone of [1, 2, 3, 4, 5]) {
        for (const syllable of syllables) {
            const numbered = `${syllable}${tone}`;
            const zhuyin = zc.pinyinToZhuyin(numbered);
            assert.ok(zhuyin, numbered);
            assert.equal(zc.pinyinToZhuyin(zc.zhuyinToPinyin(zhuyin)), zhuyin, numbered);
            assert.equal(zc.numberedPinyin(zhuyin), numbered.replaceAll("v", "ü"), numbered);
        }
    }
});

test("is zhuyin", () => {
    assert.ok(zc.isZhuyin("ㄅㄚ"));
    assert.ok(!zc.isZhuyin("ba"));
});

// The browser build, given the module's bytes since Node can't fetch a file URL
test("web build", async () => {
    const web = await import(new URL("web/zhuyin_converter.js", pkg));
    web.initSync({ module: readFileSync(new URL("web/zhuyin_converter_bg.wasm", pkg)) });
    assert.equal(web.pinyinToZhuyin("xing2dong4"), "ㄒㄧㄥˊ ㄉㄨㄥˋ");
    assert.equal(web.zhuyinToPinyin("ㄉㄚˋㄢ"), "dà'ān");
});
