FROM rust:1.42-alpine

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .
RUN cp /usr/src/myapp/target/release/german-verben .

ENTRYPOINT ["./german-verben"]
