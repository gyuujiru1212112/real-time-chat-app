FROM rust:1.81

WORKDIR /usr/src/chatapp/
COPY pubsub ./pubsub

WORKDIR /usr/src/chatapp/pubsub
RUN cargo build --release

CMD ["./target/release/pubsub"]
