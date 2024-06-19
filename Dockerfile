FROM rust:1.66 as builder
WORKDIR /app

ADD . .
RUN cargo prisma generate
RUN cargo build --release -p api

FROM gcr.io/distroless/cc-debian11
COPY --from=builder /app/target/release/api /
CMD ["./api"]

EXPOSE 3000
ARG BUILD_DATE
ARG GIT_STATE
ARG VERSION

