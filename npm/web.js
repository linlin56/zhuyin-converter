// The entry of browsers and bundlers: loads the WebAssembly module once, then gives the same functions as in Node.
import init, { isZhuyin, numberedPinyin, pinyinSyllables, pinyinToZhuyin, zhuyinToPinyin } from "./zhuyin_converter.js";

await init();

export { isZhuyin, numberedPinyin, pinyinSyllables, pinyinToZhuyin, zhuyinToPinyin };
