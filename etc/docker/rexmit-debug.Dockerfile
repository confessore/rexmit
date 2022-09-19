FROM rust AS builder
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim AS stage
RUN apt update &&  \
    apt install -y libc6 libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rexmit /usr/local/bin/rexmit
CMD ["rexmit"]