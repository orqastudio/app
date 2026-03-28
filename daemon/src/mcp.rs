// MCP protocol note for the OrqaStudio daemon.
//
// MCP (Model Context Protocol) uses stdio as its transport: the LLM client
// (e.g., Claude Code) spawns `orqa-mcp-server` as a subprocess and
// communicates with it over stdin/stdout. Each client manages its own server
// process lifetime.
//
// The daemon does NOT pre-spawn or manage an MCP server instance because:
//
//   1. The MCP binary reads JSON-RPC from stdin and responds on stdout. If the
//      daemon were to spawn it with stdin set to null (which it must — the
//      daemon has no MCP client to wire up), the server would see EOF
//      immediately and exit. The process would be useless.
//
//   2. Multiple LLM clients may be active simultaneously (Claude Code in the
//      IDE, the app sidecar, a CI agent). Each requires its own stdio pipe; a
//      single shared daemon-managed process cannot serve multiple clients.
//
//   3. core.md §3.2 states: "MCP and LSP are access protocols that expose
//      engine capabilities to consumers. They are NOT application boundaries."
//      Business logic belongs in the engine crates. The daemon is the
//      infrastructure layer, not an MCP broker.
//
// LLM clients configure `orqa-mcp-server <project-path>` as their MCP command
// directly (e.g., in `.mcp.json` or the tool's settings). The daemon manages
// LSP (TCP-based, legitimately persistent and shared across editors) but NOT
// MCP (stdio-based, client-managed lifecycle).
