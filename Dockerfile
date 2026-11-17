FROM rust:1-bookworm AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --locked

FROM debian:bookworm-slim

WORKDIR /app

# The bot writes its database to ./data/data.json
RUN mkdir data

COPY --from=builder /usr/src/app/target/release/vocal-time-counter ./bot

CMD ["./bot"]
