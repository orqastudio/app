/**
 * Plugin registry — fetch and cache official + community plugin catalogs.
 *
 * Both registries are JSON files hosted in GitHub repos.
 * Cached in memory with a 1-hour TTL.
 */
import type { RegistryCatalog, RegistryEntry } from "@orqastudio/types";
/**
 * Fetch a plugin registry catalog.
 * @param source - "official", "community", or "all" (returns merged)
 * @returns The fetched registry catalog.
 */
export declare function fetchRegistry(source?: "official" | "community" | "all"): Promise<RegistryCatalog>;
/**
 * Search the registry for plugins matching a query.
 * @param query - Search string matched against name, displayName, description, and category.
 * @param source - Registry source to search: "official", "community", or "all".
 * @returns Array of matching registry entries.
 */
export declare function searchRegistry(query: string, source?: "official" | "community" | "all"): Promise<RegistryEntry[]>;
//# sourceMappingURL=registry.d.ts.map