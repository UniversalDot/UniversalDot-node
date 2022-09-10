# Setup base layer
FROM --platform=$BUILDPLATFORM rustlang/rust:nightly as base
RUN rustup target add wasm32-unknown-unknown; \
    apt-get update -y; \
    apt-get install -y cmake pkg-config libssl-dev git gcc build-essential clang libclang-dev; \
    cargo install cargo-chef # Use cargo-chef to simplify dependency caching

# Initialise environment variable based on target architecture (maps from buildkit to Rust convention)
FROM base AS base-amd64
ENV ARCH=x86_64
FROM base AS base-arm64
ENV ARCH=aarch64

# Create recipe based on all available cargo.lock and cargo.toml manifests in project - used to cache dependencies
FROM base AS planner
WORKDIR node
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Build
FROM base-${TARGETARCH} AS builder
# Configure based on target architecture
ARG TARGETARCH
ENV TARGET=${ARCH}-unknown-linux-gnu RUSTFLAGS="-C linker=${ARCH}-linux-gnu-gcc"
RUN rustup target add ${ARCH}-unknown-linux-gnu; \
    apt-get install -y gcc-${ARCH}-linux-gnu g++-${ARCH}-linux-gnu libc6-dev-${TARGETARCH}-cross; \
    # Add missing symlinks
    ln -s /usr/${ARCH}-linux-gnu/include/bits /usr/include/bits; \
    ln -s /usr/${ARCH}-linux-gnu/include/sys /usr/include/sys; \
    ln -s /usr/${ARCH}-linux-gnu/include/gnu /usr/include/gnu
# Build dependencies using recipe created in previous stage - this is the caching Docker layer!
WORKDIR node
COPY --from=planner /node/recipe.json recipe.json
RUN cargo chef cook --release --target $TARGET --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --target $TARGET

# Construct runtime
FROM debian:stable-slim as runtime
ENV ENVIRONMENT=dev WEBSOCKET=ws-external
COPY --from=builder /node/target/*/release/node-template /usr/local/bin/node
CMD /usr/local/bin/node --$ENVIRONMENT --$WEBSOCKET
EXPOSE 9944

# Finally set metadata
LABEL org.opencontainers.image.vendor="UniversalDot" \
    org.opencontainers.image.url="https://github.com/universaldot/universal-dot-node" \
    org.opencontainers.image.title="UniversalDot Node" \
    org.opencontainers.image.description="A blockchain for creating digital economies." \
    org.opencontainers.image.documentation="https://github.com/universaldot/universal-dot-node"