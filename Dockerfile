FROM ekidd/rust-musl-builder:stable as builder
RUN USER=root cargo new --bin ens-rest-server
WORKDIR /home/rust/src/ens-rest-server
COPY ./ens-rest-server/ ./
RUN cargo build --release

FROM alpine:latest
EXPOSE 8000
ENV TZ=Etc/UTC \
    LOG_LEVEL=info,ureq=warn \
    RUST_BACKTRACE=full
COPY --from=builder /home/rust/src/ens-rest-server/target/x86_64-unknown-linux-musl/release/ens-rest-server /usr/src/app/ens-rest-server
WORKDIR /usr/src/app
ENTRYPOINT ["/usr/src/app/ens-rest-server"]

