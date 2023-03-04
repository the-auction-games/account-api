# Builder to compile the code
FROM rust as builder
COPY . .
RUN cargo build --release

# Final image to run the compiled code
FROM rust
WORKDIR /app
COPY --from=builder ./target/release/account-api /app
COPY Rocket.toml /app

ENV APP_PORT=8000
ENV SIDECAR_PORT=3500
ENV STATE_STORE_NAME="account-db"

EXPOSE $PORT
CMD ["ROCKET_PORT=${APP_PORT}", "/app/account-api"]
