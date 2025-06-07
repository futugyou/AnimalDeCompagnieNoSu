FROM rust:1.87-slim-bullseye AS base
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /app/animal_center
RUN apt-get update && apt-get dist-upgrade -y && apt-get install -y pkg-config libssl-dev
COPY . .
RUN mv .env.temp .env
RUN cargo build --release

FROM rust:1.87-slim-bullseye
COPY --from=base /app/animal_center/target/release/animal_center /usr/bin/animal_center
COPY --from=base /app/animal_center/.env /usr/bin/.env
RUN mkdir ./usr/bin/fileupload
EXPOSE 8080
WORKDIR /usr/bin
CMD [ "animal_center" ]
