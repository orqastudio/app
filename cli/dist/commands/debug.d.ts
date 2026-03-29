/**
 * Debug commands — dev environment + debug tooling.
 *
 * orqa debug                Start the full dev environment (Vite + Tauri)
 * orqa debug stop           Stop gracefully
 * orqa debug kill           Force-kill all processes
 * orqa debug restart        Restart Vite + Tauri (not the controller)
 * orqa debug restart-tauri  Restart Tauri only
 * orqa debug restart-vite   Restart Vite only
 * orqa debug status         Show process status
 * orqa debug icons          Generate brand icons from SVG sources
 * orqa debug tool           Run the debug-tool submodule
 */
export declare function runDebugCommand(args: string[]): Promise<void>;
//# sourceMappingURL=debug.d.ts.map