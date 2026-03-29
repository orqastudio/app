---
id: KNOW-40e2eb99
type: knowledge
title: TypeScript Advanced Types
domain: platform/typescript
description: "Master TypeScript's advanced type system including generics, conditional types, mapped types, template literals, and utility types for building type-safe applications. Use when implementing complex type logic, creating reusable type utilities, or ensuring compile-time type safety in TypeScript projects."
tier: on-demand
summary: "Master TypeScript's advanced type system including generics, conditional types, mapped types, template literals, and utility types for building type-safe applications. Use when implementing complex type logic, creating reusable type utilities, or ensuring compile-time type safety in TypeScript projects."
status: active
created: 2026-03-01
updated: 2026-03-10
category: domain
user-invocable: false
relationships:
  - target: DOC-7062bce9
    type: synchronised-with
---

TypeScript advanced type system patterns: generics, conditional types, mapped types, template literals, discriminated unions, and type guards.

## Generics

```typescript
// Constrained generic
function logLength<T extends { length: number }>(item: T): T {
  console.log(item.length);
  return item;
}

// Multiple type params
function merge<T, U>(obj1: T, obj2: U): T & U {
  return { ...obj1, ...obj2 };
}
```

## Conditional Types

```typescript
type IsString<T> = T extends string ? true : false;

// Infer keyword — extract return type
type ReturnType<T> = T extends (...args: any[]) => infer R ? R : never;

// Distributive
type ToArray<T> = T extends any ? T[] : never;
type Result = ToArray<string | number>; // string[] | number[]
```

## Mapped Types

```typescript
// Key remapping
type Getters<T> = {
  [K in keyof T as `get${Capitalize<string & K>}`]: () => T[K];
};

// Filter by value type
type PickByType<T, U> = {
  [K in keyof T as T[K] extends U ? K : never]: T[K];
};
```

## Template Literal Types

```typescript
type EventName = "click" | "focus" | "blur";
type EventHandler = `on${Capitalize<EventName>}`; // "onClick" | "onFocus" | "onBlur"
```

## Discriminated Unions

```typescript
type AsyncState<T> =
  | { status: "success"; data: T }
  | { status: "error"; error: string }
  | { status: "loading" };

function handle<T>(state: AsyncState<T>) {
  switch (state.status) {
    case "success": return state.data;   // narrowed
    case "error": return state.error;     // narrowed
  }
}
```

## Type Guards and Assertions

```typescript
function isString(value: unknown): value is string {
  return typeof value === "string";
}

function assertIsString(value: unknown): asserts value is string {
  if (typeof value !== "string") throw new Error("Not a string");
}
```

## Deep Recursive Types

```typescript
type DeepReadonly<T> = {
  readonly [P in keyof T]: T[P] extends object
    ? T[P] extends Function ? T[P] : DeepReadonly<T[P]>
    : T[P];
};

type DeepPartial<T> = {
  [P in keyof T]?: T[P] extends object
    ? T[P] extends Array<infer U> ? Array<DeepPartial<U>> : DeepPartial<T[P]>
    : T[P];
};
```

## Built-in Utility Types

| Type | Purpose |
| ------ | --------- |
| `Partial\<T\>` | All properties optional |
| `Required\<T\>` | All properties required |
| `Readonly\<T\>` | All properties readonly |
| `Pick\<T, K\>` | Select specific properties |
| `Omit\<T, K\>` | Remove specific properties |
| `Record\<K, T\>` | Object with keys K, values T |
| `Exclude\<T, U\>` | Remove types from union |
| `Extract\<T, U\>` | Keep types from union |
| `NonNullable\<T\>` | Remove null/undefined |

## Rules

- Use `unknown` over `any` — enforce type checking
- Prefer `interface` for objects, `type` for unions
- Use discriminated unions for state machines
- Use type guards over type assertions
- Avoid deeply nested conditional types (slow compilation)
- Enable all strict compiler options
