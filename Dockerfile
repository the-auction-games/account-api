# Builder to compile the code
FROM rust as builder
COPY . .
RUN cargo build --release

# Final image to run the compiled code
FROM rust
WORKDIR /app
COPY --from=builder ./target/release/account-api /app
COPY Rocket.toml /app
ENV PORT=8000
EXPOSE $PORT
CMD ["/app/account-api"]
