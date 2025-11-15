FROM rust:1.91 AS builder

RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /app

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/agent .

RUN useradd -m appuser
USER appuser

CMD ["./agent"]
