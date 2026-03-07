![License](https://img.shields.io/badge/license-CC--BY--ND--4.0-lightgrey)

![Orqa Studio](assets/banners/banner-1680x240.png)

# Orqa Studio — Brand Guidelines

## 1. Brand Overview

**Orqa Studio** is a platform for orchestrating intelligent software development.

The brand visual language reflects three core ideas:

* **Orchestration** — coordinating intelligent systems
* **Signal & insight** — sensing and learning from systems
* **Direction** — guiding execution through governance

The logo combines two symbols:

| Element         | Meaning                                   |
| --------------- | ----------------------------------------- |
| **Sonar rings** | sensing, feedback loops, system awareness |
| **Fin**         | direction, intelligence, orchestration    |

Together they represent the core cycle of the platform:

```text
scan → insight → execution
```

---

## 2. Logo

### Primary Mark

The primary mark consists of:

* **Two concentric sonar rings**
* **A dorsal fin rising from the center**

The fin represents decisive intelligent direction emerging from system awareness.

#### Visual Principles

The logo should always remain:

* **minimal**
* **geometric**
* **balanced**
* **easily scalable**

Avoid decorative or illustrative treatments.

---

## 3. Logo Geometry

The logo is designed on a **1024 × 1024 unit grid**.

Suggested proportions:

| Element           | Value       |
| ----------------- | ----------- |
| Canvas            | 1024 × 1024 |
| Outer ring radius | ~420        |
| Inner ring radius | ~300        |
| Ring stroke       | ~70         |
| Fin base width    | ~260        |

### Alignment

The **fin tip should align with the vertical center axis** of the rings.

This ensures the mark feels balanced and stable.

---

## 4. Logo Usage

### Preferred Usage

* dark background
* sonar rings and fin in **signal cyan**

#### Example

```text
Dark background
Cyan mark
```

---

### Light Background Version

For light surfaces:

* logo switches to **deep navy**

Avoid cyan on very light backgrounds when contrast is insufficient.

---

## 5. Wordmark

When the logo is paired with text, use the **OrqaStudio** wordmark — a single compound word with no space.

![Wordmark reference](assets/banners/banner-1680x240.png)

### Construction

| Property | Value |
| --- | --- |
| Typeface | Inter |
| **Orqa** | Bold (700) |
| **Studio** | Light (300) |
| Casing | PascalCase — **O**rqa**S**tudio |
| Spacing | No space between words |

### Sizing & Placement

* The wordmark height should sit **just under** the height of the logo mark
* Leave generous horizontal spacing between the logo and the wordmark — at minimum the width of the logo mark itself
* Vertically centre the wordmark against the logo

### Implementation

```css
.wordmark {
  font-family: 'Inter', sans-serif;
  font-size: /* match to logo height */;
  letter-spacing: -0.02em;
}

.wordmark-orqa {
  font-weight: 700;
}

.wordmark-studio {
  font-weight: 300;
}
```

```html
<span class="wordmark">
  <span class="wordmark-orqa">Orqa</span><span class="wordmark-studio">Studio</span>
</span>
```

### Don'ts

* Do not add a space between "Orqa" and "Studio"
* Do not use a single font weight for the full wordmark
* Do not set the wordmark larger than the logo height
* Do not place the wordmark too close to the logo — maintain breathing room

---

## 6. Colour Palette

### Primary Brand Colours

#### Signal Cyan

```css
#00D1FF
```

Used for:

* logo
* highlights
* signal effects
* key UI accents

---

#### Deep Ocean

```css
#0B132B
```

Primary background colour.

Represents depth and intelligence.

---

#### Midnight Navy

```css
#1C2541
```

Secondary dark tone for UI surfaces.

---

#### Ice White

```css
#E6F7FF
```

Used sparingly for highlights and light themes.

---

## 7. Typography

### Primary UI Typeface

> **Inter**

Used for:

* UI text
* documentation
* marketing content

Characteristics:

* neutral
* highly readable
* modern technical tone

---

### Monospace Typeface

> **JetBrains Mono**

Used for:

* code
* CLI examples
* technical interfaces

---

## 8. Iconography

Icons follow the same geometry principles as the logo.

Guidelines:

* circular or radial layouts
* simple strokes
* geometric shapes
* no unnecessary detail

Icons should feel like **components of the sonar system**.

---

## 9. Motion Principles

Animation reinforces the platform’s learning loop.

Typical motion pattern:

```text
pulse → detect → direct
```

Examples:

| Event           | Motion                   |
| --------------- | ------------------------ |
| Agent execution | expanding sonar pulse    |
| rule detection  | brief cyan signal flash  |
| learning update | ripple expanding outward |

Motion should feel **calm and intelligent**, never flashy.

---

## 10. Backgrounds

Preferred background styles:

* deep gradients
* subtle radial lighting
* calm ocean-inspired tones

Avoid:

* noisy textures
* heavy neon effects
* overly complex patterns

The brand should feel **precise and intelligent**.

---

## 11. Voice & Tone

The Orqa Studio voice is:

* clear
* precise
* calm
* technically confident

Avoid hype-driven AI marketing language.

Prefer language that reflects **engineering discipline and system thinking**.

Example:

### Good

> Orchestrate intelligent development.

### Avoid

> Next-generation revolutionary AI coding platform.

---

## 12. Design Principles

All brand expressions should reflect:

1. **Clarity**
2. **Precision**
3. **System thinking**
4. **Calm intelligence**

The design should always feel like **engineering infrastructure**, not consumer AI branding.

---

## 13. Summary

The Orqa Studio brand expresses a simple idea:

> Intelligent systems require awareness and direction.

The sonar represents awareness.
The fin represents direction.

Together they form a symbol for **orchestrated intelligence**.
