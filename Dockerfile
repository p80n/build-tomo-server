FROM rustlang/rust:nightly-alpine AS build
# FROM buildpack-deps:buster AS build

# ENV RUSTUP_HOME=/usr/local/rustup \
#     CARGO_HOME=/usr/local/cargo \
#     PATH=/usr/local/cargo/bin:$PATH \
#     RUST_VERSION=nightly \
#     RUSTUP_URL="https://static.rust-lang.org/rustup/archive/1.23.1/x86_64-unknown-linux-gnu/rustup-init" \
#     RUSTUP_SHA256="ed7773edaf1d289656bdec2aacad12413b38ad0193fff54b2231f5140a4b07c5"

# RUN wget "$RUSTUP_URL" && \
#     echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c - && \
#     chmod +x rustup-init && \
#     ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION --default-host x86_64-unknown-linux-gnu && \
#     rm rustup-init && \
#     chmod -R a+w $RUSTUP_HOME $CARGO_HOME


RUN cargo new builder
WORKDIR /builder
COPY Cargo.toml Cargo.lock /builder/
RUN apk --no-cache add musl-dev openssl-dev && \
    cargo build --release

COPY src /builder/src
RUN cargo build --release && \
    cargo install --target x86_64-unknown-linux-musl --path ./

FROM rust:alpine

WORKDIR /app
COPY --from=build /usr/local/cargo/bin/build-tomo-server .

ENTRYPOINT ["/app/build-tomo-server"]
