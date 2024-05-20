FROM rust:latest as builder

WORKDIR /app

COPY . /app

RUN cargo build --release

CMD ["./target/release/minimal-http-server"]
