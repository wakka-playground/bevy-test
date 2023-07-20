FROM rust:1.71

RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

