FROM rust:slim-bookworm as builder

RUN apt-get update \ 
   && apt-get install -y libpq-dev libssl-dev pkg-config

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim as runtime 

ARG BIN_DIR=/local
ARG BINARY=rust-server

RUN apt-get update \
  && apt-get install -y ca-certificates tzdata libpq-dev curl procps

EXPOSE 3000 

COPY --from=builder /app/target/release/${BINARY} ${BIN_DIR}/${BINARY}

WORKDIR ${BIN_DIR}

CMD ./rust-server

#  docker build -t chethan01/rust-server .
#  docker image push chethan01/rust-server