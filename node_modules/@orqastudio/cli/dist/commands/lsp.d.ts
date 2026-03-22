/**
 * LSP proxy command — connects to the running OrqaStudio app's IPC socket
 * and bridges stdin/stdout ↔ TCP for LSP protocol messages.
 *
 * If the app isn't running, falls back to spawning orqa-studio --lsp directly.
 *
 * orqa lsp [project-path]
 */
export declare function runLspCommand(args: string[]): Promise<void>;
//# sourceMappingURL=lsp.d.ts.map