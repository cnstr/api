FROM rust:1.66 as builder
WORKDIR /app

ADD . .
RUN cargo prisma generate
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/api /
CMD ["./api"]

EXPOSE 3000
ARG BUILD_DATE
ARG GIT_STATE
ARG VERSION

LABEL org.opencontainers.image.created=${BUILD_DATE}
LABEL org.opencontainers.image.authors="Aarnav Tale <aarnav@tale.me>"
LABEL org.opencontainers.image.source="https://github.com/cnstr/api"
LABEL org.opencontainers.image.version=${VERSION}
LABEL org.opencontainers.image.revision=${GIT_STATE}
LABEL org.opencontainers.image.vendor="Aarnav Tale"
LABEL org.opencontainers.image.licenses="Unlicensed"
LABEL org.opencontainers.image.ref.name="tale.me/canister/api"
LABEL org.opencontainers.image.title="Canister API"
LABEL org.opencontainers.image.description="The frontend API for Canister."
LABEL org.opencontainers.image.base.name="gcr.io/distroless/cc"