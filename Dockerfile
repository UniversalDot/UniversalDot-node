FROM ubuntu:20.04
LABEL name="Universaldot-node"
LABEL maintainer="https://github.com/UniversalDot"

ENV TZONE=Europe/Amsterdam
ENV LANG en_US.UTF-8
ENV LANGUAGE en_US:en
ENV LC_ALL en_US.UTF-8
RUN ln -snf /usr/share/zoneinfo/$TZONE /etc/localtime && echo $TZONE > /etc/timezone

# Install build dependencies
RUN apt-get update -y \
    && apt-get install -y automake \
    build-essential \
    apt-utils \
    curl \
    && apt-get clean

RUN apt update
RUN apt install -y cmake pkg-config libssl-dev git gcc build-essential git clang libclang-dev

RUN mkdir -p /root/setup
WORKDIR /root/setup
RUN curl https://getsubstrate.io -sSf | bash -s -- --fast
ENV PATH=/root/.cargo/bin:$PATH
RUN cargo --version
RUN rustup default stable
RUN rustup update nightly & \
    rustup update stable &\
    rustup target add wasm32-unknown-unknown --toolchain nightly

RUN git clone https://github.com/UniversalDot/universal-dot-node.git /universaldot-node
WORKDIR /universaldot-node
RUN git checkout -b add_pallets 
RUN cargo clean
RUN cargo build --release


WORKDIR /universaldot-node/target/release
# Expose ports
EXPOSE 9944