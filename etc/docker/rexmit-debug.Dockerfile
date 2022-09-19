FROM rust as builder
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
#RUN apt update && apt install -y <extra-runtime-dependencies> && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rexmit /usr/local/bin/rexmit
CMD ["rexmit"]