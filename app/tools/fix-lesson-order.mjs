#!/usr/bin/env node
// ARCHIVED: One-off migration tool — ran once to fix lesson frontmatter field
// ordering to match schema propertyOrder. Not intended for repeat use.
// parseFrontmatter intentionally left as-is (hand-rolled) since this script
// is not maintained going forward.
//
// Fix lesson frontmatter field ordering to match schema propertyOrder.
import { readFileSync, writeFileSync, readdirSync } from "fs";
import { resolve, join } from "path";
import { createRequire } from "module";

const ROOT = resolve(import.meta.dirname, "..");
const require = createRequire(resolve(ROOT, "ui", "package.json"));
const yaml = require("yaml");

const lessonsDir = resolve(ROOT, ".orqa/process/lessons");
const schema = JSON.parse(readFileSync(join(lessonsDir, "schema.json"), "utf-8"));
const propertyOrder = schema.propertyOrder;

for (const file of readdirSync(lessonsDir).sort()) {
  if (!file.startsWith("IMPL-") || !file.endsWith(".md")) continue;
  const filePath = join(lessonsDir, file);
  const content = readFileSync(filePath, "utf-8");
  const normalized = content.replace(/\r\n/g, "\n").replace(/\r/g, "\n");
  const lines = normalized.split("\n");

  let fmEnd = -1, delimCount = 0;
  for (let i = 0; i < lines.length; i++) {
    if (lines[i].trim() === "---") { delimCount++; if (delimCount === 2) { fmEnd = i; break; } }
  }
  if (fmEnd === -1) continue;

  const fmBlock = lines.slice(1, fmEnd).join("\n");
  const body = lines.slice(fmEnd + 1).join("\n");
  let fm;
  try { fm = yaml.parse(fmBlock); } catch { continue; }

  const keys = Object.keys(fm);
  const ptIdx = keys.indexOf("promoted-to");
  const relIdx = keys.indexOf("relationships");
  if (ptIdx === -1 || relIdx === -1 || ptIdx < relIdx) continue;

  const orderedFm = {};
  for (const key of propertyOrder) { if (key in fm) orderedFm[key] = fm[key]; }
  for (const key of Object.keys(fm)) { if (!(key in orderedFm)) orderedFm[key] = fm[key]; }

  const newFmBlock = yaml.stringify(orderedFm, { lineWidth: 0, defaultKeyType: "PLAIN", defaultStringType: "QUOTE_DOUBLE" }).trim();
  writeFileSync(filePath, `---\n${newFmBlock}\n---\n${body}`, "utf-8");
  console.log(`Fixed: ${file}`);
}
