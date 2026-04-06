/**
 * Dev environment — thin CLI dispatch layer.
 *
 * `orqa dev` is the primary entry point for the development environment.
 * Process management (build, watch, restart, service lifecycle) is handled by
 * ProcessManager in lib/process-manager.ts. This file is a dispatch layer only.
 *
 * orqa dev                        Launch OrqaDev (cargo tauri dev for devtools)
 * orqa dev start-processes        Build + start all dev processes (called by OrqaDev)
 * orqa dev stop                   Stop gracefully via signal file
 * orqa dev kill                   Force-kill all processes
 * orqa dev restart [target]       Send restart signal to running controller
 * orqa dev status                 Show process status
 * orqa dev graph                  Print the dependency graph build tiers
 * orqa dev icons [--deploy]       Generate brand icons from SVG sources
 * orqa dev tool [args...]         Run the debug-tool submodule
 */
/**
 * Dispatch the dev command to the appropriate subcommand handler.
 * @param args - Positional arguments passed after `orqa dev`, used to select the subcommand.
 */
export declare function runDevCommand(args: string[]): Promise<void>;
//# sourceMappingURL=dev.d.ts.map