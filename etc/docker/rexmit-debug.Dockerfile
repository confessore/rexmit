FROM debian:bookworm-slim AS base
WORKDIR /


FROM debian:bookworm-slim AS serenity-base
RUN apt update &&  \
    apt install -y libc6 libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /


FROM rust:slim AS yew-base
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /


FROM rust AS actix-builder
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-actix ./src/rexmit-actix
RUN cargo install --path ./src/rexmit-actix


FROM rust AS serenity-builder
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-serenity ./src/rexmit-serenity
RUN cargo install --path ./src/rexmit-serenity


FROM rust AS yew-builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
WORKDIR /usr/src/rexmit
COPY ./src/rexmit-yew ./src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
RUN trunk build


FROM base AS actix-prod
COPY --from=actix-builder /usr/local/cargo/bin/rexmit-actix /usr/local/bin/rexmit-actix
CMD ["rexmit-actix"]


FROM serenity-base AS serenity-prod
COPY --from=serenity-builder /usr/local/cargo/bin/rexmit-serenity /usr/local/bin/rexmit-serenity
CMD ["rexmit-serenity"]


FROM yew-base AS yew-prod
WORKDIR /usr/src/rexmit
COPY --from=yew-builder /usr/src/rexmit/src/rexmit-yew /usr/src/rexmit/src/rexmit-yew
WORKDIR /usr/src/rexmit/src/rexmit-yew
CMD ["trunk", "serve", "--address", "0.0.0.0"]