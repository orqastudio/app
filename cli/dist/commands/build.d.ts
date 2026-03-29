/**
 * Build command — production builds for the OrqaStudio app.
 *
 * orqa build            Build the full app (Tauri production bundle)
 * orqa build rust       Build only the Rust backend
 * orqa build app        Build only the frontend
 */
/**
 * Dispatch the build command: full app, Rust only, or frontend only.
 * @param args - CLI arguments after "build".
 */
export declare function runBuildCommand(args: string[]): Promise<void>;
//# sourceMappingURL=build.d.ts.map