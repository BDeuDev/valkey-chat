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
│   ├── main.rs                     # Actix server configuration
│   ├── storage/
│   │   ├── types.rs                # ChatMessage struct definition
│   │   ├── save_message.rs         # Save message into Valkey
│   │   ├── fetch_messages.rs       # Fetch messages from Valkey
│   │   ├── export_to_parquet.rs    # Export messages to Parquet
│   │   ├── read_parquet_history.rs # Read history from Parquet
│   └── endpoints.rs                # Actix handlers (message, export, history)
├── Cargo.toml
└── README.md

```