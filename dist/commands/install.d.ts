/**
 * Install commands — dev environment bootstrapping.
 *
 * orqa install              Full setup (prereqs + submodules + build-all + verify)
 * orqa install prereqs      Check and install prerequisites (node 22+, rust)
 * orqa install submodules   Init and update git submodules
 * orqa install deps         Install package dependencies (npm install + cargo fetch)
 * orqa install link         Build libs and npm link into app (requires deps published)
 * orqa install publish      Publish all libs to GitHub Package Registry
 *
 * The full install (no subcommand) uses a unified pipeline that processes each
 * lib in dependency order: install → build → publish → link. This ensures each
 * lib's @orqastudio/* deps are on the registry before the next lib needs them.
 */
export declare function runInstallCommand(args: string[]): Promise<void>;
//# sourceMappingURL=install.d.ts.map