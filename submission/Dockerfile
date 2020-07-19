FROM rust:1.44.1-buster

WORKDIR /solution
COPY vendor vendor
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY .cargo .cargo
RUN mkdir -p src/bin && \
	echo 'fn main() { println!("Hello, world!"); }' >src/bin/hello.rs && \
	cargo build --release --offline --bin=hello
COPY . .
RUN chmod +x ./build.sh
RUN chmod +x ./run.sh
RUN ./build.sh
ENTRYPOINT ["./run.sh"]
