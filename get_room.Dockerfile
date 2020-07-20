FROM rust:1.44.1-buster

WORKDIR /solution
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
RUN cargo build --release --bin=get_room && \
    cp ./target/release/get_room /usr/local/bin/get_room && \
	rm -rf ./target ~/.cache /usr/local/cargo/registry || true
ARG ENDPOINT
ENV ENDPOINT=$ENDPOINT
RUN mv /usr/local/bin/get_room /usr/local/bin/get_room-internal
RUN echo '#!/usr/bin/env bash' >>/usr/local/bin/get_room && \
    echo "exec /usr/local/bin/get_room-internal '$ENDPOINT' \"\$@\"" \
        >>/usr/local/bin/get_room && \
    chmod +x /usr/local/bin/get_room
ENTRYPOINT ["/usr/local/bin/get_room"]
