/**
 * Resolve the OrqaStudio project root directory.
 *
 * Resolution order:
 * 1. ORQA_ROOT environment variable (explicit override)
 * 2. Walk up from cwd looking for .orqa/ directory (project detection)
 * 3. Fall back to cwd
 */
/**
 * Resolve the OrqaStudio project root, caching the result for subsequent calls.
 * @returns The absolute path to the project root.
 */
export declare function getRoot(): string;
//# sourceMappingURL=root.d.ts.map