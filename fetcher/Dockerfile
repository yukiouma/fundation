FROM rust:bookworm AS builder

WORKDIR /usr/src/app

COPY .cargo/config.toml ~/.cargo/config.toml

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && \
    apt-get install apt-transport-https ca-certificates -y && \
    apt-get install -y libssl-dev 

COPY --from=builder /usr/src/app/target/release/fetcher .

COPY --from=builder /usr/src/app/.config .

CMD [ "/app/fetcher" ]

# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble main restricted universe multiverse \
# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-updates main restricted universe multiverse \
# deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-backports main restricted universe multiverse \

# RUN apt-get install apt-transport-https ca-certificates && \
#     echo "deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble main restricted universe multiverse\n\
#     deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-updates main restricted universe multiverse\n\
#     deb https://mirrors.tuna.tsinghua.edu.cn/ubuntu/ noble-backports main restricted universe multiverse\n"\
#     > /etc/apt/sources.list && \
#     apt-get update && \
#     apt-get install -y libssl-dev 