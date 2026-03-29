/**
 * LSP command — spawns the standalone orqa-lsp-server binary over stdio.
 *
 * The binary handles the LSP protocol and connects to the validation daemon
 * (HTTP) for diagnostics. The daemon should already be running via `orqa dev`.
 *
 * Architecture: IDE → orqa lsp → orqa-lsp-server (stdio) → daemon (HTTP)
 *
 * orqa lsp [project-path]
 */
/**
 * Dispatch the lsp command: spawn the orqa-lsp-server binary over stdio.
 * @param args - CLI arguments after "lsp".
 */
export declare function runLspCommand(args: string[]): Promise<void>;
//# sourceMappingURL=lsp.d.ts.map