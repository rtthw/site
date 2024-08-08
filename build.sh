rm -rf ./docs
rustdoc src/lib.rs -o docs -e style.css --crate-name rtthw --default-theme ayu
echo "<meta http-equiv=\"refresh\" content=\"0; url=rtthw/index.html\">" > docs/index.html
cargo run -r -p cleandocs -- ./docs
