# @orqastudio/sdk

[![Apache 2.0 License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)

Frontend-to-backend connection SDK for OrqaStudio apps and plugins.

Provides the core IPC layer, artifact graph SDK, and utility functions used by all OrqaStudio frontend packages.

## Installation

```bash
npm install @orqastudio/sdk
```

### Peer Dependencies

- `svelte` >= 5.0.0 (for the reactive artifact graph SDK)
- `@tauri-apps/api` >= 2.0.0 (for IPC invoke and event listening)

## Usage

### Artifact Graph SDK

The main export is a singleton reactive graph SDK that maintains an in-memory copy of the artifact graph:

```ts
import { artifactGraphSDK } from "@orqastudio/sdk";

// Initialize once at app startup
await artifactGraphSDK.initialize({
  projectPath: "/path/to/project",
});

// Synchronous lookups — no IPC round-trips
const epic = artifactGraphSDK.resolve("EPIC-048");
const tasks = artifactGraphSDK.byType("task");
const refs = artifactGraphSDK.referencesFrom("RULE-006");

// Subscribe to changes
const unlisten = artifactGraphSDK.subscribe("EPIC-048", (node) => {
  console.log("Epic updated:", node.title);
});

// Graph health
const broken = artifactGraphSDK.brokenRefs();
const orphaned = artifactGraphSDK.orphans();
const missing = artifactGraphSDK.missingInverses();
```

### IPC Invoke Wrapper

Type-safe Tauri invoke wrapper with structured error handling:

```ts
import { invoke, extractErrorMessage } from "@orqastudio/sdk/ipc";

const result = await invoke<MyType>("my_command", { arg: "value" });
```

### Frontmatter Parser

Lightweight YAML frontmatter parser for markdown files:

```ts
import { parseFrontmatter } from "@orqastudio/sdk/utils";

const { metadata, body } = parseFrontmatter(markdownContent);
```

## Sub-path Exports

| Export | Description |
|--------|-------------|
| `@orqastudio/sdk` | Everything (graph SDK, IPC, utils) |
| `@orqastudio/sdk/graph` | Artifact graph SDK only |
| `@orqastudio/sdk/ipc` | IPC invoke wrapper only |
| `@orqastudio/sdk/utils` | Frontmatter parser only |

## Development

```bash
npm install
npm run build    # TypeScript compilation
npm test         # Run tests
npm run check    # Type-check + tests
```

## License

[Apache 2.0](LICENSE)
