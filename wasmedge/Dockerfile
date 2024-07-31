# syntax=docker/dockerfile:1

FROM --platform=$BUILDPLATFORM rust:latest AS buildbase
WORKDIR /src

FROM buildbase AS build
#COPY Cargo.toml orders.json update_order.json ./
COPY Cargo.toml ./
COPY src ./src
# Build the Wasm binary
RUN cargo build --release
# This line builds the AOT Wasm binary
RUN cp target/release/docker-wasmedge docker-native

FROM debian:12-slim
ENTRYPOINT [ "/docker-native" ]
COPY --from=build /src/docker-native /docker-native
RUN chmod a+x /docker-native
