#!/usr/bin/env node
/**
 * SubagentStop hook — all matchers
 *
 * Thin daemon wrapper: reviews subagent output when a subagent completes.
 * The daemon checks for stub detection (incomplete implementations), deferral
 * scanning (silently deferred acceptance criteria), artifact integrity, and
 * findings file validation. No business logic here — all decisions are made
 * by the daemon.
 */

import { readInput, callDaemon, outputWarn } from "../hooks/shared.js";
import type { HookResult } from "../hooks/shared.js";

/** Run the SubagentStop hook. */
async function main(): Promise<void> {
	const input = await readInput();

	const agentType = input.agent_type ?? "unknown";

	const context = {
		event: "SubagentStop" as const,
		agent_type: agentType,
	};

	let result: HookResult;
	try {
		result = await callDaemon<HookResult>("/hook", context);
	} catch {
		// Daemon unavailable — fail-open
		process.exit(0);
	}

	if (result.messages?.length > 0) {
		outputWarn(result.messages);
	}

	process.exit(0);
}

main().catch(() => process.exit(0));
