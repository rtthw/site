rm ./docs/rtthw.wasm
# rustdoc src/lib.rs -o docs -e style.css --crate-name rtthw --default-theme ayu
# echo "<meta http-equiv=\"refresh\" content=\"0; url=rtthw/index.html\">" > docs/index.html
# cargo run -r -p cleandocs -- ./docs

# As far as I can tell, this is EXACTLY the same as:
# `wasm-pack build --out-dir docs --target no-modules`
cargo build -r --lib --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rtthw.wasm docs/rtthw.wasm
wasm-bindgen docs/rtthw.wasm --target no-modules --out-dir docs
