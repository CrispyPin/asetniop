build:
	cargo build

release:
	cargo build --release

t: test
test: build
	sudo ./target/debug/asetniop

r: run
run: release
	sudo ./target/release/asetniop
