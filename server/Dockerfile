FROM rust:1.81

WORKDIR /usr/src/chatapp/
COPY server ./server

WORKDIR /usr/src/chatapp/server
RUN cargo build --release

CMD ["./target/release/server"]
