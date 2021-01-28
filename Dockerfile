FROM rust:1.49 as builder

RUN USER=root cargo new --bin participant-crd
WORKDIR ./participant-crd
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/participant_crd*
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y libssl-dev
#     ca-certificates tzdata \
#     && rm -rf /var/lib/apt/lists/*

EXPOSE 80

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /participant-crd/target/release/participant-crd ${APP}/participant-crd

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./participant-crd"]
