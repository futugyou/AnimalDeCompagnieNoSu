# Use a lighter base image
FROM rust:1-slim-buster AS builder

# Set environment variables
ENV PKG_CONFIG_ALLOW_CROSS=1

# Create a working directory
WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the workspace's Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the Cargo.toml file for each member crate
COPY entity/Cargo.toml entity/Cargo.toml
COPY infrastruct/Cargo.toml infrastruct/Cargo.toml
COPY model/Cargo.toml model/Cargo.toml
COPY repository/Cargo.toml repository/Cargo.toml
COPY serialize/Cargo.toml serialize/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY tool/Cargo.toml tool/Cargo.toml
COPY web/Cargo.toml web/Cargo.toml

# Create an empty main.rs file and build dependencies
RUN mkdir src && echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs

# Build dependencies so they can be cached for subsequent builds
RUN cargo build --release && rm -rf src

# Copy all source code
COPY . .

# 构建实际应用
RUN mv web/.env.template web/.env && cargo build --release

# Build the actual application
FROM debian:buster-slim

# Copy the build results and .env file
COPY --from=builder /app/target/release/animal_center /usr/bin/animal_center
COPY --from=builder /app/web/.env /usr/bin/.env

# Create upload directory
RUN mkdir /usr/bin/fileupload

# Expose port
EXPOSE 8080

# Set working directory
WORKDIR /usr/bin

# Start application
CMD [ "animal_center" ]
