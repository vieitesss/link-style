install: build
  sudo cp ./target/release/link-style /usr/local/bin/link-style

build:
  cargo build --release
