FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

ENTRYPOINT [ "./target/release/nodemore" ]

CMD []
