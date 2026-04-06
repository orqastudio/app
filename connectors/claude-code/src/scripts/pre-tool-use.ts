#!/usr/bin/env node
/**
 * PreToolUse hook — Write|Edit|Bash
 *
 * Thin daemon wrapper: validates artifact operations and file access enforcement
 * before tool execution. The daemon checks agent role permissions, artifact
 * schema compliance for .orqa/ writes, and rule engine evaluation for the
 * proposed action. No business logic here — all decisions are made by the daemon.
 */

import { readInput, callDaemon, outputBlock, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the PreToolUse hook. */
async function main(): Promise<void> {
	const input = await readInput();

	const toolName = input.tool_name ?? "";
	const toolInput = input.tool_input ?? {};
	const agentType = input.agent_type ?? "orchestrator";

	const context = {
		event: "PreAction" as const,
		tool_name: toolName,
		tool_input: toolInput,
		file_path: toolInput.file_path ?? toolInput.command ?? "",
		agent_type: agentType,
	};

	let result: HookResult;
	try {
		result = await callDaemon<HookResult>("/hook", context);
	} catch {
		// Daemon unavailable — fail-open (session-start blocks sessions without daemon)
		process.exit(0);
	}

	if (result.action === "block") {
		const messages = result.messages?.length
			? result.messages
			: ["Action blocked by governance rules."];
		outputBlock(messages);
	}

	if (result.action === "warn" && result.messages?.length > 0) {
		outputWarn(result.messages);
	}

	process.exit(0);
}

main().catch(() => process.exit(0));
