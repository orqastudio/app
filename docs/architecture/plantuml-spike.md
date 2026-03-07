---
title: "PlantUML Bundling Spike"
category: architecture
tags: []
created: 2026-03-02
updated: 2026-03-04
---

# PlantUML Bundling Spike

**Date:** 2026-03-02 | **Status:** Phase 0e specification
**References:** [Wireframing Research](/research/wireframing) (OQ1: Java Runtime Dependency), [Wireframe Serving Infrastructure](/architecture/wireframe-serving)

A time-boxed investigation into eliminating or bundling PlantUML's Java dependency so that wireframe rendering works out of the box for all OrqaStudio™ users.

---

## The Problem

PlantUML Salt is OrqaStudio's primary wireframing tool (decided in [wireframing research](/research/wireframing)). Wireframing is a first-class product feature, not a developer-only build tool. Every OrqaStudio user must be able to render wireframes without manual setup.

PlantUML is a Java application distributed as a JAR file (~10MB). It requires a Java Runtime Environment (JRE 8+) to execute. OrqaStudio is a Tauri desktop application targeting Windows, macOS, and Linux. Shipping OrqaStudio with a "please install Java first" prerequisite is unacceptable for the following reasons:

1. **UX friction** — Users installing a desktop app expect it to work immediately. A Java installation prompt is a deal-breaker.
2. **Support burden** — Java version mismatches, PATH configuration issues, and platform-specific JRE packaging would generate a disproportionate support load.
3. **Brand perception** — OrqaStudio positions itself as a polished, modern development tool. Requiring Java installation undermines that perception.
4. **Reliability** — Depending on the user's system Java means OrqaStudio cannot guarantee a specific Java version or configuration. Silent failures are likely.

This spike evaluates four options for resolving the dependency, determines which to prototype first, and defines acceptance criteria for the spike.

---

## Option A: GraalVM Native Image

Compile `plantuml.jar` into a standalone native binary using GraalVM's `native-image` ahead-of-time (AOT) compiler. The output is a platform-specific executable with no JRE dependency.

### How It Works

GraalVM's `native-image` performs static analysis of the JAR's bytecode, resolves all reachable code paths at build time, and compiles them to machine code. The result is a self-contained binary that includes a minimal runtime (SubstrateVM) and does not require a JRE.

### Steps to Validate

1. **Download PlantUML JAR:**
   ```bash
   curl -L -o plantuml.jar https://github.com/plantuml/plantuml/releases/download/v1.2025.2/plantuml-1.2025.2.jar
   ```

2. **Install GraalVM (latest, with native-image):**
   ```bash
   # macOS (Homebrew)
   brew install --cask graalvm-jdk

   # Windows (Scoop or manual download)
   scoop install graalvm

   # Linux (SDKMAN)
   sdk install java 24.ea-graal
   ```

3. **Run native-image with tracing agent (to detect reflection):**
   ```bash
   # Step 1: Run PlantUML under the tracing agent to record reflection/resource usage
   java -agentlib:native-image-agent=config-output-dir=ni-config \
        -jar plantuml.jar -tpng test-salt.puml

   # Step 2: Compile with the generated configuration
   native-image \
        -jar plantuml.jar \
        -H:ConfigurationFileDirectories=ni-config \
        -H:+ReportExceptionStackTraces \
        --no-fallback \
        -o plantuml-native
   ```

4. **Test the native binary with Salt specifically:**
   ```bash
   ./plantuml-native -tpng test-salt.puml
   ./plantuml-native -tsvg test-salt.puml
   ./plantuml-native -pipe -tpng < test-salt.puml > output.png
   ```

5. **Test with styled Salt (theme integration):**
   ```bash
   # Verify that <style> blocks, skinparam, colors, and fonts work
   ./plantuml-native -tpng styled-salt.puml
   ```

### Expected Binary Size

