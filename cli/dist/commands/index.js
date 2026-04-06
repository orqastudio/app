/**
 * Index command — downloads the ONNX model, indexes the codebase, and
 * generates embeddings for semantic search.
 *
 * orqa index [project-path] [--model-dir <path>] [--download-only] [--skip-download]
 */
import { spawn } from "node:child_process";
import { existsSync } from "node:fs";
import { join, resolve } from "node:path";
import { getRoot } from "../lib/root.js";
const USAGE = `
Usage: orqa index [project-path] [options]

Download the ONNX model, index the codebase, and generate embeddings
for semantic search.

Options:
  --model-dir <path>   Model directory (default: ORQA_MODEL_DIR env var,
                        or models/all-MiniLM-L6-v2/ at project root)
  --download-only      Download the model but do not index or embed
  --skip-download      Skip model download (use existing model on disk)
  --db <path>          DuckDB database path (default: .orqa/search.duckdb)
  --help, -h           Show this help message
`.trim();
function parseOptions(args) {
    const projectPath = args.find((a) => !a.startsWith("--")) ?? getRoot();
    let modelDir = "";
    let dbPath = "";
    let downloadOnly = false;
    let skipDownload = false;
    for (let i = 0; i < args.length; i++) {
        if (args[i] === "--model-dir" && i + 1 < args.length) {
            modelDir = args[i + 1];
            i++;
        }
        else if (args[i] === "--db" && i + 1 < args.length) {
            dbPath = args[i + 1];
            i++;
        }
        else if (args[i] === "--download-only") {
            downloadOnly = true;
        }
        else if (args[i] === "--skip-download") {
            skipDownload = true;
        }
    }
    // Resolve model dir: CLI arg > env var > default
    if (!modelDir) {
        modelDir =
            process.env.ORQA_MODEL_DIR ?? join(resolve(projectPath), "models", "all-MiniLM-L6-v2");
    }
    // Resolve db path: CLI arg > default
    if (!dbPath) {
        dbPath = join(resolve(projectPath), ".orqa", "search.duckdb");
    }
    return { projectPath: resolve(projectPath), modelDir, dbPath, downloadOnly, skipDownload };
}
function findSearchServerManifest(projectPath) {
    const candidates = [
        join(projectPath, "engine", "search", "Cargo.toml"),
        join(projectPath, "..", "engine", "search", "Cargo.toml"),
        join(projectPath, "..", "..", "engine", "search", "Cargo.toml"),
    ];
    for (const candidate of candidates) {
        if (existsSync(candidate))
            return candidate;
    }
    return null;
}
function sendJsonRpc(child, id, method, params) {
    const request = JSON.stringify({
        jsonrpc: "2.0",
        id,
        method,
        params,
    });
    child.stdin?.write(request + "\n");
}
/**
 * Dispatch the index command: download ONNX model and index the codebase for semantic search.
 * @param args - CLI arguments after "index".
 */
export async function runIndexCommand(args) {
    if (args.includes("--help") || args.includes("-h")) {
        console.log(USAGE);
        return;
    }
    const options = parseOptions(args);
    const manifest = findSearchServerManifest(options.projectPath);
    if (!manifest) {
        console.error("Search server crate not found. Ensure engine/search exists.");
        process.exit(1);
    }
    console.log(`Project:   ${options.projectPath}`);
    console.log(`Model dir: ${options.modelDir}`);
    console.log(`Database:  ${options.dbPath}`);
    console.log();
    // Step 1: Download model (unless --skip-download)
    if (!options.skipDownload) {
        const hasModel = existsSync(join(options.modelDir, "model.onnx")) &&
            existsSync(join(options.modelDir, "tokenizer.json"));
        if (hasModel) {
            console.log("Model files already exist, skipping download.");
        }
        else {
            console.log("Downloading BGE-small-en-v1.5 model...");
            const downloadOk = await runSearchServerCommand(manifest, options, [
                { id: 1, method: "download_model", params: { model_dir: options.modelDir } },
            ]);
            if (!downloadOk) {
                console.error("Model download failed.");
                process.exit(1);
            }
            console.log("Model download complete.");
        }
    }
    if (options.downloadOnly) {
        console.log("Done (--download-only).");
        return;
    }
    // Step 2: Index codebase
    console.log(`Indexing codebase at ${options.projectPath}...`);
    // Step 3: Init embedder + embed chunks
    console.log("Initializing embedder and generating embeddings...");
    const commands = [
        {
            id: 1,
            method: "index",
            params: {
                root: options.projectPath,
                excluded: ["node_modules", "target", ".git", "dist", "models"],
            },
        },
        {
            id: 2,
            method: "init_embedder_sync",
            params: { model_dir: options.modelDir },
        },
        {
            id: 3,
            method: "embed_chunks",
            params: {},
        },
        {
            id: 4,
            method: "get_status",
            params: {},
        },
    ];
    const ok = await runSearchServerCommand(manifest, options, commands);
    if (!ok) {
        console.error("Indexing failed.");
        process.exit(1);
    }
    console.log("\nIndexing complete. Semantic search is ready.");
}
async function runSearchServerCommand(manifest, options, commands) {
    return new Promise((resolve) => {
        const child = spawn("cargo", [
            "run",
            "--manifest-path",
            manifest,
            "--bin",
            "orqa-search-server",
            "--",
            "--db",
            options.dbPath,
            "--model-dir",
            options.modelDir,
        ], {
            stdio: ["pipe", "pipe", "inherit"],
            env: {
                ...process.env,
                RUST_LOG: process.env.RUST_LOG ?? "info",
                ORQA_MODEL_DIR: options.modelDir,
            },
        });
        let buffer = "";
        let responsesReceived = 0;
        let hasError = false;
        child.stdout?.on("data", (data) => {
            buffer += data.toString();
            const lines = buffer.split("\n");
            buffer = lines.pop() ?? "";
            for (const line of lines) {
                if (!line.trim())
                    continue;
                try {
                    const response = JSON.parse(line);
                    responsesReceived++;
                    if (response.error) {
                        console.error(`Error (${response.error.code}): ${response.error.message}`);
                        hasError = true;
                    }
                    else if (response.result) {
                        const result = response.result;
                        if (result.chunk_count !== undefined) {
                            console.log(`  Chunks: ${result.chunk_count}, Indexed: ${result.is_indexed}, Embeddings: ${result.has_embeddings}`);
                        }
                        else if (result.downloaded !== undefined) {
                            console.log(`  Downloaded to: ${result.model_dir}`);
                        }
                        else if (typeof result === "boolean" && result) {
                            console.log("  Embedder initialized.");
                        }
                    }
                    // Send next command if we have more
                    if (responsesReceived < commands.length) {
                        sendJsonRpc(child, commands[responsesReceived].id, commands[responsesReceived].method, commands[responsesReceived].params);
                    }
                    else {
                        // All commands done, close stdin to let server exit
                        child.stdin?.end();
                    }
                }
                catch {
                    // Not JSON — ignore (could be partial data)
                }
            }
        });
        child.on("error", (err) => {
            console.error(`Failed to start search server: ${err.message}`);
            resolve(false);
        });
        child.on("close", () => {
            resolve(!hasError);
        });
        // Send the first command
        if (commands.length > 0) {
            sendJsonRpc(child, commands[0].id, commands[0].method, commands[0].params);
        }
    });
}
//# sourceMappingURL=index.js.map