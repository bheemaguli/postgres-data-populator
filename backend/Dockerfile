FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
RUN cargo install --version 0.1.68 cargo-chef
WORKDIR /backend

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /backend/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /backend
COPY --from=builder /backend/target/release/backend /usr/local/bin
EXPOSE 3000/tcp
ENTRYPOINT ["/usr/local/bin/backend"]