- Bare native-image with SubstrateVM: ~15-20MB base
- PlantUML with graphics dependencies: ~30-40MB
- Compressed for distribution: ~15-20MB

This is smaller than a bundled JRE (Option B) and produces a single file.

### Known Issues

#### Reflection

GraalVM native-image performs closed-world analysis. Any class loaded via `Class.forName()`, `Method.invoke()`, or similar reflection APIs must be declared in a reflection configuration file. PlantUML uses reflection in several places:

- **Diagram type registration** — PlantUML discovers diagram types (sequence, class, salt, etc.) dynamically. The tracing agent should capture these.
- **Charset handling** — Character encoding lookups use reflection internally in the JDK.
- **Font enumeration** — `java.awt.GraphicsEnvironment.getAvailableFontFamilyNames()` uses reflection.

The tracing agent (Step 3 above) records all reflection calls exercised during the test run. The key is to ensure the test run exercises Salt-specific code paths.

#### AWT and Graphics

This is the most significant risk. PlantUML renders images using Java's AWT/Java2D graphics stack:

- `java.awt.image.BufferedImage` — image buffer creation
- `java.awt.Graphics2D` — drawing operations (lines, rectangles, text)
- `java.awt.Font` — font loading and text measurement
- `java.awt.Color` — color handling
- `javax.imageio.ImageIO` — PNG encoding

GraalVM's native-image has limited AWT support. The situation as of GraalVM 24/25:

- **BufferedImage and Graphics2D**: Supported on Linux and macOS via GraalVM's AWT substrate. Windows support is more recent and may have gaps.
- **Font loading**: Requires system fonts to be accessible. The native binary does not bundle fonts. If the system has no fonts installed (unusual but possible in containers), rendering fails.
- **ImageIO PNG writer**: Generally works but must be included in the reflection config.
- **SVG output**: PlantUML's SVG output path does not use AWT for rasterization — it generates SVG XML directly. This code path is more likely to compile cleanly.

#### PlantUML Salt Specifically

Salt uses the same AWT rendering pipeline as other PlantUML diagrams. Salt-specific concerns:

- **Does Salt use reflection beyond standard PlantUML?** Unlikely. Salt's element parsing (`[Button]`, `"TextField"`, `{/ Tabs}`) is handled by Salt-specific parser classes, not reflection. The tracing agent will confirm.
- **Does Salt use AWT?** Yes, for all image rendering. Salt's layout engine computes positions, then draws via Graphics2D. SVG output follows a different code path that writes XML strings.
- **OpenIconic icons** (`<&icon_name>`) — These are bundled as resources inside the PlantUML JAR. native-image must be configured to include them (`-H:IncludeResources` or via resource config from the tracing agent).

### Risk Assessment

| Factor | Risk | Mitigation |
|--------|------|------------|
| AWT compilation | Medium-High | SVG-only output as fallback; test on all three platforms |
| Reflection config completeness | Medium | Tracing agent + manual review of PlantUML source |
| Cross-platform builds | Medium | CI matrix: build native binary on each target OS |
| Binary size | Low | 30-40MB is acceptable; smaller than JRE bundle |
| Startup time | Low-Positive | Native binary starts in <50ms vs 1-2s for JVM |

### Verdict

**Best-case outcome if it works.** Smallest footprint, fastest execution, no runtime dependency. Worth trying first, but AWT on native-image is the primary risk factor. SVG output has a higher chance of working than PNG.

---

## Option B: Bundled Minimal JRE

Ship a stripped-down Java runtime alongside OrqaStudio. Use `jlink` to create a custom JRE containing only the modules PlantUML requires.

### How jlink Works

Since Java 9, the JDK is modular. `jlink` creates a custom runtime image containing only specified modules and their transitive dependencies. The result is a self-contained `bin/java` with a minimal footprint.

### Required Modules for PlantUML

