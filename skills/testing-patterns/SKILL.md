---
id: SKILL-SVE-003
type: skill
name: Testing Patterns
status: active
plugin: "@orqastudio/plugin-svelte"
relationships:
  - target: DOC-SVE-001
    type: synchronised-with
---

# Testing Patterns

## Framework

Vitest for unit/integration tests. @testing-library/svelte for component tests.

## File Convention

Test files live alongside the code they test:

```
src/lib/components/MyComponent.svelte
src/lib/components/MyComponent.test.ts
```

## Component Testing

```typescript
import { render, screen } from "@testing-library/svelte";
import { describe, it, expect } from "vitest";
import MyComponent from "./MyComponent.svelte";

describe("MyComponent", () => {
  it("renders the title", () => {
    render(MyComponent, { props: { title: "Hello" } });
    expect(screen.getByText("Hello")).toBeInTheDocument();
  });
});
```

## Store Testing

Mock SDK stores in tests:

```typescript
import { vi } from "vitest";

vi.mock("@orqastudio/sdk", () => ({
  getStores: () => ({
    projectStore: { hasProject: true },
  }),
}));
```

## Enforcement

Test requirements are defined in coding standards rules with enforcement entries for the `vitest` tool. The configurator agent generates `vitest.config.ts` from these entries.
