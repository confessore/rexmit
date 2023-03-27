FROM debian:bullseye-slim AS base
WORKDIR /


FROM rust AS chef
RUN apt update &&  \
    apt install -y libssl-dev pkg-config libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef
WORKDIR /


FROM chef AS actix-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-actix ./src/rexmit-actix
WORKDIR /usr/src/rexmit/src/rexmit-actix
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS actix-cacher
COPY --from=actix-planner /usr/src/rexmit/src/rexmit /usr/src/rexmit/src/rexmit
COPY --from=actix-planner /usr/src/rexmit/src/rexmit-actix/recipe.json /usr/src/rexmit/src/rexmit-actix/recipe.json
WORKDIR /usr/src/rexmit/src/rexmit-actix
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-actix/target \
    cargo chef cook --release --recipe-path recipe.json


FROM chef AS actix-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-actix ./src/rexmit-actix
COPY --from=actix-cacher /usr/src/rexmit/src/rexmit-actix/target /usr/src/rexmit/src/rexmit-actix/target
COPY --from=actix-cacher $CARGO_HOME $CARGO_HOME
WORKDIR /usr/src/rexmit/src/rexmit-actix
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release


FROM base AS actix-prod
COPY --from=actix-builder /usr/src/rexmit/src/rexmit-actix/target/release/rexmit-actix /usr/local/bin/rexmit-actix
CMD ["rexmit-actix"]


FROM base AS serenity-base
RUN apt update &&  \
    apt install -y  git pip libssl-dev pkg-config libopus-dev ffmpeg youtube-dl && \
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


FROM rust:slim AS yew-base
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /

FROM chef AS yew-chef
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN --mount=type=cache,target=/var/cache/apt apt update && apt install curl -y && \
  curl -sL https://deb.nodesource.com/setup_16.x | bash - && \
  apt install nodejs -y && rm -rf /var/lib/apt/lists/*
RUN npm i -g tailwindcss
WORKDIR /


FROM yew-chef AS yew-planner
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-yew ./src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN cargo chef prepare --recipe-path recipe.json


FROM yew-chef AS yew-cacher
WORKDIR /usr/src/rexmit
COPY --from=yew-planner /usr/src/rexmit/src/rexmit /usr/src/rexmit/src/rexmit
COPY --from=yew-planner /usr/src/rexmit/src/rexmit-yew/recipe.json /usr/src/rexmit/src/rexmit-yew/recipe.json
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/rexmit/src/rexmit-yew/target \
    cargo chef cook --release --recipe-path recipe.json


FROM yew-chef AS yew-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit ./src/rexmit
COPY ./src/rexmit-yew ./src/rexmit-yew
COPY --from=yew-cacher /usr/src/rexmit/src/rexmit-yew/target /usr/src/rexmit/src/rexmit-yew/target
COPY --from=yew-cacher $CARGO_HOME $CARGO_HOME
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
RUN --mount=type=cache,target=/usr/local/cargo/registry trunk build


FROM yew-base AS yew-prod
COPY --from=yew-builder /usr/src/rexmit/src/rexmit /usr/src/rexmit/src/rexmit
COPY --from=yew-builder /usr/src/rexmit/src/rexmit-yew /usr/src/rexmit/src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN trunk build
CMD ["trunk", "serve", "--address", "0.0.0.0"]