```bash
jlink \
  --add-modules java.base,java.desktop,java.logging,java.xml,java.datatransfer \
  --strip-debug \
  --no-man-pages \
  --no-header-files \
  --compress=zip-6 \
  --output jre-minimal
```

Module breakdown:
- `java.base` — core JDK (always required)
- `java.desktop` — AWT, Graphics2D, ImageIO, font handling (required for PNG rendering)
- `java.logging` — PlantUML uses `java.util.logging`
- `java.xml` — SVG output uses XML APIs
- `java.datatransfer` — transitive dependency of `java.desktop`

### Expected Size

| Configuration | Uncompressed | Compressed |
|--------------|-------------|-----------|
| Full JRE (Eclipse Temurin) | ~180MB | ~60MB |
| jlink minimal (modules above) | ~90MB | ~45MB |
| jlink minimal + `--compress=zip-9` | ~90MB | ~38MB |

The minimal JRE adds approximately 38-50MB to OrqaStudio's installer size.

### Cross-Platform Considerations

A jlink'd JRE is platform-specific. OrqaStudio must bundle a different JRE for each target:

| Platform | JRE Variant | Notes |
|----------|------------|-------|
| Windows x64 | Temurin JRE 21 LTS, Windows x64 | Most common OrqaStudio target |
| Windows ARM64 | Temurin JRE 21 LTS, Windows aarch64 | Surface Pro X, etc. |
| macOS x64 | Temurin JRE 21 LTS, macOS x64 | Intel Macs |
| macOS aarch64 | Temurin JRE 21 LTS, macOS aarch64 | Apple Silicon (primary Mac target) |
| Linux x64 | Temurin JRE 21 LTS, Linux x64 | Primary Linux target |
| Linux aarch64 | Temurin JRE 21 LTS, Linux aarch64 | Raspberry Pi, ARM servers |

Each platform's jlink output is built in CI and included in that platform's Tauri installer bundle.

### Tauri Integration

The JRE is bundled as a sidecar resource:

```
src-tauri/
  resources/
    jre/          <-- jlink output (platform-specific, populated by CI)
      bin/
        java      <-- or java.exe on Windows
      lib/
        ...
    plantuml.jar
```

Tauri's `tauri.conf.json` includes the resource directory:

```json
{
  "bundle": {
    "resources": ["resources/jre/**", "resources/plantuml.jar"]
  }
}
```

OrqaStudio invokes PlantUML via:

```bash
{app_resources}/jre/bin/java -jar {app_resources}/plantuml.jar -tpng input.puml
```

### JRE Version

Use Java 21 LTS (Eclipse Temurin). LTS ensures long-term availability of security patches. Temurin is the community successor to AdoptOpenJDK, with permissive licensing (GPLv2 + Classpath Exception).

### Risk Assessment

| Factor | Risk | Mitigation |
|--------|------|------------|
| Installer size increase | Medium | 38-50MB; acceptable for a desktop app |
| Cross-platform CI builds | Low-Medium | Automated jlink in CI matrix; one-time setup |
| JVM startup time | Medium | Use `-pipe` mode to keep JVM warm; startup ~1-2s |
| Font availability | Low | Uses system fonts; standard on all desktop OSes |
| AWT rendering | Low | Full JRE includes complete AWT; this is the known-good path |

### Verdict

**Most reliable option.** Full AWT support is guaranteed because we are shipping a real JRE. The cost is installer size (~40MB) and JVM startup latency. This is the safe fallback if native-image (Option A) fails.

---

## Option C: Detect System JRE

Use whatever `java` binary is available on the user's PATH. No bundling.

### Detection Logic

```rust
fn find_system_java() -> Option<PathBuf> {
    // 1. Check JAVA_HOME
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        let java = PathBuf::from(java_home).join("bin").join("java");
        if java.exists() {
            return Some(java);
        }
    }

    // 2. Check PATH
    if let Ok(output) = Command::new("java").arg("-version").output() {
        if output.status.success() {
            return Some(PathBuf::from("java"));
        }
    }

    None
}
```

