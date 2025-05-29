# Use the official Rust image to build the app
FROM rust:latest AS builder

# Set working directory inside container
WORKDIR /app

# Copy source code to the container
COPY . .

# Build the Rust project in release mode
RUN cargo build --release

# Use a minimal Debian image for runtime
FROM ubuntu:22.04

# Install necessary system dependencies
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder
COPY --from=builder /app/target/release/encryption_oracle /usr/local/bin/encryption_oracle

# Run the binary as the container entrypoint
ENTRYPOINT ["encryption_oracle"]

# Expose the port your Axum app uses
EXPOSE 3000
