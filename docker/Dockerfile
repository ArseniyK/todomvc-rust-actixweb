FROM rust:1.40.0-stretch as builder

ENV DOCKERIZE_VERSION=v0.6.1 \
    PKG_CONFIG_ALLOW_CROSS=1
# muslc is required in order to build the rust image.
RUN apt-get update \
    && apt-get -y install \
    ca-certificates \
    cmake \
    musl-tools \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/* \
    && wget "https://github.com/jwilder/dockerize/releases/download/${DOCKERIZE_VERSION}/dockerize-alpine-linux-amd64-${DOCKERIZE_VERSION}.tar.gz" \
    && tar -C /usr/local/bin -xzvf "dockerize-alpine-linux-amd64-${DOCKERIZE_VERSION}.tar.gz" \
    && rm "dockerize-alpine-linux-amd64-${DOCKERIZE_VERSION}.tar.gz"

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /code
RUN USER=root cargo new todomvc

COPY Cargo.toml Cargo.lock /code/todomvc/

WORKDIR /code/todomvc/

RUN cargo fetch
RUN cargo install cargo-watch
RUN cargo install diesel_cli --no-default-features --features=postgres
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY . /code/todomvc
RUN cargo build --target x86_64-unknown-linux-musl --release
CMD "cargo watch -x run"

FROM alpine:3.8 as production_build

RUN apk --no-cache add ca-certificates
COPY --from=builder /code/todomvc/target/x86_64-unknown-linux-musl/release/todomvc-rust-actixweb .