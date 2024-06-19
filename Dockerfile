FROM --platform=$BUILDPLATFORM rust:1.66 as builder
WORKDIR /app

RUN apt-get update \
&& apt-get install -y --no-install-recommends \
	g++-aarch64-linux-gnu libc6-dev-arm64-cross \
	&& rustup target add aarch64-unknown-linux-gnu

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc

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

LABEL org.opencontainers.image.created=${BUILD_DATE}
LABEL org.opencontainers.image.authors="Aarnav Tale <aarnavtale@icloud.com>"
LABEL org.opencontainers.image.source="https://github.com/cnstr/api"
LABEL org.opencontainers.image.version=${VERSION}
LABEL org.opencontainers.image.revision=${GIT_STATE}
LABEL org.opencontainers.image.vendor="Aarnav Tale"
LABEL org.opencontainers.image.licenses="Unlicensed"
LABEL org.opencontainers.image.ref.name="us-east4-docker.pkg.dev/aarnavtale/canister/api"
LABEL org.opencontainers.image.title="Canister API"
LABEL org.opencontainers.image.description="The frontend API for Canister."
LABEL org.opencontainers.image.base.name="gcr.io/distroless/cc"
