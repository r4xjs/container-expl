TARGET = x86_64-unknown-linux-musl
BIN_PATH = target/x86_64-unknown-linux-musl/release/container-expl
RELEASE_UPLOAD = /var/www/html/release/


.PHONY: build
build:
	cargo build --release --target $(TARGET) --bin container-expl
	strip $(BIN_PATH)

.PHONY: clippy
clippy:
	cargo clippy --release --target $(TARGET) --bin container-expl

.PHONY: release
release: build
	chmod 755 $(BIN_PATH)
	scp $(BIN_PATH) www:$(RELEASE_UPLOAD)

