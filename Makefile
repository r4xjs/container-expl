TARGET = x86_64-unknown-linux-musl
BIN_PATH = target/x86_64-unknown-linux-musl/release/container-expl
RELEASE_UPLOAD = /var/www/html/release/


build:
	cargo build --release --target $(TARGET) --bin container-expl
	strip $(BIN_PATH)
release:
	cargo build --release --target $(TARGET) --bin container-expl
	strip $(BIN_PATH)
	chmod 755 $(BIN_PATH)
	scp $(BIN_PATH) www:$(RELEASE_UPLOAD)

