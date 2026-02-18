# Rust Advanced Learning

Learning project from the online course [Advanced Rust: Building Reusable and Production-Ready Web APIs](https://www.udemy.com/course/advanced-rust-web-apis/).

REST API with **Rocket**, **Diesel** (PostgreSQL), and **Redis**. Endpoints: rustaceans & crates (CRUD).

## Running with Docker Compose

```bash
docker compose up --build
```

API runs at **http://localhost:8000**. Postgres and Redis are started via compose; DB migrations run automatically when the app container starts.

---

**Student:** [Agung Yuliyanto](https://github.com/agung96tm)
