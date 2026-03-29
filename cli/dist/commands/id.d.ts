/**
 * ID management command — generate, check, and migrate artifact IDs.
 *
 * orqa id generate <type>        Generate a new hex ID
 * orqa id check                  Scan graph for duplicate IDs
 * orqa id migrate <old> <new>    Rename an ID across the entire graph
 */
/**
 * Dispatch the id command: generate, check, or migrate artifact IDs.
 * @param args - CLI arguments after "id".
 */
export declare function runIdCommand(args: string[]): Promise<void>;
//# sourceMappingURL=id.d.ts.map