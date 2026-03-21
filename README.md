![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# orqa-lsp-server

> **Pre-release** — APIs may change without notice until v1.0.0.

LSP server for real-time OrqaStudio artifact validation. Provides diagnostics directly in editors as you edit `.orqa/` markdown files, catching schema errors, broken relationships, and invalid status transitions before commit.

---

## Features

- Frontmatter schema validation against `core.json` artifact types
- Relationship verification — catches broken `depends_on`, `blocks`, and `related` references
- Bidirectional enforcement — detects one-sided relationships that should be mutual
- Status transition validation — flags invalid lifecycle progressions
- Standalone binary (`orqa-lsp-server`) for direct editor integration
- Also compiled as a library crate for embedding in the Tauri app
- Built on `tower-lsp` with stdio or `--tcp` transport

## Usage

### As a library

```toml
[dependencies]
orqa-lsp-server = { path = "libs/lsp-server" }
```

### Standalone binary

```bash
cargo build --release --bin orqa-lsp-server
./target/release/orqa-lsp-server          # stdio mode (default)
./target/release/orqa-lsp-server --tcp    # TCP mode
```

### Editor configuration (VS Code example)

```json
{
  "orqastudio.lsp.command": "orqa-lsp-server"
}
```

## Development

```bash
cargo build
cargo test
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
