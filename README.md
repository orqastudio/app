![License](https://img.shields.io/badge/license-BSL%201.1-blue)
![Status](https://img.shields.io/badge/status-pre--release-orange)
![Rust](https://img.shields.io/badge/Rust-CE422B?logo=rust&logoColor=white)

![OrqaStudio](https://github.com/orqastudio/orqastudio-brand/blob/main/assets/banners/banner-1680x240.png?raw=1)

# orqa-mcp-server

> **Pre-release** — Tool interfaces may change without notice until v1.0.0.

MCP server exposing OrqaStudio's artifact graph and search tools over JSON-RPC stdio. Used by Claude Code and other MCP-compatible clients to query the artifact graph and run semantic search without the Tauri app running.

---

## Features

- 11 MCP tools:
  - Graph tools: `graph_query`, `graph_resolve`, `graph_relationships`, `graph_stats`, `graph_validate`, `graph_read`, `graph_refresh`
  - Search tools: `search_regex`, `search_semantic`, `search_research`, `search_status`
- MCP resources exposing `core.json` and `project.json` schemas
- JSON-RPC over stdio — compatible with any MCP client
- Standalone binary (`orqa-mcp-server`) for direct process invocation
- Also compiled as a library crate for embedding in the Tauri app
- Depends on `orqa-search` for all search operations

## Usage

### As a library

```toml
[dependencies]
orqa-mcp-server = { path = "libs/mcp-server" }
```

### Standalone binary

```bash
cargo build --release --bin orqa-mcp-server
./target/release/orqa-mcp-server
```

The binary reads from stdin and writes to stdout using the MCP JSON-RPC protocol.

### MCP client configuration

```json
{
  "mcpServers": {
    "orqastudio": {
      "command": "orqa-mcp-server",
      "args": []
    }
  }
}
```

## Development

```bash
cargo build
cargo test
```

## License

BSL-1.1 — see [LICENSE](LICENSE) for details.
