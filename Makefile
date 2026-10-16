run:
	cargo run --no-default-features --features dev,singleplayer,multiplayer,remote_host,env_logger

run-web:
	trunk serve -c -a 0.0.0.0 --no-default-features --features web,singleplayer,multiplayer,dev

run-server:
	cargo run --no-default-features --features server_build,dev

fmt:
	cargo fmt --all

clean:
	cargo clean

build:
	cargo build --release

build-web:
	trunk build --features --no-default-features --features web_build

install:
	cargo install --path .

test-docs:
	sphinx-autobuild -a -E --host 0.0.0.0 docs docs-out

build-docs:
	sphinx-build docs docs-out

publish:
	gh workflow run release.yml
