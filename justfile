run *args:
	cargo run -q -- {{args}}

build *args:
	cargo-watch -c -x "build"

build-release:
	cargo build --release

install:
	just build-release
	mkdir -p ~/.local/bin
	cp "$(pwd)/target/release/hyprland-workspace-manager" ~/.local/bin/hyprland-workspace-manager
	chmod +x ~/.local/bin/hyprland-workspace-manager

uninstall:
	rm -rf ~/.local/bin/hyprland-workspace-manager
