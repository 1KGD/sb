test:
	RUST_LOG=starbloom=INFO cargo run --features dev

test-web:
	trunk serve -a 0.0.0.0 --features web,dev

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release

build-web:
	trunk build --features web

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
