rm -rf ./docs
rustdoc src/lib.rs -o docs --crate-name rtthw
echo "<meta http-equiv=\"refresh\" content=\"0; url=rtthw/index.html\">" > docs/index.html
