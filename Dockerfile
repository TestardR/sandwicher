FROM rust:1.72.1 AS builder

WORKDIR /src

COPY . /src
RUN cargo build --release

FROM gcr.io/distroless/cc AS release

COPY --from=builder /src/target/release/sandwicher /sandwicher
COPY --from=builder /src/migrations /migrations

CMD ["/sandwicher"]