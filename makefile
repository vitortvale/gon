build:
	 cargo build --release --bin gon

run:
	 ./target/release/gon

test:
	 ./target/release/gon ./src/tests/requests.http


