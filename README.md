#<div align="center">
  <img width="2500" alt="Quartz" src="https://cdn.prod.website-files.com/67504dd7fde047775f88c355/67b380029cf6f3d8e10349bf_docs_banner.jpg" />

  <h1 style="margin-top:20px;">Quartz Protocol</h1>
</div>

This indexer subscribed to all Quartz accounts and Quartz-owned Drift accounts for any state changes, caching them in Redis.

## Links

Website and waitlist: [quartzpay.io](https://quartzpay.io/)

Docs: [docs.quartzpay.io](https://docs.quartzpay.io/)

X: [@quartzpay](https://x.com/quartzpay)

Contact: [diego@quartzpay.io](mailto:diego@quartzpay.io)

## Prerequisites

- Rust and Cargo (latest stable version)
- Redis server running locally or accessible via network
- Access to a Solana RPC node with WebSocket support

## Configuration

The application uses environment variables for configuration:

- `RPC_URL`: Solana RPC endpoint (eg: https://api.mainnet-beta.solana.com)
- `REDIS_URL`: Redis connection URL (eg: redis://127.0.0.1/)

## Building

```bash
cargo build --release
```

## Running

1. Make sure Redis is running and accessible
2. Set environment variables if needed
3. Run the indexer:

```bash
cargo run --release
```

## Data Structure

The indexer stores user account data in Redis with the following structure:

- Key format: `user:drift:{pubkey}`
- Value: JSON serialized user account state
- TTL: 1 hour

## Logging

The application uses env_logger for logging. You can control the log level by setting the `RUST_LOG` environment variable:

```bash
RUST_LOG=info cargo run --release
``` 
