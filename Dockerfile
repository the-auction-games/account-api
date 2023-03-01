FROM rust
WORKDIR /app
RUN cargo build --release
COPY target/release/rust-api /app
COPY Rocket.toml /app
ENV PORT=8000
EXPOSE $PORT
CMD ["/app/rust-api"]
