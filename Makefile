test:
	cargo run --features native_dev

test-web:
	cargo build --features web_dev --target wasm32-unknown-unknown
	basic-http-server

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release --features native_release

build-web:
	cargo build --profile wasm-release --features web_release --target wasm32-unknown-unknown

build-apk:
	sudo docker run --rm -v $(pwd):/root/src: -w /root/src --name=quad-apk notfl3/cargo-apk cargo quad-apk build --features mobile_release

install:
	cargo install --path . --features native_release

test-docs:
	sphinx-autobuild -a -E --host 0.0.0.0 docs docs-out

build-docs:
	sphinx-build docs docs-out

publish:
	gh workflow run release.yml
