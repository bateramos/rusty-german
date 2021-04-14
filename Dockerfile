FROM rust:1.51

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get install pkg-config libssl-dev
RUN cargo install --path .
RUN cp /usr/src/myapp/target/release/rusty-german .

ENTRYPOINT ["./rusty-german"]
