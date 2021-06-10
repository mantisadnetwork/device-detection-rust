FROM rust:1.52-buster

WORKDIR /opt/rust

# dummy files so we can compile and build dependencies
RUN mkdir src
RUN echo "fn main(){}" > src/lib.rs
RUN echo "fn main(){}" > src/cli.rs

# define dependencies for temporary build
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# cache dependency compilation
RUN --mount=type=cache,target=/root/.cargo/registry/ cargo build --release

# remove dummy files and compilation cache (not dependency cache)
RUN rm -rf src
RUN rm -rf target/release/**/libsrc*

# copy files needed for normal build
COPY device-detection-cxx device-detection-cxx
COPY src src
COPY build.rs build.rs

RUN cargo build --release