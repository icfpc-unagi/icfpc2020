FROM rust:1.44.1-buster

WORKDIR /solution
COPY vendor vendor
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
RUN mkdir -p src/bin && \
	echo 'fn main() { println!("Hello, world!"); }' >src/bin/hello.rs && \
	cargo build --release --bin=hello
COPY src src
RUN cargo build --release --bin=get_room
ENTRYPOINT ["./target/release/get_room"]
