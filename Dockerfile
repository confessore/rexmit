FROM debian:bullseye-slim AS rexmit-base
RUN apt update &&  \
    apt install -y git pip libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pip install yt-dlp
WORKDIR /


FROM rust AS rexmit-builder-base
RUN apt update &&  \
    apt install -y git pip libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pip install yt-dlp
WORKDIR /


FROM rexmit-builder-base AS rexmit-builder
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo build --release


FROM rexmit-base AS rexmit-production
ARG DISCORD_TOKEN
ENV DISCORD_TOKEN=$DISCORD_TOKEN
COPY --from=rexmit-builder /usr/src/rexmit/target/release/rexmit /usr/local/bin/rexmit
CMD ["rexmit"]