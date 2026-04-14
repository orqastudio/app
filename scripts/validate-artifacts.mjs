#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';
import { execSync } from 'child_process';
import YAML from 'yaml';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const projectRoot = path.resolve(__dirname, '..');
const orqaDir = path.join(projectRoot, '.orqa');
const schemaPath = path.join(orqaDir, 'schema.composed.json');

// Configuration
const modes = {
  DEFAULT: 'default',
  STAGED: 'staged',
  HOOK: 'hook',
  SUMMARY: 'summary'
};

let currentMode = modes.DEFAULT;
let showSummary = false;

// Parse command-line arguments
for (const arg of process.argv.slice(2)) {
  if (arg === '--staged') currentMode = modes.STAGED;
  if (arg === '--hook') currentMode = modes.HOOK;
  if (arg === '--summary') showSummary = true;
}

// Load schema
let schema;
let relationshipTypeMap = {};
try {
  schema = JSON.parse(fs.readFileSync(schemaPath, 'utf-8'));
  // Build lookup map from relationshipTypes array
  if (Array.isArray(schema.relationshipTypes)) {
    schema.relationshipTypes.forEach(rt => {
      relationshipTypeMap[rt.key] = rt;
    });
  }
} catch (err) {
  console.error(`ERROR: Failed to load schema from ${schemaPath}`);
  console.error(err.message);
  process.exit(2);
}

/**
 * Parse YAML frontmatter from a markdown file
 * Returns { frontmatter, body }
 */
function parseFrontmatter(content) {
  const lines = content.split('\n');
  if (lines[0] !== '---') {
    return { frontmatter: {}, body: content };
  }

  let endIndex = -1;
  for (let i = 1; i < lines.length; i++) {
    if (lines[i] === '---') {
      endIndex = i;
      break;
    }
  }

  if (endIndex === -1) {
    return { frontmatter: {}, body: content };
  }

  const frontmatterText = lines.slice(1, endIndex).join('\n');
  const body = lines.slice(endIndex + 1).join('\n');

  try {
    const parsed = YAML.parse(frontmatterText) || {};
    return { frontmatter: parsed, body };
  } catch (err) {
    return { frontmatter: {}, body: content, parseError: err.message };
  }
}

/**
 * Get all artifact files to validate
 */
function getArtifactFiles() {
  if (currentMode === modes.STAGED || currentMode === modes.HOOK) {
    // Skip validation if environment variable is set (for --hook mode)
    if (currentMode === modes.HOOK && process.env.ORQA_SKIP_SCHEMA_VALIDATION) {
      return [];
    }

    try {
      const output = execSync('git diff --cached --name-only', { 
        cwd: projectRoot,
        encoding: 'utf-8'
      });
      return output
        .trim()
        .split('\n')
        .filter(f => f.endsWith('.md') && f.startsWith('.orqa/'))
        .map(f => path.join(projectRoot, f));
    } catch (err) {
      console.error('ERROR: Failed to get staged files');
      process.exit(2);
    }
  }

  // Default mode: find all .md files in .orqa/
  const files = [];
  function walkDir(dir) {
    try {
      const entries = fs.readdirSync(dir, { withFileTypes: true });
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        if (entry.isDirectory()) {
          walkDir(fullPath);
        } else if (entry.isFile() && entry.name.endsWith('.md')) {
          files.push(fullPath);
        }
      }
    } catch (err) {
      // Skip directories we can't read
    }
  }
  walkDir(orqaDir);
  return files;
}

/**
 * Build an index of all artifact IDs
 * Returns a Set of all valid artifact IDs in .orqa/
 */
function buildArtifactIndex(files) {
  const ids = new Set();
  for (const file of files) {
    try {
      const content = fs.readFileSync(file, 'utf-8');
      const { frontmatter } = parseFrontmatter(content);
      if (frontmatter.id) {
        ids.add(frontmatter.id);
      }
    } catch (err) {
      // Skip files we can't read
    }
  }
  return ids;
}

/**
 * Get all valid statuses for a type
 */
