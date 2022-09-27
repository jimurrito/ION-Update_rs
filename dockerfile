# Docker Container for Ion-Update
# sudo docker build -t jimurrito/ionupdate_rs:latest .

FROM rust:slim-buster

ENV VER=1.09.26

ARG LOG_LEVEL=info
ENV LOG_LEVEL=${LOG_LEVEL}
ARG SCOPE=unset
ENV SCOPE=${SCOPE}
ARG PUBKEY
ENV PUBKEY=${PUBKEY}
ARG PRVKEY
ENV PRVKEY=${PRVKEY}

USER root
RUN mkdir /app
ADD src/ /app/src/
ADD Cargo.lock /app/.
ADD Cargo.toml /app/.
RUN apt update && apt install --upgrade && apt install pkg-config openssl libssl-dev -y
RUN cd /app && cargo build -r

WORKDIR /app

CMD cargo run -r