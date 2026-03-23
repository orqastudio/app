/**
 * @orqastudio/plugin-coding-standards
 *
 * Unified coding standards enforcement:
 * - Config generator: reads enforcement rules → generates tool config files
 * - Check runner: runs configured tools via plugin executors
 * - Org sync: propagates standards across projects
 */

export { ConfigGenerator, type GeneratedConfig } from "./config-generator.js";
export { CheckRunner, type CheckResult, type CheckSummary } from "./check-runner.js";
