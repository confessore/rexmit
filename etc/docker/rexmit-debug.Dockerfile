FROM debian:bullseye-slim AS base
WORKDIR /


FROM rust AS chef
RUN apt update &&  \
    apt install -y git pip libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pip install 'git+https://github.com/ytdl-org/youtube-dl.git@master#egg=youtube_dl'
RUN cargo install cargo-chef
WORKDIR /


FROM base AS serenity-base
RUN apt update &&  \
    apt install -y git pip libssl-dev pkg-config libopus-dev ffmpeg && \
    rm -rf /var/lib/apt/lists/*
RUN pip install 'git+https://github.com/ytdl-org/youtube-dl.git@master#egg=youtube_dl'
WORKDIR /


FROM chef AS serenity-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-serenity ./src/rexmit-serenity
WORKDIR /usr/src/rexmit/src/rexmit-serenity
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS serenity-cacher
COPY --from=serenity-planner /usr/src/rexmit/src/rexmit /usr/src/rexmit/src/rexmit
COPY --from=serenity-planner /usr/src/rexmit/src/rexmit-serenity/recipe.json /usr/src/rexmit/src/rexmit-serenity/recipe.json
WORKDIR /usr/src/rexmit/src/rexmit-serenity
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-serenity/target \
    cargo chef cook --release --recipe-path recipe.json


FROM chef AS serenity-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-serenity ./src/rexmit-serenity
COPY --from=serenity-cacher /usr/src/rexmit/src/rexmit-serenity/target /usr/src/rexmit/src/rexmit-serenity/target
COPY --from=serenity-cacher $CARGO_HOME $CARGO_HOME
WORKDIR /usr/src/rexmit/src/rexmit-serenity
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release


FROM serenity-base AS serenity-prod
COPY --from=serenity-builder /usr/src/rexmit/src/rexmit-serenity/target/release/rexmit-serenity /usr/local/bin/rexmit-serenity
CMD ["rexmit-serenity"]