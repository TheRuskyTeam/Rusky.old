CARGO=cargo
BOT_DEBUG=./target/debug/rusky
all:
	$(CARGO) fmt # Format
	$(CARGO) clippy --quiet # Clippy Check
	$(CARGO) build --quiet # Build
fix:
	$(CARGO) clippy -Z unstable-options --fix --allow-dirty -q
	$(CARFO) fix -q --allow-dirty
run: all
	$(BOT_DEBUG) run
