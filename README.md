# 📖 Valkey Chat with Parquet

A Rust-based chat system that combines:

Valkey (Redis-compatible) → in-memory storage for recent chat messages.

Parquet → efficient columnar storage for historical data and analytics.

Actix-Web → HTTP API to interact with the chat service.

This project demonstrates how to integrate fast in-memory caching with analytical persistence in Rust.

## 🚀 Features

📩 Store chat messages by room (/message).

💾 Export messages from Valkey to a Parquet file (/export).

📜 Query historical data by reading from Parquet and returning JSON (/history).

⚡ Built with Actix-Web for high-performance APIs.

🔒 Secure serialization with Serde + JSON.

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
│   |   └── parquet.rs               # Parquet writer/reader implementation
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