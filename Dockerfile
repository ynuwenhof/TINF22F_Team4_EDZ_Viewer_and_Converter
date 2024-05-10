FROM rust:alpine AS builder
RUN apk add --no-cache musl-dev pkgconfig libarchive-dev

ENV PKG_CONFIG_PATH=/usr/lib/pkgconfig/ \
    RUSTFLAGS="-C target-feature=-crt-static"

WORKDIR /usr/src/assets
COPY ./assets .

WORKDIR /usr/src/rplan
COPY ./rplan .
RUN cargo install --path .

FROM alpine
COPY --from=builder /usr/local/cargo/bin/rplan /usr/local/bin/rplan

RUN apk add --no-cache libgcc libarchive-dev

RUN addgroup -g 1000 -S rplan && \
    adduser -u 1000 -G rplan -S rplan

RUN mkdir /data && \
    chown -R rplan:rplan /data

USER rplan

ENV RPLAN_ADDRESS=0.0.0.0 \
    RPLAN_PORT=80 \
    RPLAN_DATA_PATH=/data

EXPOSE 80

ENTRYPOINT ["rplan"]
