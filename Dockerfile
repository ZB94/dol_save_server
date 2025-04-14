FROM clux/muslrust:stable AS build-env
COPY ./ /
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.21
COPY ./img /img
COPY ./pwa /pwa
COPY ./dol_save_server.toml /dol_save_server.toml
COPY ["./Degrees of Lewdity.html", "/Degrees of Lewdity.html"]
COPY --from=build-env /target/x86_64-unknown-linux-musl/release/dol_save_server /
EXPOSE 5000
CMD ["./dol_save_server"]
