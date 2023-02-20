FROM rust:1.67 as builder

WORKDIR /api_tpl

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /bin

# Note: Some shared libraries may need to install the extra-runtime-dependencies.
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

COPY --from=builder /api_tpl/target/release/api_tpl .

EXPOSE 8000

ENTRYPOINT ["./api_tpl"]

CMD ["/data/.env"]
