FROM rust:1.59 as base

WORKDIR /usr/src/app
COPY . .

RUN rustup component add rustfmt

##############################

FROM base as client

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --bin client --path .
CMD client

##############################

FROM base as server

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --bin server --path .
CMD server