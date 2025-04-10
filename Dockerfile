# Stage 1: Build
FROM rust:1.85 as builder

# Install cross-compilation dependencies
RUN apt-get update && \
apt-get install -y musl-tools gcc-aarch64-linux-gnu && \
rustup target add aarch64-unknown-linux-musl

# Set the working directory
WORKDIR /usr/src/app

# Create .cargo/config.toml for cross-compilation
RUN mkdir -p .cargo
RUN echo '[target.aarch64-unknown-linux-musl]\nlinker = "aarch64-linux-gnu-gcc"' > .cargo/config.toml

# Copy the source code into the container
COPY . .

# Build the release version
RUN cargo build --target aarch64-unknown-linux-musl --release --all-features

# Stage 2: Runtime
FROM alpine:3.19

# Install minimal dependencies for a static binary
RUN apk --no-cache add ca-certificates

# Create a non-root user
RUN adduser -D appuser

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /usr/src/app/target/aarch64-unknown-linux-musl/release/lifers ./lifers


    # Copy static assets
    COPY assets ./assets/


# Set the ownership and permissions
RUN chown appuser:appuser ./lifers && \
chmod 755 ./lifers

# Switch to the non-root user
USER appuser

# Expose the port
EXPOSE 8080

# Start the application
CMD ["./lifers"]