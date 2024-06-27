# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
WORKDIR /src
RUN <<EOT bash
    set -ex
    rustup target add wasm32-wasi
EOT

FROM buildbase AS build
#COPY Cargo.toml orders.json update_order.json ./
COPY Cargo.toml ./
COPY src ./src
# Build the Wasm binary
RUN cargo build --target wasm32-wasi --release
# This line builds the AOT Wasm binary
RUN cp target/wasm32-wasi/release/docker-wasmedge.wasm docker-wasmedge.wasm
RUN chmod a+x docker-wasmedge.wasm

FROM scratch
ENTRYPOINT [ "/docker-wasmedge.wasm" ]
COPY --link --from=build /src/docker-wasmedge.wasm /docker-wasmedge.wasm
