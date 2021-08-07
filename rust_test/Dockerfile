FROM rust as builder

RUN USER=root cargo new --bin rust_test
WORKDIR ./rust_test
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN apt-get update -y && apt-get install sqlite3 -y
RUN sqlite3 db/sqlite3.db < db/schema.sql
# ENV DATABASE_URL="sqlite://.db/sqlite.db"
# RUN export TIDE_SECRET="$(openssl rand -base64 64)"
# RUN echo "export DATABASE_URL=\"sqlite://./db/sqlite3.db\"\nexport TIDE_SECRET=\"$(openssl rand -base64 64)\"" > .envrc
# RUN source .envrc

RUN rm ./target/release/deps/rust_test*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update && apt-get install -y sqlite3 openssl ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC APP_USER=appuser

RUN groupadd $APP_USER && useradd -g $APP_USER $APP_USER && mkdir -p ${APP}

COPY --from=builder /rust_test/target/release/rust_test ${APP}/rust_test
COPY --from=builder /rust_test/db/sqlite3.db ${APP}/sqlite3.db

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}
ENV DATABASE_URL="sqlite://./sqlite.db"
ENV TIDE_SECRET="0"

CMD ["./rust_test"]