
export OPENSSL_STATIC=1
export OPENSSL_DIR=/usr

default: .build-rpm

.requirements:
	sudo apt-get install -y libssl-dev pkg-config musl-tools musl-dev

.pre:
	rustup target add x86_64-unknown-linux-musl
	cargo install cargo-generate-rpm

.test:
	cargo test

.clean:
	cargo clean

.build: .test
	cargo build --release

.build-static: .test .requirements .pre
	cargo build --release --target x86_64-unknown-linux-musl

.build-rpm: .build-static
	cargo generate-rpm --target x86_64-unknown-linux-musl
