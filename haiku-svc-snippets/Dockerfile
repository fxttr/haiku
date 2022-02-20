FROM rust:1.58.1 as build
ENV PKG_CONFIG_ALLOW_CROSS=1

WORKDIR /usr/src/haiku-svc-snippets
COPY . .

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

COPY --from=build /usr/local/cargo/bin/haiku-svc-snippets /usr/local/bin/haiku-svc-snippets

CMD ["haiku-svc-snippets"]