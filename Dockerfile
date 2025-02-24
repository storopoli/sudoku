# Start from the official Rust image
FROM rust:latest as builder

# Install musl-tools
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Target the wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Set the working directory
WORKDIR /app

# Add labels for OCI annotations
LABEL org.opencontainers.image.source="https://github.com/storopoli/sudoku" \
    org.opencontainers.image.description="Sudoku" \
    org.opencontainers.image.licenses="MIT"

# Install diouxus-cli
RUN cargo install dioxus-cli --locked

# Copy project's root dir
COPY . .

# Build the application
RUN dx build --release --verbose

# Start a new stage from a slim version of Debian to reduce the size of the final image
FROM nginx:alpine

# Copy the dist/ from the builder stage to the new stage
COPY --from=builder /app/dist /dist
WORKDIR /dist
COPY ./nginx.conf /etc/nginx/nginx.conf
