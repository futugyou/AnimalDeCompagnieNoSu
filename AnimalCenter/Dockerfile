FROM rust:1-slim-buster AS base
ENV PKG_CONFIG_ALLOW_CROSS=1
WORKDIR /app 
RUN USER=root cargo new animal_center
WORKDIR /app/animal_center

RUN apt-get update && apt-get dist-upgrade -y && apt-get install -y pkg-config libssl-dev
RUN mkdir fileupload	
COPY Cargo.toml Cargo.lock .env README.md ./
COPY entity/Cargo.toml ./entity/Cargo.toml
COPY infrastruct/Cargo.toml ./infrastruct/Cargo.toml
COPY model/Cargo.toml ./model/Cargo.toml
COPY repository/Cargo.toml ./repository/Cargo.toml
COPY serialize/Cargo.toml ./serialize/Cargo.toml
COPY service/Cargo.toml ./service/Cargo.toml
COPY tool/Cargo.toml ./tool/Cargo.toml
COPY web/Cargo.toml ./web/Cargo.toml

RUN for file in $(ls */Cargo.toml); do mkdir ./${file%/*}/src/ && touch ./${file%/*}/src/lib.rs ; done
RUN echo 'fn main() { println!("Dummy") }' > ./web/src/main.rs && rm -rf ./web/src/lib.rs

RUN cargo build --release
COPY . .

RUN cargo build --release --frozen
 
FROM rust:1-slim-buster
COPY --from=base /app/animal_center/target/release/animal_center /usr/bin/animal_center
EXPOSE 5000
ENTRYPOINT [ "/usr/bin/animal_center" ]