cargo build --release --target wasm32-unknown-unknown --example basic
wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/examples/basic.wasm
cd out
serve -s .