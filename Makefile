test:
	cargo run --features dev

test-web:
	cargo build --features web,dev --target wasm32-unknown-unknown
	basic-http-server

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release

build-web:
	cargo build --features web --target wasm32-unknown-emscripten

build-apk:
	sudo docker run --rm -v $(pwd):/root/src: -w /root/src --name=quad-apk notfl3/cargo-apk cargo quad-apk build --features mobile

install:
	cargo install --path .

test-docs:
	sphinx-autobuild -a -E --host 0.0.0.0 docs docs-out

build-docs:
	sphinx-build docs docs-out

publish:
	gh workflow run release.yml
