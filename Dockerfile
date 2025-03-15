FROM rust:1.85 as builder

WORKDIR /usr/src/app
COPY . .

# Build the application with release optimizations
RUN cargo build --release

# Create a smaller runtime image
FROM debian:bookworm-slim

# Install necessary libraries
RUN apt-get update && \
  apt-get install -y libssl-dev ca-certificates && \
  rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-hello-world /app/

# Expose the port the server listens on
EXPOSE 8080

# Run the binary
CMD ["./rust-hello-world"]
