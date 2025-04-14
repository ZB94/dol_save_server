FROM clux/muslrust:stable AS build-env
COPY ./ /
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.21

COPY ./dol_save_server_docker.toml /dol_save_server.toml
COPY --from=build-env /target/x86_64-unknown-linux-musl/release/dol_save_server /
EXPOSE 5000

VOLUME [ "/save" ]

CMD ["./dol_save_server"]
