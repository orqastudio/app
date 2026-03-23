# Motion Assets

Animated SVG variants of the OrqaStudio logo for use in loading states, splash screens, and transitions.

---

## logo-pulse.svg

![logo-pulse](logo-pulse.svg)

A looping sonar pulse animation. Each ring draws itself along its elliptical path as it appears, then all three hold together before fading out. The fin remains static as an anchor point.

| Property | Value |
|---|---|
| Cycle duration | 3s |
| Easing | ease-in-out |
| Loop | infinite |
| Technique | CSS `@keyframes` + SVG masks (no JS required) |

### Timing

| Phase | Frames | What happens |
|---|---|---|
| Ripple in | 0–45% | Rings stroke-draw sequentially — inner, middle, then outer. Each ring traces its elliptical path as it fades in. |
| Hold | 45–65% | All three rings fully visible together |
| Fade out | 65–80% | All rings fade out simultaneously |
| Pause | 80–100% | All rings invisible before the next cycle |

Each ring uses a masked `stroke-dashoffset` animation to create the draw effect, synced with its opacity fade-in. Rings go fully to `opacity: 0` between pulses.

### Usage

**Inline in HTML:**

```html
<img src="/assets/motion/logo-pulse.svg" alt="Loading..." width="64" height="64">
```

**As a React component:**

```tsx
function LoadingSpinner({ size = 64 }: { size?: number }) {
  return (
    <img
      src="/assets/motion/logo-pulse.svg"
      alt="Loading..."
      width={size}
      height={size}
    />
  );
}
```

**In CSS as a background:**

```css
.loading {
  width: 64px;
  height: 64px;
  background: url('/assets/motion/logo-pulse.svg') no-repeat center;
  background-size: contain;
}
```

**Inline SVG (for full CSS control):**

If you need to override timing or colours, embed the SVG directly in your markup. The animation classes are:

- `.ring-inner` / `.draw-inner` — first ring (opacity + stroke draw)
- `.ring-middle` / `.draw-middle` — second ring
- `.ring-outer` / `.draw-outer` — third ring

To change the speed, override all animation durations together:

```css
.ring-inner, .ring-middle, .ring-outer,
.draw-inner, .draw-middle, .draw-outer {
  animation-duration: 4s; /* slower pulse */
}
```

### Recommended sizes

| Context | Size |
|---|---|
| Inline loading spinner | 32–64px |
| Page-level loading state | 96–128px |
| Splash screen | 256px+ |

### Browser support

Uses CSS animations inside SVG, supported in all modern browsers. When loaded via `<img>`, the animation runs automatically with no JavaScript. For older browsers that don't support SVG CSS animations, the logo renders static at full opacity as a graceful fallback.
