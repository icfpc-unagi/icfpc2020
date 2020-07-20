FROM golang:latest

ARG ENDPOINT
ENV ENDPOINT=$ENDPOINT
WORKDIR /build
COPY . .
RUN go build -o /usr/local/bin/proxy-internal ./cmd/proxy
RUN echo '#!/usr/bin/env bash' >>/usr/local/bin/proxy && \
    echo "exec /usr/local/bin/proxy-internal --endpoint='$ENDPOINT' \"\$@\"" \
        >>/usr/local/bin/proxy && \
    chmod +x /usr/local/bin/proxy
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/proxy", "--logtostderr", "--port=:8080"]
