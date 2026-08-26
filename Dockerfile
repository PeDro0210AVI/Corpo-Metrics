ARG RUST_VERSION=1.90 
ARG APP_NAME=corpo_metrics
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
ARG PORT
WORKDIR /app


# thx docker docs :D
#For taking advatange of rust cache
RUN --mount=type=bind,source=src,target=src \
  --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
  --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
  --mount=type=cache,target=/app/target/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
  <<EOF
set -e
cargo build --locked --release
cp ./target/release/$APP_NAME /bin/server
EOF

FROM ubuntu:latest AS final

COPY data/ data/
COPY --from=build /bin/server /bin/

EXPOSE PORT

CMD ["/bin/server"]
