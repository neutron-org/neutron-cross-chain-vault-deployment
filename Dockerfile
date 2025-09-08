FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    curl build-essential pkg-config libssl-dev git ca-certificates unzip \
 && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install just

WORKDIR /app

COPY . .

RUN ls -1 strategies | xargs -I "{}" cargo build -p {}_strategist --bin runner

ENTRYPOINT ["just"]