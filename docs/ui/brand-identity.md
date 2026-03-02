# Forge Dual-Mode Identity Guidelines (v1)

**Date:** 2026-03-02 | **Status:** Authoritative brand direction

Forge uses one logo family with two render modes:

- **Product Mode (Industrial Minimal):** neutral, legible, dynamic across contexts
- **Marketing Mode (Cyber-Technical):** energetic, molten, high-contrast storytelling

This is one brand with a controlled "style switch," not two identities.

---

## 1. Brand Invariants (Never Change)

These rules apply in both modes.

### Logo Family (3 Marks)

| Mark | Name | Usage |
|------|------|-------|
| **Forge Anvil** | Primary brand mark | Hero/default mark |
| **Forge Frame-F** | Minimal / enterprise / small-size mark | Nav, favicons, docs |
| **Forge Agent-Strike** | Orchestration / agents / execution mark | Agent UI surfaces |

### Shared Geometry

- Same spark wedge shape across all marks (the brand "signature")
- Same corner radius language (rounded-square, not circles)
- Same proportions for each mark (don't redraw per use)

### Typography

| Role | Font Stack | CSS Variable |
|------|-----------|--------------|
| UI + headings | Inter | `--font-sans` |
| Code / mono | JetBrains Mono (or IBM Plex Mono) | `--font-mono` |

Full font stack declarations (from [design-tokens research](/research/design-tokens)):

```css
:root {
  --font-sans: Inter, Roboto, 'Helvetica Neue', 'Arial Nova', 'Nimbus Sans',
    Arial, sans-serif;
  --font-mono: 'JetBrains Mono', 'Cascadia Code', 'Fira Code', Menlo, Monaco,
    'Courier New', monospace;
}
```

### Palette Semantics

- **Ember / heat** = action + execution + impact
- **Frame / box** = governance boundaries
- **Graph / links** = orchestration + delegation
- **Check / verify** states must be visually clear and consistent

---

## 2. Render Modes

### A) Product Mode — Industrial Minimal (in-app default)

**Intent:** calm, governed, adaptable, readable at 16-24px.

**Allowed styling:**
- Flat 1-color mark (preferred)
- Optional single solid spark accent
- No gradients, no glow, no bevel, no inner shadow

**Default colors:**

| Context | Mark Color | Maps to CSS Variable |
|---------|-----------|---------------------|
| On dark UI | `#E5E7EB` | `--foreground` (dark mode) |
| On light UI | `#0B0F14` | `--foreground` (light mode) |

**Spark usage:**
- Spark can be omitted at micro sizes
- If included: use solid accent only

**Best mark by size:**

| Size | Recommended Mark |
|------|-----------------|
| 16-24px | Frame-F |
| 24-64px | Frame-F or simplified Anvil |
| 64px+ | Anvil lockups are safe |

### B) Marketing Mode — Cyber-Technical (web/launch/media)

**Intent:** heat + momentum + precision. Show the "forge" energy.

**Allowed styling:**
- Heated metal gradient on spark (and strike line on Agent-Strike)
- Subtle glow (tight radius, low spread)
- Optional metallic shading on the main body (restrained)

**Gradient rule:** Gradients live on spark/strike, not everywhere. The logo must still work in monochrome.

**Composition motifs:**
- UI artifact cards (ADR / RULE / IMPL / RETRO)
- Grids, scan lines, node graphs
- Single bright "impact point" (spark)

---

## 3. Context Mapping (What Goes Where)

| Context | Product Mode | Marketing Mode |
|---------|-------------|----------------|
| App icon / store listing / installer | Anvil (flat) | Anvil (heated spark) |
| Nav + small UI icons (16-24px) | Frame-F (flat) | — |
| Docs, governance pages, enterprise decks | Frame-F (flat) | Frame-F with subtle heated spark |
| Agents module / orchestration surfaces | Agent-Strike (flat) | Agent-Strike with heated strike + spark impact |

---

## 4. Dynamic Branding (Product Mode Only)

Forge can safely "theme" itself via the spark without changing the mark.

### Contextual Spark Color Mapping

| Context | Spark Color | Hex | CSS Variable |
|---------|------------|-----|-------------|
| Governance / Rules | Arc Blue | `#4CC9F0` | `--forge-arc` |
| Verify / Scanners (pass) | Success | `#22C55E` | `--forge-success` |
| Verify / Scanners (fail) | Error | `#EF4444` | `--destructive` |
| Agents / Execution | Ember | `#FF4D2E` | `--forge-ember` |
| Learning Loop | Amber | `#FFB020` | `--forge-amber` |

**Hard rule:** Only the spark (or strike line) changes color. The mark body remains monochrome.

---

## 5. Do / Don't

### Do

- Keep spark shape identical across all three marks
- Test every mark at 24px in 1-color
- Use glow/gradient only in marketing mode
- Prefer Frame-F for tiny sizes

### Don't

- Don't add gradients/glow inside the product UI
- Don't recolor the whole mark dynamically
- Don't redraw per campaign — reuse the master geometry

---

## 6. Token Specification

All brand tokens below are mapped to the shadcn-svelte CSS variable convention established in [design-tokens research](/research/design-tokens). Tokens that map directly to existing shadcn-svelte variables use those names. Tokens that extend the base set use the `--forge-*` namespace.

### Token Mapping

| Brand Token | Hex | OKLCH (approx.) | CSS Variable | Shadcn Role |
|-------------|-----|-----------------|-------------|-------------|
| **Core Colors** | | | | |
| `forge.obsidian` | `#0B0F14` | `oklch(0.13 0.01 260)` | `--background` (dark) | Dark mode base surface |
| `forge.text.primary` | `#E5E7EB` | `oklch(0.92 0.003 260)` | `--foreground` (dark) | Primary text on dark |
| `forge.text.secondary` | `#A3AAB8` | `oklch(0.72 0.015 260)` | `--muted-foreground` (dark) | Secondary/muted text on dark |
| **Accents** | | | | |
| `forge.ember` | `#FF4D2E` | `oklch(0.64 0.27 25)` | `--forge-ember` | Action/execution spark color |
| `forge.amber` | `#FFB020` | `oklch(0.80 0.17 75)` | `--forge-amber` | Learning loop spark color |
| `forge.arc` | `#4CC9F0` | `oklch(0.78 0.12 220)` | `--forge-arc` | Governance spark color |
| **Status** | | | | |
| `forge.success` | `#22C55E` | `oklch(0.72 0.19 150)` | `--forge-success` | Pass/success state |
| `forge.warning` | `#F59E0B` | `oklch(0.79 0.17 75)` | `--forge-warning` | Warning state |
| `forge.error` | `#EF4444` | `oklch(0.63 0.24 25)` | `--destructive` | Error/fail state (reuses shadcn) |
| `forge.info` | `#38BDF8` | `oklch(0.76 0.13 230)` | `--forge-info` | Informational state |

### Logo Mode Switch Tokens

| Brand Token | Value | CSS Variable | Description |
|-------------|-------|-------------|-------------|
| `forge.brand.mode` | `"product"` \| `"marketing"` | `--forge-brand-mode` | Active render mode |
| **Product mode** | | | |
| `forge.logo.body` | `--foreground` (contextual) | `--forge-logo-body` | Mark body color (obsidian on light, text.primary on dark) |
| `forge.logo.spark` | contextColor \| `--forge-ember` | `--forge-logo-spark` | Spark accent (default: ember) |
| **Marketing mode** | | | |
| `forge.logo.spark.gradient` | `[--forge-amber → --forge-ember → deepRed]` | — | Spark gradient (CSS/SVG only) |
| `forge.logo.spark.glow` | `on` (subtle) | — | Spark glow effect (CSS/SVG only) |

### Relationship to shadcn-svelte Base Variables

The Forge brand layer **extends** the shadcn-svelte variable set rather than replacing it. The complete CSS variable hierarchy:

```
shadcn-svelte base (30 variables)     ← Component library contract
  ├── --background, --foreground       ← Brand core colors map here
  ├── --primary, --primary-foreground  ← Forge's UI accent (from branding research Q1)
  ├── --destructive                    ← Brand error maps here
  ├── --muted-foreground               ← Brand text.secondary maps here
  └── ... (all other shadcn vars)

Forge brand extensions (custom)        ← Brand-specific tokens
  ├── --forge-ember                    ← Contextual spark: execution
  ├── --forge-amber                    ← Contextual spark: learning
  ├── --forge-arc                      ← Contextual spark: governance
  ├── --forge-success                  ← Status: pass/success
  ├── --forge-warning                  ← Status: warning
  ├── --forge-info                     ← Status: informational
  ├── --forge-logo-body                ← Logo body color
  ├── --forge-logo-spark               ← Logo spark color
  └── --forge-brand-mode               ← product | marketing
```

### Concrete CSS Declaration

```css
:root {
  /* Forge brand extensions — light mode */
  --forge-ember: oklch(0.64 0.27 25);
  --forge-amber: oklch(0.80 0.17 75);
  --forge-arc: oklch(0.78 0.12 220);
  --forge-success: oklch(0.72 0.19 150);
  --forge-warning: oklch(0.79 0.17 75);
  --forge-info: oklch(0.76 0.13 230);
  --forge-logo-body: oklch(0.145 0 0);    /* obsidian on light */
  --forge-logo-spark: var(--forge-ember);  /* default spark */
}

.dark {
  /* Forge brand extensions — dark mode */
  --forge-ember: oklch(0.64 0.27 25);     /* unchanged — already vibrant */
  --forge-amber: oklch(0.80 0.17 75);
  --forge-arc: oklch(0.78 0.12 220);
  --forge-success: oklch(0.72 0.19 150);
  --forge-warning: oklch(0.79 0.17 75);
  --forge-info: oklch(0.76 0.13 230);
  --forge-logo-body: oklch(0.92 0.003 260); /* text.primary on dark */
  --forge-logo-spark: var(--forge-ember);
}
```

> **Note:** The shadcn-svelte base variables (`--background`, `--foreground`, `--primary`, etc.) are declared separately in the Forge default theme — see [branding research Q1](/research/branding) for the complete light/dark theme declarations. The `--forge-*` variables above are additive.

---

## 7. Deliverables Checklist

| Deliverable | Variants | Format |
|-------------|----------|--------|
| Master SVGs | Anvil / Frame-F / Agent-Strike (monochrome) | SVG |
| Product exports | 16, 24, 32, 48, 64, 128, 256, 512, 1024 | PNG |
| Marketing variants | Same marks with heated spark | SVG + PNG |
| App assets | macOS + Windows icons | `.icns` + `.ico` |
| Shared spark wedge | Single reusable component | SVG |

---

## Relationship to Other Documents

| Document | Relationship |
|----------|-------------|
| [Branding Research](/research/branding) | Implementation strategy for three-layer branding (Q1-Q4). This document provides the authoritative brand direction that informs Q1. |
| [Design Tokens Research](/research/design-tokens) | Token format, storage, runtime application, extraction pipeline. Defines the CSS variable convention this document maps into. |
| [Frontend Research](/research/frontend) | shadcn-svelte component library selection. Establishes the base variable contract. |
