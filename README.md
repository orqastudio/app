![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# orqa-search

> **Pre-release** — APIs may change without notice until v1.0.0.

ONNX-based semantic search engine for OrqaStudio. Provides BGE-small-en-v1.5 embeddings stored in DuckDB, with support for regex search, semantic search, and combined research queries across artifact and codebase scopes.

---

## Features

- BGE-small-en-v1.5 embeddings via ONNX Runtime (`ort`)
- DuckDB storage for vector indices and full-text search
- Three search modes: `search_regex`, `search_semantic`, `search_research`
- Scope parameter: `artifacts`, `codebase`, or `all`
- Standalone binary (`orqa-search-server`) for out-of-process use
- Compiled into the OrqaStudio Tauri app as a library crate

## Usage

### As a library

```toml
[dependencies]
orqa-search = { path = "libs/search" }
```

### Standalone binary

```bash
cargo build --release --bin orqa-search-server
./target/release/orqa-search-server
```

The server listens on stdio and accepts JSON-RPC requests for indexing and querying.

## Development

```bash
cargo build
cargo test
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
