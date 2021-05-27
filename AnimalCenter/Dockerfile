FROM rust:1-slim-buster AS base

WORKDIR /app

RUN USER=root cargo new animal_center
WORKDIR /app/animal_center

COPY Cargo.toml Cargo.lock .env README.md ./

COPY entity/Cargo.toml ./entity/Cargo.toml
COPY infrastruct/Cargo.toml ./infrastruct/Cargo.toml
COPY model/Cargo.toml ./model/Cargo.toml
COPY repository/Cargo.toml ./repository/Cargo.toml
COPY serialize/Cargo.toml ./serialize/Cargo.toml
COPY service/Cargo.toml ./service/Cargo.toml
COPY tool/Cargo.toml ./tool/Cargo.toml
COPY web/Cargo.toml ./web/Cargo.toml

RUN mkdir ./entity/src
RUN mkdir ./infrastruct/src
RUN mkdir ./model/src
RUN mkdir ./repository/src
RUN mkdir ./serialize/src
RUN mkdir ./service/src
RUN mkdir ./tool/src
RUN mkdir ./web/src

RUN echo '' > ./entity/src/lib.rs
RUN echo '' > ./infrastruct/src/lib.rs
RUN echo '' > ./model/src/lib.rs
RUN echo '' > ./repository/src/lib.rs
RUN echo '' > ./serialize/src/lib.rs
RUN echo '' > ./service/src/lib.rs
RUN echo '' > ./tool/src/lib.rs
RUN echo 'fn main() { println!("Dummy") }' > ./web/src/main.rs

RUN cargo build --release

COPY entity/src ./entity/src
COPY infrastruct/src ./infrastruct/src
COPY model/src ./model/src
COPY repository/src ./repository/src
COPY serialize/src ./serialize/src
COPY service/src ./service/src
COPY tool/src ./tool/src
COPY web/src ./web/src

RUN cargo build --release --frozen
 
FROM rust:1-slim-buster

COPY --from=builder /animal_center/target/release/animal_center /usr/bin/animal_center

EXPOSE 5000

ENTRYPOINT [ "/usr/bin/animal_center" ]