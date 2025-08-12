FROM rust:latest

# Install jj (Jujutsu VCS)
RUN cargo install --git https://github.com/martinvonz/jj jj-cli

RUN mkdir /app
COPY . /app/.

WORKDIR /app

RUN cargo build --release

ENTRYPOINT ["target/release/jj-mcp-server"]
