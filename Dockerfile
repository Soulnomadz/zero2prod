FROM rust:1.84
WORKDIR /app
COPY sources.list /etc/apt/
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release
ENTRYPOINT ["./target/release/zero"]
