
all:
	make clean
	make build
	make test
	make run

clean:
	cargo clean

build:
	cargo build

test:
	cargo test

client:
	cargo run --bin client

run:
	cargo run --bin server -- \
	--stdout-log

fmt:
	/usr/bin/find ./ -type f -name "*.rs" -exec rustfmt {} \;
