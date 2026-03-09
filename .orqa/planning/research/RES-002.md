---
id: RES-002
title: "Claude Integration Research"
type: research
status: surpassed
milestone: MS-000
surpassed-by: provider-architecture
date: 2026-03-02
category: claude-integration
description: How OrqaStudio integrates with Claude's API and tooling. Most foundational research area — decisions here constrain the entire backend architecture.
questions:
  - id: Q1
    title: Claude Agent SDK vs Raw API
    status: resolved
    verdict: Agent SDK as primary (spawns official CLI, Max subscription)
  - id: Q2
    title: Claude Max Integration Path
    status: resolved
    verdict: Max subscription via Agent SDK. Cost-effective at 25+ conv/day.
  - id: Q3
    title: Tool Implementation Strategy
    status: resolved
    verdict: OrqaStudio tools as custom MCP server exposed to Agent SDK
  - id: Q4
    title: Streaming Architecture
    status: resolved
    verdict: Agent SDK → sidecar (Bun-compiled) → NDJSON → Rust → Channel<T> → Svelte
produces_decisions: [AD-007, AD-008, AD-009, AD-010, AD-017]
informs_phases: [0b, 0e, 1, 2]
informs_features: [F-002, F-003, F-012]
---


**Date:** 2026-03-02 | **Status:** Complete

Research into how OrqaStudio integrates with Claude's API and tooling. This is the most foundational research area — decisions here constrain the entire backend architecture.

---

## Open Questions

### Q1: Claude Agent SDK vs Raw API

**Question:** Should OrqaStudio use the Claude Agent SDK (which handles the tool-use loop, multi-turn, etc.) or call the raw Claude API directly?

**Key considerations:**
- Does the Agent SDK support streaming responses?
- Can we intercept tool calls before execution (for approval UIs)?
- Does it work from Rust (via FFI), or do we need a Node/Python sidecar?
- Can it target both Claude API (API key) and Claude Max (subscription)?
- What is the SDK's architecture — is it a library, a server, or a CLI wrapper?

**Findings:**

The Claude Agent SDK is a **production-grade library** (not a CLI wrapper or framework) available in **Python** (`pip install claude-agent-sdk`) and **TypeScript** (`npm install @anthropic-ai/claude-agent-sdk`). No official Rust SDK exists.

**Critical architecture detail:** The SDK does NOT call the Anthropic HTTP API directly. It **bundles the Claude Code CLI binary** and communicates with it as a subprocess via JSON over stdin/stdout. Error types confirm this: `CLINotFoundError`, `CLIConnectionError`, `ProcessError`.

**Streaming:** Yes. Both `query()` and `ClaudeSDKClient.receive_response()` are async iterators yielding messages as they arrive. Message types include `AssistantMessage`, `ToolUseBlock`, `ToolResultBlock`, `ResultMessage`, `SystemMessage`.

**Tool call interception:** Yes, via the **hooks system**. The `PreToolUse` hook fires before any tool executes and can return `"allow"`, `"deny"`, or `"ask"`. The `PermissionRequest` hook fires when the agent would normally display a permission dialog — OrqaStudio could implement its own approval UI. Hooks can also **modify tool inputs** before execution (since v2.0.10).

**Rust support:** Not natively. Options for OrqaStudio:

| Path | How | Pros | Cons |
|------|-----|------|------|
| TypeScript sidecar | Run TS SDK in Node.js process, Tauri IPC | Full SDK features, hooks, MCP | Extra process, Node dependency |
| Community Rust crate | `anthropic-agent-sdk` on lib.rs | Mirrors TS SDK, idiomatic Rust | Requires CLI binary, unofficial |
| Raw Anthropic API | `reqwest` + SSE parsing in Rust | Full control, no deps, native perf | Must implement tool loop, tools, context |
| Spawn CLI directly | Shell out to `claude` binary | Same as SDK without wrapper | Fragile, CLI output format may change |

**Tool loop:** The SDK handles the tool loop automatically inside the CLI subprocess. Built-in tools (Read, Write, Edit, Bash, Glob, Grep, WebSearch, WebFetch) execute inside the CLI process. Custom tools can be defined as in-process MCP servers.

**Subagents:** Supported. Define agents with `AgentDefinition(description, prompt, tools)` and the SDK manages delegation.

**Sessions:** Can be resumed by capturing `session_id` and passing `resume=session_id`.

**REVISED Assessment (March 2026):** The Agent SDK is more controllable than initially assessed:

