FROM debian:bookworm-slim AS base
WORKDIR /


FROM debian:bookworm-slim AS serenity-base
RUN apt update &&  \
    apt install -y libc6 libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /


FROM rust AS actix-builder
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo install --path ./src/rexmit-actix


FROM rust AS serenity-builder
RUN apt update &&  \
    apt install -y libopus-dev ffmpeg youtube-dl && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo install --path ./src/rexmit-serenity


FROM rust AS yew-builder
WORKDIR /usr/src/rexmit
COPY . .
RUN cargo install --path ./src/rexmit-yew


FROM base AS actix-prod
COPY --from=actix-builder /usr/local/cargo/bin/rexmit-actix /usr/local/bin/rexmit-actix
CMD ["rexmit-actix"]


FROM serenity-base AS serenity-prod
COPY --from=serenity-builder /usr/local/cargo/bin/rexmit-serenity /usr/local/bin/rexmit-serenity
CMD ["rexmit-serenity"]


FROM base AS yew-prod
COPY --from=yew-builder /usr/local/cargo/bin/rexmit-yew /usr/local/bin/rexmit-yew
CMD ["rexmit-yew"]