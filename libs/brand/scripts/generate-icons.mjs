#!/usr/bin/env node
/**
 * generate-icons.mjs — Read icons.json, render SVGs to PNG/ICO at specified sizes.
 *
 * Usage:
 *   node scripts/generate-icons.mjs              Generate all icons
 *   node scripts/generate-icons.mjs --deploy     Generate + copy to deploy targets
 *   node scripts/generate-icons.mjs --deploy web-app   Generate + copy to specific target
 *
 * Reads icons.json from the brand lib root. Each entry maps an SVG source to
 * output files with format, size, and purpose. Add new entries to icons.json
 * and re-run to generate.
 */

import { readFileSync, writeFileSync, copyFileSync, mkdirSync, existsSync } from "node:fs";
import { resolve, dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const BRAND_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const DEV_ROOT = resolve(BRAND_ROOT, "../..");

async function main() {
  const args = process.argv.slice(2);
  const deploy = args.includes("--deploy");
  const deployTarget = args[args.indexOf("--deploy") + 1]; // optional specific target

  // Dynamic import sharp (installed in brand lib)
  let sharp;
  try {
    sharp = (await import("sharp")).default;
  } catch {
    console.error("sharp not installed. Run: cd libs/brand && npm install");
    process.exit(1);
  }

  const config = JSON.parse(readFileSync(join(BRAND_ROOT, "icons.json"), "utf-8"));
  let generated = 0;

  for (const icon of config.icons) {
    if (!icon.outputs || icon.outputs.length === 0) {
      console.log(`  skip  ${icon.source} (no outputs defined)`);
      continue;
    }

    const svgPath = join(BRAND_ROOT, icon.source);
    if (!existsSync(svgPath)) {
      console.error(`  ERROR  SVG not found: ${svgPath}`);
      continue;
    }

    const svgBuffer = readFileSync(svgPath);
    console.log(`\n  ${icon.source} → ${icon.purpose}`);

    for (const output of icon.outputs) {
      const outPath = join(BRAND_ROOT, output.path);
      mkdirSync(dirname(outPath), { recursive: true });

      if (output.format === "svg") {
        // Just copy the SVG
        copyFileSync(svgPath, outPath);
        console.log(`    ✓ ${output.path} (svg copy)`);
        generated++;
      } else if (output.format === "png") {
        const png = await sharp(svgBuffer, { density: 300 })
          .resize(output.size, output.size, { fit: "contain", background: { r: 0, g: 0, b: 0, alpha: 0 } })
          .png()
          .toBuffer();
        writeFileSync(outPath, png);
        console.log(`    ✓ ${output.path} (${output.size}x${output.size} png)`);
        generated++;
      } else if (output.format === "ico") {
        // ICO = multiple PNG sizes packed together
        // sharp doesn't write ICO natively, so we build one manually
        const sizes = output.sizes || [16, 32, 48];
        const pngBuffers = [];

        for (const size of sizes) {
          const png = await sharp(svgBuffer, { density: 300 })
            .resize(size, size, { fit: "contain", background: { r: 0, g: 0, b: 0, alpha: 0 } })
            .png()
            .toBuffer();
          pngBuffers.push({ size, buffer: png });
        }

        const ico = buildIco(pngBuffers);
        writeFileSync(outPath, ico);
        console.log(`    ✓ ${output.path} (ico: ${sizes.join("+")}px)`);
        generated++;
      }
    }
  }

  console.log(`\n  ${generated} icon(s) generated.`);

  // Deploy to targets
  if (deploy && config.deployTargets) {
    console.log("\n  Deploying to targets...\n");
    const targets = deployTarget
      ? { [deployTarget]: config.deployTargets[deployTarget] }
      : config.deployTargets;

    for (const [name, target] of Object.entries(targets)) {
      if (!target) {
        console.error(`  ERROR  Unknown deploy target: ${name}`);
        continue;
      }
      console.log(`  ${name}: ${target.description}`);
      for (const copy of target.copy) {
        const from = join(BRAND_ROOT, copy.from);
        const to = join(DEV_ROOT, copy.to);
        if (!existsSync(from)) {
          console.error(`    ✗ ${copy.from} (source not found)`);
          continue;
        }
        mkdirSync(dirname(to), { recursive: true });
        copyFileSync(from, to);
        console.log(`    ✓ ${copy.from} → ${copy.to}`);
      }
    }
  }
}

/**
 * Build an ICO file from multiple PNG buffers.
 * ICO format: header (6 bytes) + directory entries (16 bytes each) + PNG data.
 */
function buildIco(pngBuffers) {
  const numImages = pngBuffers.length;
  const headerSize = 6;
  const dirEntrySize = 16;
  const dirSize = dirEntrySize * numImages;

  // Calculate offsets
  let dataOffset = headerSize + dirSize;
  const entries = pngBuffers.map(({ size, buffer }) => {
    const entry = { size, buffer, offset: dataOffset };
    dataOffset += buffer.length;
    return entry;
  });

  // Total file size
  const totalSize = dataOffset;
  const ico = Buffer.alloc(totalSize);

  // Header: reserved (0), type (1=icon), count
  ico.writeUInt16LE(0, 0);      // reserved
  ico.writeUInt16LE(1, 2);      // type: icon
  ico.writeUInt16LE(numImages, 4); // count

  // Directory entries
  entries.forEach((entry, i) => {
    const pos = headerSize + i * dirEntrySize;
    ico.writeUInt8(entry.size >= 256 ? 0 : entry.size, pos);     // width (0 = 256)
    ico.writeUInt8(entry.size >= 256 ? 0 : entry.size, pos + 1); // height
    ico.writeUInt8(0, pos + 2);          // color palette
    ico.writeUInt8(0, pos + 3);          // reserved
    ico.writeUInt16LE(1, pos + 4);       // color planes
    ico.writeUInt16LE(32, pos + 6);      // bits per pixel
    ico.writeUInt32LE(entry.buffer.length, pos + 8);  // size of PNG data
    ico.writeUInt32LE(entry.offset, pos + 12);         // offset to PNG data
  });

  // PNG data
  for (const entry of entries) {
    entry.buffer.copy(ico, entry.offset);
  }

  return ico;
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
