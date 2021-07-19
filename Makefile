all:
	cargo build --release --target x86_64-unknown-linux-musl --bin container-expl
	strip target/x86_64-unknown-linux-musl/release/container-expl
