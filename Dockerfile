# Etapa 1: Build
FROM rust:1.86 as builder

WORKDIR /app

# Instalar dependencias para compilar
RUN apt-get update && apt-get install -y libpq-dev pkg-config libssl-dev
# Copiar archivos necesarios
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Compilar binario de la app
RUN cargo build --locked

# Etapa 2: Runtime
FROM debian:bookworm-slim

# Instalar solo runtime deps
RUN apt-get update && apt-get install -y libpq5 ca-certificates netcat-openbsd && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado
COPY --from=builder /app/target/debug/valkey-chat .
COPY entrypoint.sh .

RUN chmod +x entrypoint.sh

EXPOSE 8080

CMD ["./entrypoint.sh"]