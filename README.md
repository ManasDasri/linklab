# LinkLab

**Short links. Fast redirects. Built in Rust.**

[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Axum](https://img.shields.io/badge/Axum-HTTP%20framework-blue)](https://github.com/tokio-rs/axum)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Status: Shipping in phases](https://img.shields.io/badge/status-shipping%20in%20phases-orange)

LinkLab is a URL shortener built for speed and simplicity. The goal is a small,
fast service that turns long URLs into short links and gets visitors to their
destination quickly.

I'm building and shipping it in phases: a focused core first, then link creation,
persistence, and measured performance improvements.

## Current status

**Phase 0 — repository foundations.** This first commit contains the README,
MIT license, and Rust `.gitignore`. The Rust service will ship in a later phase.

## Stack

- **Rust** — the language behind the service.
- **Axum** — HTTP routing and request handling.
- **Tokio** — the asynchronous runtime.

## Phased roadmap

The roadmap is a starting point and may evolve as the project takes shape.

| Phase | Focus | Status |
| --- | --- | --- |
| 0 | Repository setup, documentation, and licensing | Shipped |
| 1 | Read-only redirects, short-code lookups, and a health endpoint | In progress |
| 2 | Link creation and destination URL validation | Planned |
| 3 | Persistent storage so links survive restarts | Planned |
| 4 | Benchmarks, performance tuning, and deployment readiness | Planned |

Usage instructions will arrive with the first code release. Performance numbers
will be published alongside reproducible benchmarks.

## Follow along

Watch the repository for each phase. Ideas and feedback are welcome in
[Issues](https://github.com/ManasDasri/linklab/issues).

## License

Licensed under the [MIT License](LICENSE).
