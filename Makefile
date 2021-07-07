CARGO=cargo
BOT_DEBUG=./target/debug/rusky
all:
	$(CARGO) fmt # Format
	$(CARGO) clippy --quiet # Clippy Check
	$(CARGO) build --quiet # Build
run: all
	$(BOT_DEBUG) run
