FROM rust:1.34

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

RUN cp /usr/local/cargo/bin/* /usr/src/app
