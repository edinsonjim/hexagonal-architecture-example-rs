# Build stage
FROM rust:1.83-slim-bullseye as builder

WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./crates ./crates
COPY ./portal_migration ./portal_migration
COPY ./portal_schema ./portal_schema

RUN cargo build --locked --release
RUN cp ./target/release/portal_service_rs /bin/portal_services

# Deploy stage
FROM debian:bullseye-slim as final

COPY --from=builder /bin/portal_services /bin/portal_services

EXPOSE 8080

CMD [ "./bin/portal_services" ]
