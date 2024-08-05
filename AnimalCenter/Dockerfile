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

# Copy each crate's Cargo.toml file
COPY entity/Cargo.toml entity/Cargo.toml
COPY infrastruct/Cargo.toml infrastruct/Cargo.toml
COPY model/Cargo.toml model/Cargo.toml
COPY repository/Cargo.toml repository/Cargo.toml
COPY serialize/Cargo.toml serialize/Cargo.toml
COPY service/Cargo.toml service/Cargo.toml
COPY tool/Cargo.toml tool/Cargo.toml
COPY web/Cargo.toml web/Cargo.toml

# Define a list of member crates
ARG MEMBERS="entity infrastruct model repository serialize service tool web"

# Copy each crate's Cargo.toml file and create a temporary src directory and lib.rs file for each member crate
RUN for member in $MEMBERS; do \
    mkdir -p $member/src; \
    echo "fn main() {}" > $member/src/lib.rs; \
    done

# Build dependencies so they can be cached for subsequent builds
RUN cargo build --release

# Delete the temporary src directory
RUN for member in $MEMBERS; do \
    rm -rf $member/src; \
    done

# Copy all source code
COPY . .

# Build the actual application
RUN mv .env.template .env && cargo build --release

# Use a smaller base image to run the application
FROM debian:buster-slim

# Copy the build results and .env file
COPY --from=builder /app/target/release/animal_center /usr/bin/animal_center
COPY --from=builder /app/.env /usr/bin/.env

# Create upload directory
RUN mkdir /usr/bin/fileupload

# Expose port
EXPOSE 8080

# Set working directory
WORKDIR /usr/bin

# Start application
CMD [ "animal_center" ]
