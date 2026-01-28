FROM rust:1.93 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:trixie-slim

RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/telegram-send-file-action /telegram-send-file-action

ENTRYPOINT ["/telegram-send-file-action"]
