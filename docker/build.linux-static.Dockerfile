FROM rust:alpine

RUN apk add --no-cache openssl-libs-static pkgconf musl-dev openssl-dev
