# =======================
#   Builder
# =======================
FROM rust:1.88-alpine AS builder
WORKDIR /usr/src/app

RUN apk add --no-cache musl-dev build-base nodejs npm

COPY . .

RUN npm i -g @asyncapi/cli@latest
RUN asyncapi --version
RUN asyncapi config versions
RUN asyncapi generate fromTemplate asyncapi.yaml @asyncapi/html-template@latest -o mqtt-docs --force-write --use-new-generator

RUN cargo build --release

# =======================
#   Runtime environment
# =======================
FROM alpine:latest
WORKDIR /app

RUN apk add --no-cache \
    sqlite \
    ca-certificates

COPY --from=builder /usr/src/app/target/release/mqtt-rest-bridge /app/mqtt-rest-bridge

COPY --from=builder /usr/src/app/mqtt-docs /app/mqtt-docs

COPY --from=builder /usr/src/app/config /app/config
COPY --from=builder /usr/src/app/queries /app/queries

CMD ["/app/mqtt-rest-bridge"]