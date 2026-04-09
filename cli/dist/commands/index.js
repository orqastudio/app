/**
 * Index command — triggers the daemon to index the codebase and generate
 * embeddings for semantic search.
 *
 * The daemon owns search state. This command calls daemon HTTP endpoints
 * instead of running the binary directly. The daemon must be running.
 *
 * orqa index [project-path] [--download-only] [--skip-download] [--status]
 */
import { getPort } from "@orqastudio/constants";
const USAGE = `
Usage: orqa index [project-path] [options]

Trigger the daemon to index the codebase and generate embeddings
for semantic search. The daemon must be running (orqa daemon start).

Options:
  --download-only      Download the ONNX model but do not index or embed
  --skip-download      Skip model download (use existing model on disk)
  --status             Show current search index status and exit
  --help, -h           Show this help message
`.trim();
/**
 * Dispatch the index command: call daemon HTTP endpoints to trigger indexing.
 * @param args - CLI arguments after "index".
 */
export async function runIndexCommand(args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(USAGE);
        return;
    }
    const daemonPort = getPort("daemon");
    const daemonUrl = `http://localhost:${daemonPort}`;
    const projectPath = args.find((a) => !a.startsWith("--")) ?? process.cwd();
    const downloadOnly = args.includes("--download-only");
    const skipDownload = args.includes("--skip-download");
    const statusOnly = args.includes("--status");
    // Check daemon is reachable before proceeding.
    try {
        const healthRes = await fetch(`${daemonUrl}/health`);
        if (!healthRes.ok) {
            throw new Error(`HTTP ${healthRes.status}`);
        }
    }
    catch (err) {
        const msg = err instanceof Error ? err.message : String(err);
        console.error(`Daemon is not running or not reachable on port ${daemonPort}: ${msg}`);
        console.error("Start the daemon first: orqa daemon start");
        process.exit(1);
    }
    if (statusOnly) {
        await getSearchStatus(daemonUrl);
        return;
    }
    console.log(`Project: ${projectPath}`);
    console.log();
    if (!skipDownload) {
        console.log("Downloading ONNX model (if needed)...");
        const embedRes = await fetch(`${daemonUrl}/search/embed`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ download_only: true, project_path: projectPath }),
        });
        if (!embedRes.ok) {
            const body = await embedRes.text().catch(() => "");
            console.error(`Model download failed (HTTP ${embedRes.status}): ${body}`);
            process.exit(1);
        }
        console.log("Model ready.");
    }
    if (downloadOnly) {
        console.log("Done (--download-only).");
        return;
    }
    // Trigger indexing.
    console.log(`Indexing codebase at ${projectPath}...`);
    const indexRes = await fetch(`${daemonUrl}/search/index`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ project_path: projectPath }),
    });
    if (!indexRes.ok) {
        const body = await indexRes.text().catch(() => "");
        console.error(`Indexing failed (HTTP ${indexRes.status}): ${body}`);
        process.exit(1);
    }
    // Trigger embedding generation.
    console.log("Generating embeddings...");
    const embedRes = await fetch(`${daemonUrl}/search/embed`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ project_path: projectPath }),
    });
    if (!embedRes.ok) {
        const body = await embedRes.text().catch(() => "");
        console.error(`Embedding failed (HTTP ${embedRes.status}): ${body}`);
        process.exit(1);
    }
    console.log("\nIndexing complete. Semantic search is ready.");
    await getSearchStatus(daemonUrl);
}
/**
 * Fetch and print the current search index status from the daemon.
 * @param daemonUrl - Base URL of the running daemon.
 */
async function getSearchStatus(daemonUrl) {
    const res = await fetch(`${daemonUrl}/search/status`);
    if (!res.ok) {
        console.error(`Failed to get status (HTTP ${res.status})`);
        return;
    }
    const status = (await res.json());
    console.log("Search status:");
    for (const [key, value] of Object.entries(status)) {
        console.log(`  ${key}: ${value}`);
    }
}
//# sourceMappingURL=index.js.map