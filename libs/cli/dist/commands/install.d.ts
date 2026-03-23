/**
 * Install commands — dev environment bootstrapping.
 *
 * orqa install              Full setup (prereqs + deps + build + plugin sync + verify)
 * orqa install prereqs      Check and install prerequisites (node 22+, rust)
 * orqa install deps         Install npm (workspaces) and cargo dependencies
 * orqa install build        Build all libs in dependency order
 * orqa install publish      Publish all libs to GitHub Package Registry
 *
 * Uses npm workspaces (root package.json) and Cargo workspace (root Cargo.toml).
 * No submodules, no npm link — workspaces resolve all @orqastudio/* packages.
 */
export declare function runInstallCommand(args: string[]): Promise<void>;
//# sourceMappingURL=install.d.ts.map