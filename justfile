run *args:
	cargo run -q -- {{args}}

build *args:
	cargo-watch -c -x "build"

build-release:
	cargo build --release

install:
	just build-release
	mkdir -p ~/.local/bin
	cp "$(pwd)/target/release/hyprland-vdm" ~/.local/bin/hyprland-vdm
	chmod +x ~/.local/bin/hyprland-vdm

sim-link:
	just build-release
	mkdir -p ~/.local/bin
	ln -s "$(pwd)/target/release/hyprland-vdm" ~/.local/bin/hyprland-vdm

uninstall:
	rm -rf ~/.local/bin/hyprland-vdm