function getValidStatuses(typeConfig) {
  const statuses = new Set();
  
  // Add statuses from the statuses array
  if (typeConfig.statuses && Array.isArray(typeConfig.statuses)) {
    typeConfig.statuses.forEach(s => statuses.add(s));
  }
  
  // Add statuses from stateCategories (includes terminal states)
  if (typeConfig.stateCategories && typeof typeConfig.stateCategories === 'object') {
    for (const categoryStatuses of Object.values(typeConfig.stateCategories)) {
      if (Array.isArray(categoryStatuses)) {
        categoryStatuses.forEach(s => statuses.add(s));
      }
    }
  }
  
  // Also check fields.required.status.enum for completeness
  if (typeConfig.fields?.required?.status?.enum && Array.isArray(typeConfig.fields.required.status.enum)) {
    typeConfig.fields.required.status.enum.forEach(s => statuses.add(s));
  }
  
  return Array.from(statuses).sort();
}

/**
 * Count approximate tokens (word count * 1.3)
 */
function estimateTokens(text) {
  const words = text.trim().split(/\s+/).length;
  return Math.ceil(words * 1.3);
}

/**
 * Validate a single artifact file
 */
function validateArtifact(filePath, artifactIndex) {
  const errors = [];
  const warnings = [];

  let content;
  try {
    content = fs.readFileSync(filePath, 'utf-8');
  } catch (err) {
    errors.push({
      category: 'file-read',
      message: `Cannot read file: ${err.message}`
    });
    return { errors, warnings };
  }

  const { frontmatter, body, parseError } = parseFrontmatter(content);

  if (parseError) {
    errors.push({
      category: 'frontmatter-parse',
      message: `Failed to parse YAML frontmatter: ${parseError}`
    });
    return { errors, warnings };
  }

  // 1. Check required frontmatter fields
  const id = frontmatter.id;
  const type = frontmatter.type;

  if (!id) {
    errors.push({
      category: 'required-field',
      field: 'id',
      message: 'Missing required field: id'
    });
  }

  if (!type) {
    errors.push({
      category: 'required-field',
      field: 'type',
      message: 'Missing required field: type'
    });
  }

  // If we have both id and type, continue validation
  if (id && type) {
    // 2. Validate ID format
    if (!schema.artifactTypes[type]) {
      warnings.push({
        category: 'unknown-type',
        message: `Unknown artifact type: ${type}`
      });
    } else {
      const typeConfig = schema.artifactTypes[type];
      
      // Use the pattern from fields.required.id if available, otherwise use top-level id_pattern
      const idPatternStr = typeConfig.fields?.required?.id?.pattern || typeConfig.id_pattern;
      const idPattern = new RegExp(idPatternStr);
      
      if (!idPattern.test(id)) {
        errors.push({
          category: 'id-format',
          message: `ID does not match pattern ${idPatternStr}: ${id}`
        });
      }

      // 3. Check type-location consistency
      const relativeDir = path.relative(orqaDir, path.dirname(filePath));
      const expectedPath = typeConfig.default_path;

      if (expectedPath) {
        // Extract the path part after .orqa/ and remove trailing slash
        // e.g., ".orqa/discovery/ideas/" -> "discovery/ideas"
        const expectedRelative = expectedPath
          .replace(/^\.orqa\//, '')
          .replace(/\/$/, '');

        let isMatch = false;

        // Special handling for 'knowledge' type: allow knowledge/ subdirectory anywhere under documentation/
        if (type === 'knowledge') {
          isMatch = relativeDir.startsWith('documentation/') && relativeDir.includes('/knowledge');
        } else {
          // Check if actual directory matches expected location
          // Allows both exact match and subdirectory match
          // e.g., "doc" type expects "documentation" and allows "documentation/guides", "documentation/architecture", etc.
          isMatch = relativeDir === expectedRelative ||
                   relativeDir.startsWith(expectedRelative + '/');
        }

        if (!isMatch) {
          warnings.push({
            category: 'location-mismatch',
            message: `Type '${type}' should be in directory '${expectedRelative}' or subdirectory, found in '${relativeDir}'`
          });
        }
      }

      // 4. Validate status field for stateful types
      if (typeConfig.fields?.required?.status) {
        const status = frontmatter.status;
        if (!status) {
          errors.push({
            category: 'required-field',
            field: 'status',
            message: 'Missing required field: status'
          });
        } else {
          // Get all valid statuses from statuses array and stateCategories
          const validStatuses = getValidStatuses(typeConfig);
          if (validStatuses.length > 0 && !validStatuses.includes(status)) {
            errors.push({
              category: 'invalid-status',
              message: `Invalid status '${status}'. Valid statuses: ${validStatuses.join(', ')}`
            });
          }
        }
      }

      // 5. Check knowledge size constraints (read from schema, not hardcoded)
      if (type === 'knowledge') {
        const sizeConstraints = typeConfig?.size_constraints;
        if (sizeConstraints && sizeConstraints.unit === 'tokens') {
          const tokens = estimateTokens(body);
          const minT = sizeConstraints.min_tokens;
          const maxT = sizeConstraints.max_tokens;
          if (typeof minT === 'number' && tokens < minT) {
            warnings.push({
              category: 'knowledge-size',
              message: `Knowledge artifact too short (~${tokens} tokens, expected ${minT}-${maxT})`
            });
          } else if (typeof maxT === 'number' && tokens > maxT) {
            warnings.push({
              category: 'knowledge-size',
              message: `Knowledge artifact too long (~${tokens} tokens, expected ${minT}-${maxT})`
            });
          }
        }
      }
    }
  }

  // 6. Validate relationship targets
  const relationships = frontmatter.relationships || [];
  if (Array.isArray(relationships)) {
    for (const rel of relationships) {
      if (!rel.target) {
        errors.push({
          category: 'relationship-incomplete',
          message: 'Relationship missing target field'
        });
      } else if (!artifactIndex.has(rel.target)) {
        errors.push({
          category: 'relationship-target-missing',
          target: rel.target,
          message: `Relationship target does not exist: ${rel.target}`
        });
      }

      if (!rel.type) {
        errors.push({
          category: 'relationship-incomplete',
          message: 'Relationship missing type field'
        });
      } else if (!relationshipTypeMap[rel.type]) {
        warnings.push({
          category: 'unknown-relationship-type',
          message: `Unknown relationship type: ${rel.type}`
        });
      }
    }
  }

  return { errors, warnings };
}

/**
 * Main validation logic
 */
function main() {
  const files = getArtifactFiles();

  if (files.length === 0) {
    if (currentMode === modes.STAGED || currentMode === modes.HOOK) {
      process.exit(0); // No staged files, all good
    }
  }

  // Build artifact index for fast lookups
  const artifactIndex = buildArtifactIndex(files);

  const allErrors = {};
  const allWarnings = {};
  let totalErrors = 0;
  let totalWarnings = 0;

  for (const file of files) {
    const { errors, warnings } = validateArtifact(file, artifactIndex);
    const relPath = path.relative(projectRoot, file);

    if (errors.length > 0 || warnings.length > 0) {
      if (errors.length > 0) {
        if (!allErrors[relPath]) allErrors[relPath] = [];
        allErrors[relPath].push(...errors);
        totalErrors += errors.length;
      }
      if (warnings.length > 0) {
        if (!allWarnings[relPath]) allWarnings[relPath] = [];
        allWarnings[relPath].push(...warnings);
        totalWarnings += warnings.length;
      }
    }
  }

  // Output results
  if (showSummary) {
    const categories = {};
    
    for (const fileErrors of Object.values(allErrors)) {
      for (const err of fileErrors) {
        const cat = err.category;
        categories[cat] = (categories[cat] || 0) + 1;
      }
    }

    for (const fileWarnings of Object.values(allWarnings)) {
      for (const warn of fileWarnings) {
        const cat = 'warning:' + warn.category;
        categories[cat] = (categories[cat] || 0) + 1;
      }
    }

    console.log('\n=== VALIDATION SUMMARY ===');
    for (const [cat, count] of Object.entries(categories).sort()) {
      console.log(`  ${cat}: ${count}`);
    }
    console.log(`\nTotal errors: ${totalErrors}`);
    console.log(`Total warnings: ${totalWarnings}`);
  } else {
    // Detailed output
    if (totalErrors > 0) {
      console.log('\n=== ERRORS ===\n');
      for (const [file, errors] of Object.entries(allErrors).sort()) {
        console.log(`${file}`);
        for (const err of errors) {
          console.log(`  - [${err.category}] ${err.message}`);
        }
        console.log();
      }
    }

    if (totalWarnings > 0) {
      console.log('\n=== WARNINGS ===\n');
      for (const [file, warnings] of Object.entries(allWarnings).sort()) {
        console.log(`${file}`);
        for (const warn of warnings) {
          console.log(`  - [${warn.category}] ${warn.message}`);
        }
        console.log();
      }
    }

    if (totalErrors === 0 && totalWarnings === 0) {
      console.log('✓ All artifacts validated successfully');
    } else {
      console.log(`\nTotal errors: ${totalErrors}, warnings: ${totalWarnings}`);
    }
  }

  // Exit with appropriate code
  if (totalErrors > 0) {
    process.exit(1);
  }
  process.exit(0);
}

main();
