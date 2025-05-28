FROM clux/muslrust:stable AS build-env
COPY ./ .
RUN apt-get update && apt-get install -y tzdata
RUN target="$(uname -m)-unknown-linux-musl" && \
    cargo build --release --target $target && \
    cp target/$target/release/dol_save_server /

FROM scratch

WORKDIR /
COPY ./dol_save_server_docker.toml /dol_save_server.toml
COPY --from=build-env /usr/share/zoneinfo /usr/share/zoneinfo
COPY --from=build-env /dol_save_server /
EXPOSE 5000

VOLUME [ "/save", "/backup" ]

CMD ["./dol_save_server"]
