FROM rust:alpine

RUN apk add openssl-libs-static pkgconf musl-dev openssl-dev
