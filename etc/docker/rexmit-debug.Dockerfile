FROM rust AS builder
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/rexmit
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo install --path .

FROM debian:buster-slim AS stage
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rexmit /usr/local/bin/rexmit
CMD ["rexmit"]