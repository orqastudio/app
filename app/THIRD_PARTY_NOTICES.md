# Third-Party Notices

OrqaStudio™ includes or depends on the following third-party software. Each component is used in accordance with its respective license.

---

## Rust Dependencies

| Crate | License | Purpose |
|-------|---------|---------|
| [tauri](https://github.com/tauri-apps/tauri) | Apache-2.0 / MIT | Desktop application framework |
| [tauri-plugin-fs](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | File system access with watch |
| [tauri-plugin-shell](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | Shell command execution |
| [tauri-plugin-store](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | Key-value persistence |
| [tauri-plugin-window-state](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | Window position/size persistence |
| [tauri-plugin-dialog](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | Native dialog windows |
| [tauri-plugin-notification](https://github.com/tauri-apps/plugins-workspace) | Apache-2.0 / MIT | System notifications |
| [rusqlite](https://github.com/rusqlite/rusqlite) | MIT | SQLite database bindings |
| [serde](https://github.com/serde-rs/serde) | Apache-2.0 / MIT | Serialization framework |
| [serde_json](https://github.com/serde-rs/json) | Apache-2.0 / MIT | JSON serialization |
| [serde_yaml](https://github.com/dtolnay/serde-yaml) | Apache-2.0 / MIT | YAML serialization |
| [thiserror](https://github.com/dtolnay/thiserror) | Apache-2.0 / MIT | Error derive macros |
| [tokio](https://github.com/tokio-rs/tokio) | MIT | Async runtime |
| [reqwest](https://github.com/seanmonstar/reqwest) | Apache-2.0 / MIT | HTTP client |
| [futures-util](https://github.com/rust-lang/futures-rs) | Apache-2.0 / MIT | Async utilities |
| [duckdb](https://github.com/duckdb/duckdb-rs) | MIT | DuckDB bindings for vector search |
| [ort](https://github.com/pykeio/ort) | Apache-2.0 / MIT | ONNX Runtime bindings |
| [tokenizers](https://github.com/huggingface/tokenizers) | Apache-2.0 | Hugging Face tokenizer library |
| [ndarray](https://github.com/rust-ndarray/ndarray) | Apache-2.0 / MIT | N-dimensional array operations |
| [regex](https://github.com/rust-lang/regex) | Apache-2.0 / MIT | Regular expressions |
| [glob](https://github.com/rust-lang/glob) | Apache-2.0 / MIT | File path globbing |
| [ignore](https://github.com/BurntSushi/ripgrep) | MIT / Unlicense | Gitignore-aware file walking |
| [base64](https://github.com/marshallpierce/rust-base64) | Apache-2.0 / MIT | Base64 encoding/decoding |
| [sha2](https://github.com/RustCrypto/hashes) | Apache-2.0 / MIT | SHA-2 hash functions |
| [tracing](https://github.com/tokio-rs/tracing) | MIT | Structured logging |
| [tempfile](https://github.com/Stebalien/tempfile) | Apache-2.0 / MIT | Temporary file handling |

## Frontend Dependencies

| Package | License | Purpose |
|---------|---------|---------|
| [Svelte](https://github.com/sveltejs/svelte) | MIT | UI framework |
| [SvelteKit](https://github.com/sveltejs/kit) | MIT | Application framework |
| [Tailwind CSS](https://github.com/tailwindlabs/tailwindcss) | MIT | Utility-first CSS framework |
| [@tauri-apps/api](https://github.com/tauri-apps/tauri) | Apache-2.0 / MIT | Tauri frontend API |
| [bits-ui](https://github.com/huntabyte/bits-ui) | MIT | Headless UI components (shadcn-svelte) |
| [paneforge](https://github.com/huntabyte/paneforge) | MIT | Resizable pane layouts |
| [Lucide](https://github.com/lucide-icons/lucide) | ISC | Icon library |
| [@humanspeak/svelte-markdown](https://github.com/humanspeak/svelte-markdown) | MIT | Markdown rendering |
| [svelte-highlight](https://github.com/metonym/svelte-highlight) | MIT | Syntax highlighting |
| [clsx](https://github.com/lukeed/clsx) | MIT | Class name utility |
| [tailwind-merge](https://github.com/dcastil/tailwind-merge) | MIT | Tailwind class merging |
| [tailwind-variants](https://github.com/nextui-org/tailwind-variants) | MIT | Tailwind variant API |
| [tw-animate-css](https://github.com/Wombosvideo/tw-animate-css) | MIT | Tailwind animation utilities |
| [zod](https://github.com/colinhacks/zod) | MIT | Schema validation |
| [TypeScript](https://github.com/microsoft/TypeScript) | Apache-2.0 | Type system |
| [Vite](https://github.com/vitejs/vite) | MIT | Build tool |
| [ESLint](https://github.com/eslint/eslint) | MIT | Linting |
| [Vitest](https://github.com/vitest-dev/vitest) | MIT | Testing framework |

## Sidecar Dependencies

| Package | License | Purpose |
|---------|---------|---------|
| [@anthropic-ai/claude-agent-sdk](https://github.com/anthropics/claude-code) | Apache-2.0 | Claude Agent SDK for AI conversations |
| [zod](https://github.com/colinhacks/zod) | MIT | Schema validation |
| [Bun](https://github.com/oven-sh/bun) | MIT | JavaScript runtime (sidecar compilation) |

## Fonts

| Font | License | Purpose |
|------|---------|---------|
| [Inter](https://github.com/rsms/inter) | SIL Open Font License 1.1 | UI typeface |
| [JetBrains Mono](https://github.com/JetBrains/JetBrainsMono) | SIL Open Font License 1.1 | Monospace typeface |

---

This file is provided for informational purposes. For authoritative license terms, refer to each project's repository. Dependency versions and their exact licenses can be verified via `Cargo.lock` and `package-lock.json`.