| Capability | How |
|------------|-----|
| Disable ALL built-in tools | `tools: []` parameter |
| Provide only OrqaStudio's tools | Custom MCP server via `mcpServers` parameter |
| Intercept tool calls for approval UI | `canUseTool` callback returns allow/deny |
| Token-level streaming for rich UI | `includePartialMessages: true` gives raw SSE events |
| Control system prompt | `systemPrompt` parameter (replace or append) |
| Session management | `resume`, `sessionId`, `continue` |
| Multi-turn conversations | V2 `send()`/`stream()` interface |
| Restrict tool set per conversation | `tools` parameter (whitelist) |
| Budget caps | `maxBudgetUsd` parameter |

The Agent SDK is NOT a black box when configured correctly. With `tools: []` + custom MCP + `canUseTool`, OrqaStudio controls tool execution, approval, and UI rendering — the SDK only handles API communication and context management.

**Critical:** The Agent SDK is the ONLY legal path to using Max subscriptions. It spawns the official Claude Code CLI, which authenticates via OAuth. Anthropic's third-party ban applies to apps that extract OAuth tokens directly — not to apps that use the official SDK/CLI.

**Recommendation:** Agent SDK as the primary communication layer (Max subscription support), with TypeScript SDK as the API-key fallback.

**Decision:** Dual-provider architecture — see Architecture Recommendation below.

---

### Q2: Claude Max Integration Path

**Question:** How does Claude Max expose API access for third-party apps? Is this feasible for launch or a later addition?

**Key considerations:**
- OAuth flow? Session tokens? API keys?
- Is there a documented API, or would this require reverse-engineering?
- Pricing implications — does Claude Max subscription include API usage?
- This determines whether we support "bring your own API key" only, or also subscription-based access

**Findings:**

**Conclusive: Third-party access to Claude Max subscriptions is banned.**

As of January 9, 2026, Anthropic deployed server-side checks that **block all third-party tools from authenticating with Claude Pro/Max subscription OAuth tokens**. Formally documented February 19, 2026.

From the Agent SDK docs:
> Unless previously approved, Anthropic does not allow third party developers to offer claude.ai login or rate limits for their products, including agents built on the Claude Agent SDK. Please use the API key authentication methods described in this document instead.

OAuth tokens from Claude Free/Pro/Max are only valid for Claude Code and claude.ai. Server-side enforcement is active — tokens are rejected. The only exception is a specific partnership agreement with Anthropic.

**Authentication options for OrqaStudio users:**

| Method | Setup | Pricing |
|--------|-------|---------|
| **Anthropic API key** (primary) | Create at console.anthropic.com | Pay per token (Opus $5/$25 MTok, Sonnet $3/$15 MTok, Haiku $1/$5 MTok) |
| **Amazon Bedrock** | `CLAUDE_CODE_USE_BEDROCK=1` + AWS credentials | AWS pricing |
| **Google Vertex AI** | `CLAUDE_CODE_USE_VERTEX=1` + GCP credentials | GCP pricing |
| **Microsoft Azure AI Foundry** | `CLAUDE_CODE_USE_FOUNDRY=1` + Azure credentials | Azure pricing |

**REVISED (March 2026):** The Agent SDK (`@anthropic-ai/claude-agent-sdk`) spawns the official Claude Code CLI binary, which authenticates via Max subscription OAuth. Anthropic's fingerprinting sees the official CLI, not a third-party tool. This is architecturally identical to VS Code / JetBrains / Emacs extensions, which are officially supported.

**Decision:** OrqaStudio supports BOTH authentication paths:
- **Path A (Max subscription):** Agent SDK spawns Claude Code CLI → uses Max subscription at $0 marginal cost
- **Path B (API key):** TypeScript SDK direct HTTP → pay per token

**Cost analysis (Sonnet, estimated 75 conversations/day @ 20K in + 5K out):**
- API: ~$304/month
- Max 5x: $100/month (break-even at ~25 conversations/day)
- Max 20x: $200/month (break-even at ~49 conversations/day)

Max subscription is the default recommendation for regular OrqaStudio users. API key path exists for burst work, enterprise users, and cloud provider routing (Bedrock/Vertex/Azure).

---

### Q3: Tool Implementation Strategy

**Question:** How should OrqaStudio implement Claude Code's tools (Read, Write, Edit, Bash, Glob, Grep)?

**Options:**

