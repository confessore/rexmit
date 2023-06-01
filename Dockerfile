FROM debian:bookworm-slim AS rexmit-base
RUN apt update &&  \
    apt install -y git pipx libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pipx install yt-dlp
WORKDIR /


FROM rustlang/rust:nightly-bookworm-slim AS rexmit-builder-base
RUN apt update &&  \
    apt install -y git pipx libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pipx install yt-dlp
WORKDIR /


FROM rexmit-builder-base AS rexmit-builder
WORKDIR /usr/src/rexmit
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "// this is a dummy file for build caching" > src/lib.rs && \
    cargo build --release && \
    rm -r src
COPY . .
RUN cargo build --release


FROM rexmit-base AS rexmit-production
ARG DEBUG
ENV DEBUG=$DEBUG
ARG DISCORD_TOKEN
ENV DISCORD_TOKEN=$DISCORD_TOKEN
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
COPY --from=rexmit-builder /usr/src/rexmit/target/release/rexmit /usr/local/bin/rexmit
CMD ["rexmit"]