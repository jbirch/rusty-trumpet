FROM rust:1.69 as builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch

COPY src/ src/
RUN cargo build --release

FROM gcr.io/distroless/cc as runtime

COPY --from=builder /app/target/release/rusty-trumpet /
EXPOSE 8080
ENTRYPOINT ["/rusty-trumpet"]