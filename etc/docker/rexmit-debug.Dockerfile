FROM debian:bookworm-slim AS base
WORKDIR /


FROM rust AS chef
RUN cargo install cargo-chef
WORKDIR /


FROM chef AS actix-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-actix ./src/rexmit-actix
WORKDIR /usr/src/rexmit/src/rexmit-actix
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS actix-cacher
WORKDIR /usr/src/rexmit/src/rexmit-actix
COPY --from=actix-planner /usr/src/rexmit/src/rexmit-actix/recipe.json /usr/src/rexmit/src/rexmit-actix/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-actix/target \
    cargo chef cook --release --recipe-path recipe.json


FROM chef AS actix-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-actix ./src/rexmit-actix
WORKDIR /usr/src/rexmit/src/rexmit-actix
COPY --from=actix-cacher /usr/src/rexmit/src/rexmit-actix/target /usr/src/rexmit/src/rexmit-actix/target
COPY --from=actix-cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release


FROM base AS actix-prod
COPY --from=actix-builder /usr/src/rexmit/src/rexmit-actix/target/release/rexmit-actix /usr/local/bin/rexmit-actix
CMD ["rexmit-actix"]


FROM debian:bookworm-slim AS serenity-base
RUN apt update &&  \
    apt install -y libc6 libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /


FROM rust AS serenity-chef
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef


FROM serenity-chef AS serenity-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-serenity ./src/rexmit-serenity
WORKDIR /usr/src/rexmit/src/rexmit-serenity
RUN cargo chef prepare --recipe-path recipe.json


FROM serenity-chef AS serenity-cacher
WORKDIR /usr/src/rexmit/src/rexmit-serenity
COPY --from=serenity-planner /usr/src/rexmit/src/rexmit-serenity/recipe.json /usr/src/rexmit/src/rexmit-serenity/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-serenity/target \
    cargo chef cook --release --recipe-path recipe.json


FROM serenity-chef AS serenity-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-serenity ./src/rexmit-serenity
WORKDIR /usr/src/rexmit/src/rexmit-serenity
COPY --from=serenity-cacher /usr/src/rexmit/src/rexmit-serenity/target /usr/src/rexmit/src/rexmit-serenity/target
COPY --from=serenity-cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release


FROM serenity-base AS serenity-prod
COPY --from=serenity-builder /usr/src/rexmit/src/rexmit-serenity/target/release/rexmit-serenity /usr/local/bin/rexmit-serenity
CMD ["rexmit-serenity"]


FROM rust:slim AS yew-base
RUN cargo install trunk
WORKDIR /


FROM chef AS yew-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-yew ./src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS yew-cacher
WORKDIR /usr/src/rexmit/src/rexmit-yew
COPY --from=yew-planner /usr/src/rexmit/src/rexmit-yew/recipe.json /usr/src/rexmit/src/rexmit-yew/recipe.json
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-yew/target \
    cargo chef cook --release --recipe-path recipe.json


FROM yew-base AS yew-builder
RUN rustup target add wasm32-unknown-unknown
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-yew ./src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
COPY --from=yew-cacher /usr/src/rexmit/src/rexmit-yew/target /usr/src/rexmit/src/rexmit-yew/target
COPY --from=yew-cacher $CARGO_HOME $CARGO_HOME
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    trunk build


FROM yew-base AS yew-prod
COPY --from=yew-builder /usr/src/rexmit/src/rexmit-yew /usr/src/rexmit/src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
CMD ["trunk", "serve", "--address", "0.0.0.0"]