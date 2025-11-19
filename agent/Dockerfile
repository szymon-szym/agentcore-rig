FROM rust:1.91 AS builder

RUN rustup target add aarch64-unknown-linux-gnu

WORKDIR /app

COPY . .

RUN cargo build --release --target aarch64-unknown-linux-gnu

FROM ubuntu:24.04 AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && update-ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/agent .

ENV AWS_REGION=us-east-1 \
    AWS_DEFAULT_REGION=us-east-1 \
    RUNNING_ENV=agentcore

RUN useradd -m appuser
USER appuser

CMD ["./agent"]
