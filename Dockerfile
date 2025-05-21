# Step 1: Use the cargo-chef image to build the application in a multi-stage build
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Step 2: Plan the build by copying the code and generating the build recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Step 3: Build the dependencies using the recipe (this step will be cached)
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# Step 4: Build the application binary
COPY . .
RUN cargo build --release --bin webserver

# Step 5: Create a smaller image for the runtime
FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install necessary system dependencies for PostgreSQL, SSL, and runtime
RUN apt-get update && apt-get install -y \
    libssl-dev \
    libpq-dev \
    ca-certificates \
    openssl \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/webserver /app/webserver

# Copy .env files from the build context (if they exist)
COPY .env* /app/

# Generate vapid.pem if it doesn’t exist and create a minimal .env for VAPID_FILE
RUN if [ ! -f vapid.pem ]; then \
    openssl ecparam -name prime256v1 -genkey -noout -out vapid.pem; \
    echo "VAPID_FILE=./vapid.pem" > .env; \
    fi

# Set the binary as the entrypoint
ENTRYPOINT ["./webserver"]