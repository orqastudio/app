#!/usr/bin/env node

/**
 * CLI tool entry point — one-shot process.
 *
 * Reads arguments, performs work, outputs results to stdout.
 * Exit code 0 = success, non-zero = failure.
 */

const args = process.argv.slice(2);
const projectRoot = args[0] ?? process.cwd();

console.log(JSON.stringify({
	tool: "my-tool",
	status: "ok",
	message: `Ran against ${projectRoot}`,
}));
