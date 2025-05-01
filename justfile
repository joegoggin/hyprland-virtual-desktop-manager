run *args:
	cargo run -q -- {{args}}

build *args:
	cargo-watch -c -x "build"

build-release:
	cargo build --release
