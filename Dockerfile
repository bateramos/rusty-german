FROM rust:1.52.0 AS build
WORKDIR /usr/src

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

#RUN apt-get update
#RUN apt-get install pkg-config libssl-dev libc6-dev musl-gcc

RUN USER=root cargo new rusty-german
WORKDIR /usr/src/rusty-german
COPY Cargo.toml Cargo.lock ./

RUN cargo build --features vendored --release

COPY src ./src
COPY data ./data

RUN apt-get update
RUN apt-get install -y musl-tools
RUN cargo install --features vendored --target x86_64-unknown-linux-musl --path .
RUN touch storage.txt
RUN chmod 666 storage.txt

FROM scratch
COPY --from=build /usr/local/cargo/bin/rusty-german .
COPY --from=build /usr/src/rusty-german/data data
COPY --from=build /usr/src/rusty-german/storage.txt storage.txt
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
USER 1000
CMD ["./rusty-german"]
