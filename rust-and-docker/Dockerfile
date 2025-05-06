FROM rust:latest as builder

# Install the musl target
RUN rustup target add aarch64-unknown-linux-musl

# Create project
RUN cargo new --bin rust-and-docker
WORKDIR /rust-and-docker

# Copy files
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build statically linked binary
RUN cargo build --release --target aarch64-unknown-linux-musl

# Final image
FROM alpine:latest
COPY --from=builder /rust-and-docker/target/aarch64-unknown-linux-musl/release/rust-and-docker /rust-and-docker
CMD ["/rust-and-docker"]