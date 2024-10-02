# First stage: Build the application
FROM rust:1.75 as builder
RUN apt-get update && apt-get install -y musl-tools
WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Build dependencies only
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src
COPY ./src ./src
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --release --target x86_64-unknown-linux-musl

# Second stage: Create a minimal image with the binary
FROM alpine:3.18
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/ReplicatedLog .
RUN chmod +x /app/ReplicatedLog
EXPOSE 8080
CMD ["./ReplicatedLog"]
