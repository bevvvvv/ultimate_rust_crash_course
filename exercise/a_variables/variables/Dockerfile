FROM rust:1-bullseye

WORKDIR /app

COPY ./src /app/src
COPY ./Cargo.* /app/

RUN cargo build

CMD cargo run
