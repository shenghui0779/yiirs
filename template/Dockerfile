FROM rust:1.84 as builder

WORKDIR /project

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /bin

# Note: Some shared libraries may need to install the extra-runtime-dependencies.
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /project/target/release/main .

EXPOSE 8000

ENTRYPOINT ["./main"]

CMD ["serve", "--config", "/data/config.toml"]
