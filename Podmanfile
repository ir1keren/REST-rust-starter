# Use official Rust image as base
FROM rust:1.75 as builder

# Set working directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src
COPY static ./static

# Build the application
RUN cargo build --release

# Use distroless image for final runtime
FROM gcr.io/distroless/cc-debian12

# Copy the binary from builder stage
COPY --from=builder /app/target/release/rust-mvc-api /usr/local/bin/rust-mvc-api

# Copy static files
COPY --from=builder /app/static /app/static

# Set working directory
WORKDIR /app

# Expose port
EXPOSE 8080

# Set environment variables
ENV RUST_LOG=info
ENV SERVER_HOST=0.0.0.0
ENV SERVER_PORT=8080

# Run the binary
ENTRYPOINT ["/usr/local/bin/rust-mvc-api"]