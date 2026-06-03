FROM rust:1.88 AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/domain-router /usr/local/bin/domain-router

COPY --from=builder /app/pkg /app/pkg

EXPOSE 8080

CMD ["domain-router"]
