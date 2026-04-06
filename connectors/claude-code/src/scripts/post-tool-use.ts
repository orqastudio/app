#!/usr/bin/env node
/**
 * PostToolUse hook — Write|Edit, TaskUpdate
 *
 * Thin daemon wrapper: after a tool completes, notifies the daemon to validate
 * written artifacts against the composed schema, track task completions, and
 * update telemetry. No business logic here — all decisions are made by the daemon.
 */

import { readInput, callDaemon, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the PostToolUse hook. */
async function main(): Promise<void> {
	const input = await readInput();

	const toolName = input.tool_name ?? "";
	const toolInput = input.tool_input ?? {};

	const context = {
		event: "PostAction" as const,
		tool_name: toolName,
		tool_input: toolInput,
		file_path: toolInput.file_path ?? "",
	};

	let result: HookResult;
	try {
		result = await callDaemon<HookResult>("/hook", context);
	} catch {
		process.exit(0);
	}

	if (result.messages?.length > 0) {
		outputWarn(result.messages);
	}

	process.exit(0);
}

main().catch(() => process.exit(0));
