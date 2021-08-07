FROM rust:latest as builder

RUN USER=root cargo new --bin rust_test
WORKDIR ./rust_test
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN apt-get update -y && apt-get install sqlite3 -y
RUN sqlite3 db/sqlite3.db < db/schema.sql
RUN echo "export DATABASE_URL=\"sqlite://./db/sqlite3.db\"\nexport TIDE_SECRET=\"$(openssl rand -base64 64)\"" > .envrc
RUN bash .envrc

RUN rm ./target/release/deps/rust_test*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update && apt-get install -y sqlite3 ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC APP_USER=appuser

RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir -p ${APP}

COPY --from=builder /rust_test/target/release/rust_test ${APP}/rust_test

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust_test"]