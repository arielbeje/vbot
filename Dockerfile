# Builder image
FROM rust:1.55.0 AS builder
WORKDIR /tmp/src

RUN rustup target add x86_64-unknown-linux-musl

# Create a dummy project to build dependencies.
# Allows Docker to cache built dependencies as long as
# Cargo.toml and Cargo.lock don't change.
RUN USER=root cargo new vbot
WORKDIR /tmp/src/vbot
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Statically compile νbot
COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Output image
FROM scratch AS binary
COPY --from=builder /usr/local/cargo/bin/vbot .
USER 1000
CMD ["./vbot"]
