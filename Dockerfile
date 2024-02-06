FROM rust:latest AS builder

RUN apt-get update && \
    apt-get install -y clang libclang-dev git

WORKDIR /usr/src

RUN git clone https://github.com/tfsingh/xor-redis.git

WORKDIR /usr/src/xor-redis

RUN cargo build --release

FROM redis/redis-stack-server:latest

COPY --from=builder /usr/src/xor-redis/target/release/libxor_redis.so /usr/lib/

CMD ["redis-server", "--protected-mode", "no", "--loadmodule", "/usr/lib/libxor_redis.so"]