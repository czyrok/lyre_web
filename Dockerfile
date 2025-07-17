#syntax=docker/dockerfile:1.7.0-labs

FROM rustlang/rust:nightly-alpine as installer

RUN apk update \
  && apk add --no-cache curl bash libc-dev binaryen \
  && curl --proto '=https' --tlsv1.3 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh \
  && curl --proto '=https' --tlsv1.3 -LsSf https://bun.com/install | bash -s "bun-v1.2.18" \
  && cargo install --no-default-features --debug cargo-make

# FIXME: le faire fonctionner lol
# COPY rust-toolchain.toml .
# RUN rustup toolchain install --no-self-update --profile minimal

RUN rustup target add wasm32-unknown-unknown

FROM installer as builder

WORKDIR /project

COPY .env .
COPY Cargo.toml .
COPY Cargo.lock .
COPY package.json .
COPY bun.lock .
COPY Makefile.toml .
COPY --parents makefiles/**/*.toml .

COPY --parents tailwind/**/*.ts .
COPY --parents tailwind/**/*.css .
COPY --parents project_data/**/*.md .
COPY --exclude=assets/tailwind_output.css --parents assets/**/* .

## Required by SQLx for macros
COPY local.db .

COPY --parents ./src/**/*.rs .

ENV PATH="/root/.bun/bin:$PATH"

RUN --mount=type=cache,target=/root/.bun/install/cache \
  --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} \
  --mount=type=cache,target=/project/target,id=${TARGETPLATFORM} \
  cargo make --profile release-ci build \
  && mkdir dist/ \
  && mv target/release/lyre_web dist/ \
  && mv target/site dist/

FROM scratch as runner

WORKDIR /app

COPY --from=builder /project/.env /app/
COPY --from=builder /project/Cargo.toml /app/

COPY --from=builder /project/project_data /app/project_data

COPY --from=builder /project/local.db /app/

COPY --from=builder /project/dist/lyre_web /app/
COPY --from=builder /project/dist/site /app/site

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8507"
ENV LEPTOS_SITE_ROOT=./site

EXPOSE 8507
CMD ["/app/lyre_web"]
