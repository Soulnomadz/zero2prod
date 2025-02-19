# Builder stage
FROM rust:1.84 as builder

WORKDIR /app
COPY sources.list /etc/apt/
RUN apt update && apt install lld clang -y
COPY . .
ENV SQLX_OFFLINE true
RUN echo export RUSTUP_DIST_SERVER="https://rsproxy.cn" >> ~/.bashrc
RUN bash -c "source ~/.bashrc"

RUN mkdir -p ~/.cargo \
  && echo > ~/.cargo/config '[source.crates-io]' \
  && echo > ~/.cargo/config "replace-with = 'rsproxy-sparse'" \
  && echo > ~/.cargo/config '[source.rsproxy]' \
  && echo > ~/.cargo/config 'registry = "https://rsproxy.cn/crates.io-index"' \
  && echo > ~/.cargo/config '[source.rsproxy-sparse]' \
  && echo > ~/.cargo/config 'registry = "sparse+https://rsproxy.cn/index/"' \
  && echo > ~/.cargo/config '[registries.rsproxy]' \
  && echo > ~/.cargo/config 'index = "https://rsproxy.cn/crates.io-index"' \
  && echo > ~/.cargo/config '[net]' \
  && echo > ~/.cargo/config 'git-fetch-with-cli = true'

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release

RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim AS runtime

WORKDIR /app
COPY sources.list /etc/apt/
#RUN apt-get update -y \
#  && apt-get install --no-install-recommends openssl ca-certificates -y \
#  && apt-get autoremove -y \
#  && apt-get clean -y \
#  && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/zero zero
COPY config config
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero"]
