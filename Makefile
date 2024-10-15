build:
	cargo build --release

install:
	cp ./target/release/link-style /usr/local/bin/link-style
