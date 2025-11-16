FROM rust:1.91 AS builder

RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /app

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM ubuntu:24.04 AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends libssl3 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/agent .

RUN useradd -m appuser
USER appuser

CMD ["./agent"]
