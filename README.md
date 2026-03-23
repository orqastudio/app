![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# Validation

Schema-driven integrity engine for the OrqaStudio artifact graph — validates artifact frontmatter against JSON Schemas defined by installed plugins, enforces filename-to-ID matching, and checks relationship integrity.

## Usage

The validation library is consumed by the OrqaStudio CLI and git hooks plugin. It runs as a sidecar binary during `orqa enforce` and the pre-commit hook.

```bash
orqa enforce
```

## Development

```bash
cargo build
cargo test
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
