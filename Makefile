all: build install

build:
	cargo build --release

install:
	cp target/release/to ~/bin
