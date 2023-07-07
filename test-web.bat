cargo build --release --target wasm32-unknown-unknown --example test
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/nebulousengine.wasm
cd out
serve -s .