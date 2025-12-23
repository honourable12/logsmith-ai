# logsmith-ai

A high-performance Rust backend pipeline that ingests structured logs, processes them concurrently, and produces time-windowed features suitable for AI/ML training and monitoring.

This project is intentionally designed to demonstrate **backend engineering fundamentals**, **Rust concurrency**, and **AI systems awareness** at an entry–junior level.

---

## 🚀 Why This Project Exists

Modern AI systems depend on reliable, high-throughput data ingestion pipelines. Raw logs must be:

* Parsed safely
* Processed concurrently
* Aggregated into fixed-size feature windows
* Exported in ML-friendly formats

`logsmith-ai` implements this end-to-end in **safe, idiomatic Rust**.

---

## ✨ Features

* 📥 **Streaming ingestion** of large log files
* 🧵 **Multi-threaded parsing** using message-passing (no shared mutable state)
* ⏱️ **Time-windowed feature aggregation**
* 📤 **CSV/Parquet export** for ML training pipelines
* ⚡ **Batch parallelism** using Rayon (offline workloads)
* 📊 **Benchmarks** using Criterion

---

## 🏗️ Architecture Overview

```
file → reader thread → channel → N worker threads → channel → windowed aggregator → CSV
```

### Concurrency Model

* Ownership is transferred through channels
* No `Arc<Mutex<T>>`
* Channels provide backpressure and graceful shutdown

This mirrors real production ingestion pipelines.

---

## 📦 Project Structure

```
logsmith-ai/
├── src/
│   ├── ingest/        # File reading & parsing
│   ├── pipeline/      # Channel & Rayon pipelines
│   ├── features/      # Feature extraction & windowing
│   ├── output/        # CSV export
│   └── main.rs
├── benches/           # Criterion benchmarks
└── tests/             # End-to-end tests
```

---

## 🧪 Example Input

Each line is a JSON log event:

```json
{"timestamp":"2025-11-08T10:00:00Z","level":"INFO","latency_ms":120}
```

---

## 📊 Example Output (CSV)

```csv
window_start,window_end,request_count,avg_latency
2025-11-08T10:00:00Z,2025-11-08T10:05:00Z,842,124.6
```

This file can be loaded directly into Pandas, Spark, or ML training pipelines.

---

## ⚡ Streaming vs Batch Processing

| Use Case             | Approach           |
| -------------------- | ------------------ |
| Real-time ingestion  | Channels + threads |
| Offline dataset prep | Rayon              |

The project includes both implementations and benchmarks comparing them.

---

## 📊 Benchmarks

Benchmarks are implemented using **Criterion**:

```bash
cargo bench
```

They compare:

* Channel-based streaming parsing
* Rayon-based batch parsing

---

## 🧠 What This Demonstrates

* Rust ownership & lifetimes by design
* Safe concurrency without locks
* Backend data pipeline architecture
* AI/ML feature engineering awareness
* Performance measurement, not guesswork

---

## 🛠️ Tech Stack

* Rust
* clap
* serde / serde_json
* chrono
* csv
* rayon
* criterion
* crossbeam-channel
* log
* env_logger
* arrow


---

## 📌 Future Improvements

* Parquet export
* Prometheus metrics
* Async I/O
* Sliding windows

