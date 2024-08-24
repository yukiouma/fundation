FROM rust:bookworm AS builder

WORKDIR /usr/src/app

ENV DATABASE_URL=mysql://root:000000@172.17.0.3:3306/fundation?parseTime=True

COPY .cargo/config.toml ~/.cargo/config.toml

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/repository .

ENV DATABASE_URL=mysql://root:000000@172.17.0.3:3306/fundation?parseTime=True

CMD [ "/app/repository" ]



