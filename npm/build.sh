#!/bin/sh
# Builds the npm package into npm/pkg: the WebAssembly module, bound for Node (node/) and for browsers (web/).
# Needs the wasm32-unknown-unknown target and wasm-bindgen-cli of the same version as the wasm-bindgen crate.
set -eu
cd "$(dirname "$0")/.."

# cdylib only here: the crate itself stays a plain Rust library
cargo rustc --lib --release --target wasm32-unknown-unknown --features wasm --crate-type cdylib
wasm=target/wasm32-unknown-unknown/release/zhuyin_converter.wasm

rm -rf npm/pkg
wasm-bindgen --target nodejs --out-dir npm/pkg/node "$wasm"
wasm-bindgen --target web --out-dir npm/pkg/web "$wasm"
# The Node build is CommonJS, in a package that is otherwise ES modules
echo '{ "type": "commonjs" }' > npm/pkg/node/package.json
cp npm/web.js npm/pkg/web/index.js
cp npm/package.json README.md LICENSE npm/pkg/

# The version comes from Cargo.toml, like the Python package's
version=$(grep -m1 '^version' Cargo.toml | cut -d'"' -f2)
(cd npm/pkg && npm pkg set version="$version")
echo "npm/pkg: zhuyin-converter $version"
