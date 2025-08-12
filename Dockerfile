FROM rust:latest
RUN mkdir /app
COPY . /app/.

WORKDIR /app

RUN cargo build --release

ENTRYPOINT ["target/release/jj-mcp-server"]
