FROM rust:1.79 as builder
ENV UPLOAD_OPENAPI=true
WORKDIR /app

ADD . .
RUN cargo build --release -p api

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /app/target/release/api /
CMD ["./api"]

EXPOSE 3000
