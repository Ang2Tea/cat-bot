ARG RUST_VERSION=1.90
ARG APP_NAME=cat-bot
ARG EXECUTABLE_NAME=standalone

FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
ARG EXECUTABLE_NAME

WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git libressl-dev

RUN --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=cat-core,target=cat-core \
    --mount=type=bind,source=cat-adapters,target=cat-adapters \
    --mount=type=bind,source=cat-diesel-repo,target=cat-diesel-repo \
    --mount=type=bind,source=.sqlx,target=.sqlx \
    --mount=type=bind,source=cat-sqlx-repo,target=cat-sqlx-repo \
    --mount=type=bind,source=cat-bot,target=cat-bot \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
cargo build --locked --release --package ${APP_NAME} --bin ${EXECUTABLE_NAME} && \
cp ./target/release/${EXECUTABLE_NAME} /bin/server



FROM alpine:latest AS final

WORKDIR /app

ENV TZ=Europe/Moscow

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /app/

ENTRYPOINT ["./server"]