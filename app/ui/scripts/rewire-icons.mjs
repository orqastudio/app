/**
 * Rewires Lucide icon imports to use the library's Icon component.
 * Run from ui/ directory: node scripts/rewire-icons.mjs
 */

import { readFileSync, writeFileSync } from "fs";
import { execSync } from "child_process";

// Find all svelte files with lucide imports (excluding ui/ and shared/)
const files = execSync(
  `grep -rl '@lucide/svelte/icons/' --include='*.svelte' src/lib/components/ src/routes/ | grep -v '/ui/' | grep -v '/shared/'`,
  { encoding: "utf-8" }
).trim().split("\n").filter(Boolean);

console.log(`Found ${files.length} files to process`);

// Size mapping: Tailwind class → Icon size prop
const sizeMap = {
  "h-3 w-3": "xs",
  "h-3.5 w-3.5": "sm",
  "h-4 w-4": "md",
  "h-5 w-5": "lg",
  "h-6 w-6": "xl",
  "h-8 w-8": "xl",
  "h-10 w-10": "xl",
  "h-12 w-12": "xl",
};

// Convert PascalCase icon import name to kebab-case icon key
// e.g. "ChevronRightIcon" → "chevron-right", "CircleAlertIcon" → "circle-alert"
function iconNameToKey(importName) {
  // Remove trailing "Icon" if present
  let name = importName.replace(/Icon$/, "");
  // Handle special cases like "LoaderCircle" → "loader-circle"
  // PascalCase → kebab-case
  return name
    .replace(/([a-z0-9])([A-Z])/g, "$1-$2")
    .replace(/([A-Z])([A-Z][a-z])/g, "$1-$2")
    .toLowerCase();
}

let totalImportsRemoved = 0;
let totalTagsReplaced = 0;

for (const file of files) {
  let content = readFileSync(file, "utf-8");
  const original = content;

  // Find all lucide icon imports
  const importRegex = /\timport\s+(\w+)\s+from\s+"@lucide\/svelte\/icons\/([a-z0-9-]+)";\n/g;
  const imports = [];
  let match;
  while ((match = importRegex.exec(content)) !== null) {
    imports.push({ full: match[0], varName: match[1], iconKey: match[2] });
  }

  if (imports.length === 0) continue;

  // Remove all lucide imports
  for (const imp of imports) {
    content = content.replace(imp.full, "");
    totalImportsRemoved++;
  }

  // Check if Icon is already imported
  const hasIconImport = content.includes("Icon") && content.includes("@orqastudio/svelte-components");

  // Add Icon import if needed
  if (!hasIconImport) {
    // Find the right place to add it — after existing @orqastudio imports or at top of script
    const orqaImportMatch = content.match(/(\timport\s+.*from\s+"@orqastudio\/svelte-components\/pure";\n)/);
    if (orqaImportMatch) {
      // Check if Icon is already in this import
      if (!orqaImportMatch[1].includes("Icon")) {
        // Add Icon to existing import
        content = content.replace(
          orqaImportMatch[1],
          orqaImportMatch[1].replace("import {", "import { Icon,")
        );
      }
    } else {
      // Add new import after first import
      const firstImport = content.match(/(\t?import\s+.*;\n)/);
      if (firstImport) {
        content = content.replace(
          firstImport[0],
          firstImport[0] + '\timport { Icon } from "@orqastudio/svelte-components/pure";\n'
        );
      }
    }
  }

  // Replace icon component usages in template
  for (const imp of imports) {
    const varName = imp.varName;
    const iconKey = imp.iconKey;

    // Pattern 1: <IconName class="h-N w-N ..." />
    const selfClosingRegex = new RegExp(
      `<${varName}\\s+class="([^"]*)"\\s*/>`,
      "g"
    );
    content = content.replace(selfClosingRegex, (match, classes) => {
      let size = "md";
      for (const [cls, sz] of Object.entries(sizeMap)) {
        if (classes.includes(cls)) {
          size = sz;
          break;
        }
      }
      totalTagsReplaced++;
      return `<Icon name="${iconKey}" size="${size}" />`;
    });

    // Pattern 2: <IconName class="..." /> with extra props
    const propsRegex = new RegExp(
      `<${varName}([^/]*?)/>`,
      "g"
    );
    content = content.replace(propsRegex, (match, props) => {
      if (match.includes(`name="${iconKey}"`)) return match; // Already replaced
      let size = "md";
      const classMatch = props.match(/class="([^"]*)"/);
      if (classMatch) {
        for (const [cls, sz] of Object.entries(sizeMap)) {
          if (classMatch[1].includes(cls)) {
            size = sz;
            break;
          }
        }
      }
      totalTagsReplaced++;
      return `<Icon name="${iconKey}" size="${size}" />`;
    });

    // Pattern 3: <IconName class="..."> (with children, rare)
    // Pattern 4: {IconName} or Icon={IconName} as component prop — replace with string
    const componentPropRegex = new RegExp(`\\b${varName}\\b`, "g");
    // Only replace in non-import, non-already-replaced contexts
    // This handles cases like icon={ShieldIcon} → we can't just replace, need manual review
  }

  if (content !== original) {
    writeFileSync(file, content);
    console.log(`${file}: removed ${imports.length} imports`);
  }
}

console.log(`\nDone: ${totalImportsRemoved} imports removed, ${totalTagsReplaced} tags replaced across ${files.length} files`);
