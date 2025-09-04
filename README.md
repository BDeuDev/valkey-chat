# 📖 Valkey Chat with Parquet

A Rust-based chat system that combines:

- 📦 **Valkey (Redis-compatible)** → in-memory storage for recent chat messages.  
- 📊 **Parquet** → efficient columnar storage for historical data and analytics.  
- ⚡ **Actix-Web** → HTTP API to interact with the chat service.  


## 🚀 Features

- 📩 **Store chat messages** by room (`/message`)  
- 💾 **Export messages** from Valkey to a Parquet file (`/export`)  
- 📜 **Query historical data** by reading from Parquet and returning JSON (`/history`)  
- ⚡ **High-performance APIs** built with Actix-Web  
- 🔒 **Secure serialization** with Serde + JSON  


## 📂 Project Structure
```bash
valkey-chat/
├── src/                   
│   ├── config/                      # Configuration modules
│   │   ├── s3.rs                    # S3/MinIO config and client
│   │   └── valkey.rs                # Valkey (Redis) config and client
│   ├── controllers/                 # HTTP controllers (Actix handlers)
│   │   ├── export.rs                # Export messages to Parquet/S3
│   │   ├── history.rs               # Handle history retrieval endpoints
│   │   ├── messages.rs              # Handle message endpoints (save/fetch)
│   │   └── types.rs                 # Shared request/response types
│   ├── models/                      # Data models
│   │   └── chat_message.rs          # ChatMessage struct definition
│   ├── services/                    # Business logic
│   │   ├── export.rs                # Export service (Parquet + S3)
│   │   ├── message.rs               # Message service (Valkey ops)
│   │   └── read_history.rs          # Service for reading Parquet history
│   ├── storage/                     # Low-level storage layer
│   │    └── parquet.rs               # Parquet writer/reader implementation
│   ├── config.rs                    # Storage-related configs
│   ├── controllers.rs               # Storage-specific handlers (legacy/misc)
│   ├── main.rs                      # Entry point, Actix server setup
│   ├── models.rs                    # Storage-related models
│   ├── routes.rs                    # Route definitions for storage endpoints
│   ├── services.rs                  # Storage service implementations
│   └── storage.rs                   # Entry for storage module exports
├── .env                             # Environment variables (local dev)
├── .gitignore                       # Git ignored files
├── Cargo.lock                       # Cargo lock file
├── Cargo.toml                       # Project dependencies & metadata
├── docker-compose.yml               # Docker services (Valkey, MinIO, App)
├── Dockerfile                       # Container build for the app
├── entrypoint.sh                    # Script for container startup
└── README.md                        # Project documentation
```

## ⚙️ Installation

### Prerequisites

- **Rust**
- **Docker & Docker Compose**
- **Optional: MinIO or AWS S3 for Parquet storage**
### Clone & Run

```bash
git clone https://github.com/BDeuDev/valkey-chat
cd valkey-chat

# Run with Docker Compose
docker-compose up --build
```
The API will be available at:
👉 http://localhost:8080


## 🔑 Configuration

Set environment variables in .env:

```bash
# Valkey (Redis-compatible)
VALKEY_HOST=127.0.0.1
VALKEY_PORT=6379

# S3 / MinIO
S3_ENDPOINT=http://172.19.0.2:9000
S3_BUCKET=valkey-chat
S3_REGION=us-east-1
S3_ACCESS_KEY=minio
S3_SECRET_KEY=minio123
```