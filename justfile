run *args:
	cargo run -q -- {{args}}

build *args:
	cargo-watch -c -x "build"
