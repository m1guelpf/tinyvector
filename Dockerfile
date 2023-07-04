FROM lukemathwalker/cargo-chef:latest-rust-bookworm AS chef
WORKDIR /tinyvector

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /tinyvector/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin tinyvector

FROM debian:bookworm-slim as runtime
WORKDIR /tinyvector
COPY --from=builder /tinyvector/target/release/tinyvector /usr/local/bin

EXPOSE 8000
VOLUME ["/storage"]
ENTRYPOINT ["/usr/local/bin/tinyvector"]

HEALTHCHECK --interval=5m \
    CMD curl -f http://localhost:8000/ || exit 1
