test:
	cargo run --features desktop_build,dev

test-web:
	trunk serve -c -a 0.0.0.0 --features web,dev

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release

build-web:
	trunk build --features web

install:
	cargo install --path .

test-docs:
	sphinx-autobuild -a -E --host 0.0.0.0 docs docs-out

build-docs:
	sphinx-build docs docs-out

publish:
	gh workflow run release.yml
