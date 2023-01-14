FROM rust:1.65.0-slim-bullseye as builder

WORKDIR /usr/src/lab14
COPY . .

RUN cargo install --path .

EXPOSE 80:3000

CMD ["lab14"]