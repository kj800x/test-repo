# Build Stage
FROM rust:1.85-alpine AS builder
WORKDIR /usr/src/
# Install required build dependencies
RUN apk add --no-cache musl-dev pkgconfig openssl-dev libc-dev gcc

# - Install dependencies
RUN USER=root cargo new rust-hello-world
WORKDIR /usr/src/rust-hello-world
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# - Copy source
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime Stage
FROM alpine:latest AS runtime
WORKDIR /app
# Install runtime dependencies if needed
# RUN apk add --no-cache ca-certificates

COPY --from=builder /usr/src/rust-hello-world/target/release/rust-hello-world ./rust-hello-world
USER 1000
CMD ["./rust-hello-world"]