### Version Requirements

- Minimum: Java 8 (PlantUML's official minimum)
- Recommended: Java 11+
- Optimal: Java 17+ (better startup, lower memory)

Version detection:

```bash
java -version 2>&1 | head -1
# openjdk version "21.0.2" 2024-01-16
```

### When This Option Applies

This is not a primary distribution strategy. It serves as:

1. **Development fallback** — During OrqaStudio development, contributors use their own JRE
2. **Graceful degradation** — If the bundled native binary or JRE is corrupted/missing, attempt system Java before declaring failure
3. **Lightweight installs** — Users who already have Java and want a smaller OrqaStudio download could opt out of the bundled JRE (future advanced installer option)

### Risk Assessment

| Factor | Risk | Mitigation |
|--------|------|------------|
| Java not installed | High | This is why we cannot rely on this option alone |
| Wrong Java version | Medium | Version check + clear error message |
| PATH misconfiguration | Medium | Check JAVA_HOME first, then PATH |
| Platform inconsistency | Medium | Java on PATH behaves differently across OSes |

### Verdict

**Not acceptable as the primary strategy.** Useful as the last link in a fallback chain. Zero additional distribution cost.

---

## Option D: WebAssembly (WASM)

Compile PlantUML (or a PlantUML-compatible renderer) to WebAssembly and run it directly in Tauri's WebView.

### Current State

Several projects attempt to bring PlantUML to the browser:

- **plantuml-wasm** (https://github.com/nickg/nickg.github.io/tree/master/plantuml) — Community effort. Uses CheerpJ to transpile PlantUML's Java bytecode to JavaScript/WASM. Limited diagram type support.
- **CheerpJ** (https://cheerpj.com/) — Commercial Java-to-WebAssembly compiler by Leaning Technologies. Can run unmodified JARs in the browser. Adds ~20-40MB of runtime. Commercial license required for production use.
- **TeaVM** — Java-to-JavaScript/WASM AOT compiler. Open source. Would require PlantUML to be compatible with TeaVM's subset of java.* APIs.

### Salt Support Status

PlantUML Salt support in WASM environments is **unverified and likely incomplete**:

- Salt rendering depends on AWT for image generation. WASM environments do not have AWT.
- CheerpJ emulates AWT using HTML5 Canvas. This may work for basic rendering but font metrics and layout calculations could differ from desktop AWT.
- SVG output (which bypasses AWT rasterization) has a better chance of working in WASM.
- No known project has specifically validated PlantUML Salt in a WASM environment.

### Integration Approach (If Pursued)

```typescript
// In Tauri's WebView frontend
import { PlantUMLWasm } from 'plantuml-wasm';

const renderer = await PlantUMLWasm.init();
const svgString = await renderer.render(saltSource);
// Display SVG directly in the DOM — no file I/O needed
```

This would be attractive because:
- No external process spawning
- No file system I/O for rendering
- Runs in the same process as the UI
- Cross-platform by definition (WASM is portable)

### Risk Assessment

| Factor | Risk | Mitigation |
|--------|------|------------|
| Salt rendering correctness | High | No one has validated Salt in WASM |
| AWT emulation fidelity | High | Font metrics, layout may differ |
| CheerpJ licensing | Medium | Commercial license for production use |
| WASM runtime size | Medium | 20-40MB of CheerpJ runtime |
| Maintenance burden | High | OrqaStudio would depend on third-party Java-to-WASM tooling |
| Performance | Unknown | Could be slower than native; could be faster than JVM cold start |

### Verdict

**Not ready for production.** The WASM approach is conceptually appealing but practically unproven for PlantUML Salt. The AWT dependency is the same blocker as native-image, but with less mature tooling. Revisit if GraalVM native-image (Option A) and bundled JRE (Option B) both fail, or if the plantuml-wasm ecosystem matures significantly.

---

## Recommendation

### Primary Strategy: Try Option A First, Fall Back to Option B

```
Option A (GraalVM native-image)
  |
  |-- Success? Ship native binary per platform
  |
  |-- Failure (AWT issues)?
  |     |
  |     |-- Try SVG-only mode with native binary
  |     |     |
  |     |     |-- Success? Ship native binary, SVG only, convert to PNG via
  |     |     |   image crate in Rust if PNG needed
  |     |     |
  |     |     |-- Failure? Move to Option B
  |     |
  |     v
  |   Option B (bundled minimal JRE via jlink)
  |     |
  |     |-- Ship jlink'd JRE + plantuml.jar per platform
  |
  v
Runtime fallback chain (regardless of primary):
  1. Bundled native binary (if Option A succeeded)
  2. Bundled JRE + JAR (if Option B was adopted, or as fallback)
  3. System java on PATH (Option C, always available as last resort)
```

### Why This Order

1. **Option A (native-image) has the best outcome** if it works: smallest binary, fastest startup, simplest distribution. The risk is AWT/graphics support, which is specifically what this spike validates.

2. **Option B (bundled JRE) is the safe fallback** with guaranteed AWT support. The cost (40-50MB installer size increase) is acceptable for a desktop application. This is the option we are most confident will work.

3. **Option C (system JRE) costs nothing to implement** and should always be in the fallback chain for development environments and edge cases.

4. **Option D (WASM) is deferred.** The ecosystem is not mature enough for production use with PlantUML Salt. If the WASM situation improves (particularly if an official PlantUML WASM build emerges with Salt support), it can be reconsidered.

---

## Spike Acceptance Criteria

The spike is considered successful when the following conditions are met. Each criterion has a clear pass/fail determination.

### Must-Pass Criteria

1. **Salt PNG rendering** — At least one option (A or B) produces a correct PNG image from a PlantUML Salt source file containing: buttons, text fields, tabs, a nested grid layout, and OpenIconic icons. The output must be visually correct (elements positioned properly, text readable, icons rendered).

2. **Salt SVG rendering** — Same Salt source as criterion 1 produces a correct SVG output.

3. **Style block support** — A `<style>` block prepended to the Salt source correctly changes background color, font color, and line color in the rendered output. All three variants (light, dark, brand) render with visually distinct themes.

4. **Pipe mode** — PlantUML can accept Salt source on stdin and write image data to stdout (for the warm-process optimization described in the wireframe serving spec).

5. **Cross-platform build** — The chosen option (A or B) builds successfully on at least two platforms: one of {macOS aarch64, macOS x64} and one of {Windows x64, Linux x64}.

6. **Automated build** — The build process for the PlantUML binary/JRE bundle can be scripted (for CI integration). No manual GUI steps.

### Should-Pass Criteria

7. **Rendering performance** — A typical wireframe (20-30 Salt elements) renders in under 2 seconds on commodity hardware, including any cold-start cost.

8. **Binary/bundle size** — The distributable artifact (native binary or jlink'd JRE + JAR) is under 50MB compressed.

9. **Font rendering** — Text in the rendered wireframe uses a readable sans-serif font on all tested platforms (does not fall back to a bitmap or monospace font).

10. **Three-platform build** — The chosen option builds on all three primary platforms: macOS, Windows, and Linux.

### Spike Output Artifacts

- A working build script (shell or CI YAML) that produces the PlantUML artifact
- A test Salt source file exercising all UI element types
- Rendered PNG and SVG samples from the test file in all three style variants
- A short findings document recording: which option was selected, binary size, render time, any platform-specific issues encountered, and any PlantUML features that do not work

### Time Box

**3 working days.** If Option A (native-image) does not produce correct Salt PNG output within 2 days, switch to Option B (bundled JRE) for the remaining day. The spike must produce a working solution within the time box, even if it is Option B.
