---
id: DOC-074
title: "How To: Write Frontend Tests in OrqaStudio"
description: "Practical guide for writing Vitest tests for Svelte 5 stores and components in OrqaStudio."
created: 2026-03-14
updated: 2026-03-14
sort: 3
relationships:
  - target: RULE-029
    type: informed-by
    rationale: Testing standards define organisation, coverage requirements, and mock boundaries
  - target: RULE-006
    type: informed-by
    rationale: Coding standards (Svelte 5 runes, strict TypeScript) apply inside test files too
  - target: DOC-021
    type: informed-by
    rationale: Coding standards document is the source of truth for TypeScript strictness requirements
  - type: informed-by
    target: PILLAR-001
    rationale: This document describes architecture or practices that serve the Clarity Through Structure pillar
  - type: informed-by
    target: PILLAR-003
    rationale: This document describes features or patterns that serve the Purpose Through Continuity pillar
---

## Test Framework

OrqaStudio uses **Vitest** for frontend unit tests. Vitest is configured via
`ui/vite.config.ts` and shares the same module resolution as the app — `$lib`
aliases and `@orqastudio/types` imports all work inside tests.

---

## Where Tests Live

Test files sit alongside the code they test, named `*.test.ts`:

```
ui/src/lib/
├── stores/
│   ├── session.svelte.ts
│   ├── artifact.svelte.ts
│   └── __tests__/
│       ├── setup.ts          # shared mock setup — import this first
│       ├── session.test.ts
│       └── artifact.test.ts
├── components/
│   └── shared/
│       └── EmptyState.test.ts  # component tests alongside the component
```

---

## Running Tests

```bash
# All frontend tests (run once)
make test-frontend

# Watch mode — reruns affected tests on save
make test-watch

# Single test file
npx vitest run ui/src/lib/stores/__tests__/session.test.ts
```

---

## Mocking Tauri `invoke`

Stores call `invoke()` from `@tauri-apps/api/core`. In tests, mock the entire
module so calls resolve to whatever value you configure:

```typescript
// At the TOP of setup.ts — this runs before any imports of stores
import { vi } from "vitest";

const mockInvoke = vi.fn();

vi.mock("@tauri-apps/api/core", () => ({
  invoke: mockInvoke,
  Channel: class MockChannel<T> {
    onmessage: ((event: T) => void) | null = null;
    emit(event: T) {
      this.onmessage?.(event);
    }
  },
}));

export { mockInvoke };
```

Import `mockInvoke` from `setup.ts` and reset it in `beforeEach`:

```typescript
import { describe, it, expect, beforeEach } from "vitest";
import { mockInvoke } from "./setup";
import { sessionStore } from "../session.svelte";

beforeEach(() => {
  mockInvoke.mockReset();   // clear call history and configured values
  sessionStore.clear();     // reset store state to initial values
});
```

---

## Testing Stores

Stores in OrqaStudio are runes-based (`.svelte.ts` files). Test them by calling
their action methods and asserting on their reactive state:

```typescript
describe("SessionStore", () => {
  it("loads sessions from backend", async () => {
    const fakeSummaries = [
      {
        id: 1,
        title: "Test session",
        status: "active",
        message_count: 5,
        preview: null,
        created_at: "2026-01-01T00:00:00Z",
        updated_at: "2026-01-01T00:00:00Z",
      },
    ];

    // Configure what invoke() returns for this test
    mockInvoke.mockResolvedValueOnce(fakeSummaries);

    await sessionStore.loadSessions(10);

    // Assert the command name and arguments
    expect(mockInvoke).toHaveBeenCalledWith("session_list", { projectId: 10 });

    // Assert the store's reactive state
    expect(sessionStore.sessions).toEqual(fakeSummaries);
    expect(sessionStore.isLoading).toBe(false);
    expect(sessionStore.error).toBeNull();
  });

  it("sets error state on backend failure", async () => {
    mockInvoke.mockRejectedValueOnce(new Error("DB error"));

    await sessionStore.loadSessions(10);

    expect(sessionStore.error).toBe("DB error");
    expect(sessionStore.isLoading).toBe(false);
    expect(sessionStore.sessions).toEqual([]); // state unchanged on error
  });
});
```

**Always test three states per async action:** success, error, and the initial
(before the action runs). These map to the loading → loaded → error lifecycle that
every store must handle.

---

## Mocking Other Modules

When a store depends on another module (e.g., the artifact-graph SDK), mock it
before importing the store:

```typescript
import { vi } from "vitest";

// Mock before the store import
vi.mock("$lib/sdk/artifact-graph.svelte", () => ({
  artifactGraphSDK: {
    readContent: vi.fn(),
    resolve: vi.fn(),
  },
}));

// Now import the store — it gets the mocked SDK
import { artifactStore } from "../artifact.svelte";
import { artifactGraphSDK } from "$lib/sdk/artifact-graph.svelte";

beforeEach(() => {
  vi.mocked(artifactGraphSDK.readContent).mockReset();
  artifactStore.clear();
});

it("loads content via SDK", async () => {
  vi.mocked(artifactGraphSDK.readContent).mockResolvedValueOnce("# EPIC-001\n...");

  await artifactStore.loadContent(".orqa/delivery/epics/EPIC-001.md");

  expect(artifactGraphSDK.readContent).toHaveBeenCalledWith(
    ".orqa/delivery/epics/EPIC-001.md"
  );
  expect(artifactStore.activeContent).toBe("# EPIC-001\n...");
});
```

---

## Testing Svelte 5 Components

Component tests use `@testing-library/svelte`. Pass props directly and query
the rendered output:

```typescript
import { render, screen } from "@testing-library/svelte";
import EmptyState from "../EmptyState.svelte";

it("renders title and description", () => {
  render(EmptyState, {
    title: "No epics yet",
    description: "Create your first epic to get started.",
  });

  expect(screen.getByText("No epics yet")).toBeInTheDocument();
  expect(screen.getByText("Create your first epic to get started.")).toBeInTheDocument();
});
```

Test components at the prop boundary — don't reach into internals. If a component
shows different content based on a boolean prop, write one test for each branch.

---

## TypeScript in Tests

Test files are TypeScript with the same `strict: true` config as production code.
This means:

- No `any` types in test fixtures — define inline types or import from `$lib/types`
- `vi.mocked(fn)` rather than casting: `(fn as jest.Mock)`
- Type your fake data explicitly so TypeScript catches mismatches with the real type

```typescript
import type { Session } from "$lib/types";

const fakeSession: Session = {
  id: 1,
  project_id: 10,
  title: "Test session",
  model: "auto",
  system_prompt: null,
  status: "active",
  // TypeScript will error if you omit a required field
};
```

---
