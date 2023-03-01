FROM rust
WORKDIR /app
COPY target/debug/rust-api /app
COPY Rocket.toml /app
ENV PORT=8000
EXPOSE $PORT
CMD ["/app/rust-api"]
