# Builder to compile the code
FROM rust as builder
WORKDIR /build
COPY . .
RUN cargo build --release

# Final image to run the compiled code
FROM rust
WORKDIR /app
COPY --from=builder ./build/target/release/account-api .

# Environment variables
ENV APP_PORT=8000
ENV SIDECAR_PORT=3500
ENV STATE_STORE_NAME="account-db"

# Rocket environment variables
ENV ROCKET_ADDRESS=0.0.0.0
#ROCKET_PORT is set at runtime

# Expose the port
EXPOSE ${APP_PORT}

# Run the binary
CMD ROCKET_PORT=${APP_PORT} ./account-api
