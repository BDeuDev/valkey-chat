# 📖 Valkey Chat with Parquet


A Rust-based chat system that combines:

- 📦 **Valkey (Redis-compatible)** → in-memory storage for recent chat messages.  
- 📊 **Parquet** → efficient columnar storage for historical data and analytics.  
- ⚡ **Actix-Web** → HTTP API to interact with the chat service.  
- 🔔 **Event-driven architecture** → workers and triggers reacting to Valkey Pub/Sub events.  

---

## 🚀 Features

- 📩 **Store chat messages** by room (`/message`)  
- 💾 **Export messages** from Valkey to a Parquet file (`/export`)  
- 📜 **Query historical data** by reading from Parquet and returning JSON (`/history`)  
- 🔔 **Event triggers** for custom workflows using Valkey Pub/Sub  
- 👷 **Background worker** that listens for events and executes tasks asynchronously  
- ⚡ **High-performance APIs** built with Actix-Web  
- 🔒 **Secure serialization** with Serde + JSON  

---

## ⚙️ Event-Driven Workflow

This project is not only a request/response API — it also reacts to **real-time events**:

- 📨 When a new message is pushed into Valkey, a **Pub/Sub channel** emits an event.  
- 👂 The **`pubsub_worker`** listens for these events continuously.  
- 🚀 The **`trigger_service`** processes the event and can:  
  - Start a background export to Parquet.  
  - Upload to S3

This allows **automation** without requiring manual HTTP calls, making the system reactive and scalable.  

---

## 📂 Project Structure
```bash
valkey-chat/
├── src/                   
│   ├── config/                      # Configuration modules
│   │   ├── s3.rs                    # S3/MinIO config and client
│   │   └── valkey.rs                # Valkey (Redis) config and client
│   ├── controllers/                 # HTTP controllers (Actix handlers)
│   │   ├── export_controller.rs     # Export messages to Parquet/S3
│   │   ├── history_controller.rs    # Handle history retrieval endpoints
│   │   ├── messages_controller.rs   # Handle message endpoints (save/fetch)
│   │   └── types.rs                 # Shared request/response types
│   ├── models/                      # Data models
│   │   └── chat_message.rs          # ChatMessage struct definition
│   ├── services/                    # Business logic
│   │   ├── export_service.rs        # Export service (Parquet + S3)
│   │   ├── history_service.rs       # Service for reading Parquet history (S3)
│   │   ├── message_service.rs       # Message service (Valkey ops)
│   │   └── trigger_service.rs       # Event triggers (hooks/pubsub integration)
│   ├── storage/                     # Low-level storage layer
│   │   └── parquet.rs               # Parquet writer/reader implementation
│   ├── workers/                     # Background async tasks
│   │   └── pubsub_worker.rs         # Worker for Valkey Pub/Sub events
│   ├── config.rs                    # App-wide configs (entry point for config/)
│   ├── controllers.rs               # Legacy/misc controller aggregator
│   ├── main.rs                      # Entry point, Actix server setup
│   ├── models.rs                    # Legacy/misc model aggregator
│   ├── routes.rs                    # Route definitions for APIs
│   ├── services.rs                  # Legacy/misc service aggregator
│   ├── storage.rs                   # Entry point for storage module exports
│   └── workers.rs                   # Worker module exports
├── .env                             # Environment variables (local dev)
├── .gitignore                       # Git ignored files
├── Cargo.lock                       # Cargo lock file
├── Cargo.toml                       # Project dependencies & metadata
├── docker-compose.yml               # Docker services (Valkey, MinIO, App)
├── Dockerfile                       # Container build for the app
├── entrypoint.sh                    # Script for container startup
└── README.md                        # Project documentation
```
## 🔑 Configuration

Set environment variables in .env:

```bash
# Valkey (Redis-compatible)
VALKEY_HOST=valkey
VALKEY_PORT=6379

# S3 / MinIO
S3_ENDPOINT=http://minio:9000
S3_BUCKET=valkey-chat
S3_REGION=us-east-1
S3_ACCESS_KEY=minio
S3_SECRET_KEY=minio123
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

## 📡 API Endpoints

### 1. Store Message

```bash
POST api/v1/message
Content-Type: application/json

{
  "room": "general",
  "user": "alice",
  "text": "Hello World!"
}
```
### 2. Obtain 100 recent messages

```bash
GET api/v1/messages?room={room_name}
```
Get all recent messages by room from Valkey

### 3. Export to Parquet

```bash
GET api/v1/export?room={room_name}
```
Exports all messages by room from Valkey to Parquet (locally or S3).

## 🏗️ Architecture Overview

```text
                 ┌───────────────────────┐
                 │       Client App      │
                 │   (HTTP Requests)     │
                 └───────────┬───────────┘
                             │
                             ▼
                 ┌───────────────────────┐
                 │     Actix-Web API     │
                 │  controllers/services │
                 └───────────┬───────────┘
                             │
        ┌────────────────────┼───────────────────────┐
        │                    │                       │
        ▼                    ▼                       ▼
┌─────────────┐     ┌──────────────────┐     ┌───────────────────┐
│  Valkey DB  │     │ PubSub Worker    │     │  Export Service   │
│ (in-memory) │◄──► │ (async listener) │     │ (Parquet writer)  │
└──────┬──────┘     └─────────┬────────┘     └─────────┬─────────┘
       │                      │                        │
       ▼                      ▼                        ▼
┌───────────────┐     ┌──────────────────┐     ┌──────────────────┐
│  Message Ops  │     │ Trigger Service  │     │   Parquet Files  │
│ (save/fetch)  │     │ (react to events)│     │  (local or S3)   │
└───────────────┘     └──────────────────┘     └──────────────────┘
