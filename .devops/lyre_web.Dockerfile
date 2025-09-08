#syntax=docker/dockerfile:1.7.0-labs

FROM rustlang/rust:nightly-alpine as installer

RUN \
  apk update \
  && apk add --no-cache curl bash libc-dev binaryen \
  && curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/download/v0.2.37/cargo-leptos-installer.sh | sh \
  && curl --proto '=https' --tlsv1.3 -LsSf https://bun.com/install | bash -s "bun-v1.2.18" \
  && cargo install --no-default-features --debug cargo-make

# FIXME: le faire fonctionner lol
# COPY rust-toolchain.toml .
# RUN rustup toolchain install --no-self-update --profile minimal

RUN rustup target add wasm32-unknown-unknown

FROM installer as builder

WORKDIR /project

ENV PATH="/root/.bun/bin:$PATH"
## Incremental compilation adds unnecessary
## dependency-tracking and IO overhead,
## reducing caching effectiveness.
## This disables incremental compilation.
ENV CARGO_INCREMENTAL=0

## `local.db` required by SQLx for macros
RUN \
  --mount=type=bind,readonly,source=./.env,target=/project/.env \
  --mount=type=bind,readonly,source=./Cargo.toml,target=/project/Cargo.toml \
  --mount=type=bind,readonly,source=./Cargo.lock,target=/project/Cargo.lock \
  --mount=type=bind,readonly,source=./rust-toolchain.toml,target=/project/rust-toolchain.toml \
  --mount=type=bind,readonly,source=./.cargo,target=/project/.cargo \
  --mount=type=bind,readonly,source=./package.json,target=/project/package.json \
  --mount=type=bind,readonly,source=./bun.lock,target=/project/bun.lock \
  --mount=type=bind,readonly,source=./Makefile.toml,target=/project/Makefile.toml \
  --mount=type=bind,readonly,source=./makefiles,target=/project/makefiles \
  --mount=type=bind,readonly,source=./assets,target=/project/assets \
  --mount=type=bind,readonly,source=./project_data,target=/project/project_data \
  --mount=type=bind,readonly,source=./tailwind,target=/project/tailwind \
  --mount=type=bind,readonly,source=./local.db,target=/project/local.db \
  --mount=type=bind,readonly,source=./src,target=/project/src \
  --mount=type=cache,target=/project/target/ \
  --mount=type=cache,target=/usr/local/cargo/git/db \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  --mount=type=cache,target=/root/.bun/install/cache \
  cargo make --profile release-ci build \
  && mkdir dist/ \
  && mv target/release/lyre_web dist/ \
  && mv target/site dist/

FROM scratch as runner

WORKDIR /app

COPY .env .
COPY Cargo.toml .

COPY --parents project_data/**/*.md .
COPY local.db .

COPY --from=builder /project/dist/lyre_web /app/
COPY --from=builder /project/dist/site /app/site

ENV RUST_LOG=info
ENV LEPTOS_SITE_ADDR=0.0.0.0:8507
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8507
ENTRYPOINT ["/app/lyre_web"]