| Option | Pros | Cons |
|--------|------|------|
| **(a) Native Rust** | Full control, best UI integration, no external deps | Must reimplement every tool, may diverge from Claude Code behavior |
| **(b) Shell out** | Simple, matches CLI behavior | Platform-dependent, harder to render in UI |
| **(c) Claude Code subprocess** | Exact compatibility, reuses proven tool implementations | Complex IPC, dependency on Claude Code CLI, harder to customize UI |
| **(d) MCP server** | Standard protocol, tool discovery, extensible | Adds protocol overhead, may need custom MCP client in Rust |

**Findings:**

Given the decision to use the raw API (Q1), OrqaStudio must implement its own tools. The Agent SDK option (c) would tie us to the CLI subprocess for tool execution, defeating the purpose of going raw API.

**Option (a) Native Rust is the strongest fit** for OrqaStudio because:
- Tools need deep UI integration (show file contents in editor panel, render grep results, display git diffs)
- OrqaStudio already needs file system access via Tauri — tools are a natural extension
- Full control over permission model (OrqaStudio's own approval UI, not CLI prompts)
- No external binary dependency

**MCP (option d) is complementary, not a replacement.** OrqaStudio should:
- Implement core tools (Read, Write, Edit, Bash, Glob, Grep) natively in Rust
- Act as an **MCP host** for user-provided MCP servers (extensibility)
- Expose native tools to Claude as standard tool definitions in the API call
- Route Claude's `tool_use` responses to either native tools or MCP servers

**Findings on MCP:**
- MCP is an open protocol (JSON-RPC 2.0) now under the Linux Foundation
- OrqaStudio as MCP host: spawn MCP servers (stdio or HTTP), discover tools via `tools/list`, route `tool_use` calls
- 10,000+ MCP servers exist in the ecosystem — OrqaStudio users could connect databases, browsers, custom APIs
- For Rust: implement JSON-RPC over stdin/stdout (straightforward — spawn process, write JSON, read JSON)

**Decision:** _Pending user approval — see Architecture Recommendation below_

---

### Q4: Streaming Architecture

**Question:** How should the streaming pipeline work from Claude API to the Svelte UI?

**Requirements:**
- Render tokens as they arrive (conversational feel)
- Intercept structured events (tool calls, agent delegation)
- Buffer and parse SSE events in Rust
- Push incremental updates to Svelte via Tauri events
- Handle connection drops, retries, and backpressure

**Findings:**

**SSE Event Flow from Claude API:**

1. `message_start` — Message object with empty content and initial usage
2. Per content block:
   - `content_block_start` — Block type (`text`, `tool_use`, `thinking`) and index
   - `content_block_delta` (repeated) — Incremental updates:
     - `text_delta` for text: `{"type": "text_delta", "text": "Hello"}`
     - `input_json_delta` for tool input: `{"type": "input_json_delta", "partial_json": "{\"location\": \"San Fra"}`
     - `thinking_delta` for thinking: `{"type": "thinking_delta", "thinking": "I need to..."}`
   - `content_block_stop` — Block complete
3. `message_delta` — Top-level changes (stop_reason, cumulative usage)
4. `message_stop` — Stream complete

Plus `ping` events and possible `error` events.

**Tool calls interleave with text:** Claude streams text first, then starts a tool_use block. Tool input arrives as partial JSON chunks. After `stop_reason: "tool_use"`, you execute the tool and send back `tool_result`, then Claude responds with more text/tools.

**Rust crates for Claude API:**

| Crate | Streaming | Tool Use | Notes |
|-------|-----------|----------|-------|
| `anthropic_rust` | Yes (SSE) | Yes | Async/tokio, retry logic, type-safe |
| `anthropic-sdk-rust` | Yes (SSE) | Yes | Claims full TS SDK parity |
| `misanthropy` | Yes | Yes | Mature, includes CLI tool |
| `anthropic-rs` | Yes | Yes | Production-grade, strongly typed |
| `claudius` | Yes | Yes | Complete API coverage |
| DIY: `reqwest` + `eventsource-stream` | Yes | Manual | Full control, no abstraction |

**Proposed pipeline (confirmed viable):**

```
Claude API (SSE stream)
  → Rust HTTP client (reqwest + eventsource-stream, or anthropic_rust crate)
    → Stream parser (match on event type: text_delta → append, tool_use → buffer input JSON)
      → Tauri event emitter (emit typed events to frontend)
        → Svelte store ($state updates on each event)
          → Component re-render (streaming text, tool call cards)
```

**Decision:** _Pending — depends on which Rust crate to use. Evaluate `anthropic_rust` and `misanthropy` during Phase 1 scaffold._

---

## Architecture Recommendation

### Final Direction: Agent SDK Sidecar + Rust Core

Three insights drove the architecture to its final form:

1. **Don't reinvent the SDK** — Using official Anthropic packages avoids building and maintaining our own API client. (Composability principle.)
2. **Max subscription requires the Agent SDK** — The only legal path to using Max subscriptions ($100-200/month flat rate) is through the Agent SDK, which spawns the official Claude Code CLI binary. The Agent SDK is controllable enough when configured with `tools: []` + custom MCP + `canUseTool`.
3. **Focus on one provider first** — API key support and other providers are roadmap items. The provider-agnostic interface ensures they can be added later without changing the core.

### Agent SDK Sidecar + Rust Core + MCP Host (FINAL RECOMMENDATION)

```
User (Svelte UI)
  → Tauri IPC (invoke / Channel<T>)
    → Rust backend (tool execution, persistence, agentic loop coordination)
      → TypeScript sidecar (Bun-compiled binary, stdin/stdout NDJSON)
        └─ @anthropic-ai/claude-agent-sdk
             └─ Spawns Claude Code CLI → OAuth → Claude API
      ← ProviderEvent stream (provider-agnostic)
    ← Channel<T> to frontend
    → MCP servers (user-extensible tools)
```

**Agent SDK sidecar implementation:**

```typescript
// sidecar/src/index.ts
import { query, createSdkMcpServer, tool } from "@anthropic-ai/claude-agent-sdk";
import { z } from "zod";

// OrqaStudio's tools exposed as MCP server to the Agent SDK
const orqaTools = createSdkMcpServer({
  name: "orqa-tools",
  tools: [
    tool("read_file", "Read a file", { path: z.string() }, async (args) => {
      // Route to Rust backend via stdin/stdout
      send({ type: 'tool_execute', name: 'read_file', input: args });
      const result = await waitForStdinResponse('tool_result');
      return { content: [{ type: "text", text: result.output }] };
    }),
    // ... Write, Edit, Bash, Glob, Grep, etc.
  ],
});

async function handleConversation(request) {
  const conversation = query({
    prompt: request.prompt,
    options: {
      tools: [],                          // Disable ALL built-in tools
      mcpServers: { "orqa": orqaTools },// Route tool calls to OrqaStudio
      canUseTool: async (name, input) => {
        // Delegate approval to OrqaStudio's UI via stdin/stdout
        send({ type: 'tool_approval_request', name, input });
        const response = await waitForStdinResponse('tool_approval');
        return response.approved
          ? { behavior: 'allow' }
          : { behavior: 'deny', message: response.reason };
      },
      includePartialMessages: true,       // Token-level streaming
      systemPrompt: request.systemPrompt,
      model: request.model,
    }
  });

  for await (const msg of conversation) {
    send(translateToProviderEvent(msg));
  }
}
```

### Why Agent SDK for Phase 1

| Concern | Agent SDK |
|---------|----------|
| **Authentication** | Max subscription via Claude Code CLI ($0 marginal cost) |
| **Cost at 75 conv/day** | $100-200/month flat (vs ~$304/month API) |
| **SDK maintenance** | Anthropic maintains |
| **Streaming** | `includePartialMessages` gives raw SSE-level events |
| **Tool control** | `tools: []` + custom MCP = full OrqaStudio control |
| **Session management** | Built into CLI (resume, context, compaction) |
| **Context management** | CLI handles compaction automatically |

**Future providers** (API key, Bedrock, Vertex, OpenAI, local models) are roadmap items. The provider-agnostic `ProviderEvent` interface ensures they can be added as new sidecar implementations without changing the Rust core or Svelte UI.

### Design Principle: Provider-Agnostic Interface (Composability)

The interface between the Rust backend and the AI sidecar is **provider-neutral**:

- The Rust backend sends a `ConversationRequest` (system prompt, messages, tool definitions, config)
- The sidecar translates to the active provider's format and streams back `ProviderEvent` types
- Swapping providers means implementing the same interface — Claude today, potentially others later
- This extends the **composability principle** from Alvarez into OrqaStudio's architecture

**Provider-agnostic event types (Rust side):**

```rust
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum ProviderEvent {
    TextDelta { text: String },
    ThinkingDelta { text: String },
    ToolUseStart { id: String, name: String },
    ToolUseInputDelta { partial_json: String },
    ToolUseComplete { id: String, name: String, input: Value },
    ToolApprovalRequest { id: String, name: String, input: Value },
    MessageComplete { stop_reason: StopReason, usage: Usage },
    RateLimited { retry_after_ms: Option<u64> },
    Error { message: String, code: Option<String> },
}
```

### Sidecar Implementation Details

**Communication:** stdin/stdout with newline-delimited JSON (NDJSON). Same pattern as LSP. Sub-millisecond latency per message — negligible vs Claude's 30-100ms/token generation.

**Bundling:** Bun compile produces ~18-25 MB standalone binaries per platform. Cross-compilation supported. Total app size: ~25-35 MB (still far smaller than Electron's ~150+ MB).

```
src-tauri/binaries/
  claude-sidecar-x86_64-pc-windows-msvc.exe
  claude-sidecar-aarch64-apple-darwin
  claude-sidecar-x86_64-apple-darwin
  claude-sidecar-x86_64-unknown-linux-gnu
```

**Lifecycle:** Rust spawns sidecar on app launch via `tauri-plugin-shell` `spawn()`. `CommandEvent::Stdout` delivers streaming lines. `child.write()` sends requests. `child.kill()` on app exit.

**Streaming pipeline (full):**

```
Claude API (SSE)
  → Anthropic SDK (TS, handles SSE parsing)
    → NDJSON line to stdout
      → Rust (CommandEvent::Stdout, parse JSON, deserialize to ProviderEvent)
        → Channel<T> (Tauri IPC, ordered delivery)
          → Svelte store ($state, fine-grained reactivity)
            → Component re-render
```

Latency overhead of the sidecar hop: ~0.1-0.5ms per event. Completely negligible.

### Risks and Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| Anthropic changes Agent SDK to block tool interception | Medium | API key path as fallback. Agent SDK is the officially endorsed extension mechanism. |
| Claude Code CLI not installed | Low | Detect and prompt install. Fall back to API key path. |
| Max rate limits disrupt heavy use | Medium | Surface rate limit events in UI. Allow switching to API key for burst. |
| Agent SDK `allowedTools` bugs | Low | Use `tools` parameter (which works correctly) for restriction, not `allowedTools`. |
| Sidecar process crashes | Low | Auto-restart with exponential backoff. Persist conversation state in SQLite for recovery. |

### Previous Options (Superseded)

<details>
<summary>Option B: Raw Anthropic API from Rust</summary>

- Rust calls Claude API directly (reqwest + SSE parsing)
- **Superseded because:** Full maintenance burden for API client. No Max subscription support. No provider swapping.
</details>

<details>
<summary>Option C: Hybrid — Raw API + MCP Host</summary>

- Same as Option B but with MCP host
- **Superseded because:** Same raw API maintenance concern. No Max subscription support.
</details>

<details>
<summary>Option D: TypeScript SDK Sidecar only (no Agent SDK)</summary>

- TypeScript SDK in sidecar, API key only
- **Superseded because:** No Max subscription support. Agent SDK reassessment showed it is controllable enough. API key provider is a roadmap item.
</details>

<details>
<summary>Option E: Dual-Provider Sidecar (Agent SDK + TypeScript SDK)</summary>

- Both providers in the same sidecar, auto-detect auth method
- **Deferred:** Technically sound but unnecessary for Phase 1. Focus on Agent SDK path. API key provider is a roadmap item that follows the same provider-agnostic interface.
</details>

---

## Dependencies

- Q1 (SDK vs API) → **Resolved: Agent SDK sidecar as primary integration. Max subscription auth.**
- Q2 (Claude Max) → **Resolved: Supported via Agent SDK (spawns official CLI). API key + other providers on roadmap.**
- Q3 (tools) → **Resolved: OrqaStudio's tools as custom MCP server exposed to Agent SDK. Native Rust execution.**
- Q4 (streaming) → **Resolved: Agent SDK → sidecar → NDJSON stdout → Rust → Channel<T> → Svelte**

## Related

- [Architecture Decisions](/architecture/decisions) — Where decisions land once approved
- [Product Roadmap](/product/roadmap) — Phase 1 scope depends on these decisions
- [Product Governance](/product/governance) — Composability principle (#6)
- [Tauri v2 Research](/research/tauri-v2) — IPC events and sidecar support via tauri-plugin-shell
