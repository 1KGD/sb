test:
	cargo run

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release --features native_release

build-web:
	cargo build --release --features web_release --target wasm32-unknown-unknown

install:
	cargo install --path . --features native_release
