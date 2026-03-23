/**
 * Sync Bridge — Bidirectional webhook relay between Forgejo and GitHub.
 *
 * Endpoints:
 *   POST /webhook/github   — receives GitHub webhook events
 *   POST /webhook/forgejo  — receives Forgejo webhook events
 *   GET  /health           — health check
 */

import { createServer, type IncomingMessage, type ServerResponse } from 'node:http';
import { loadConfig } from './config.js';
import {
  handleGitHubPROpened,
  handleForgejoPRMerged,
  type GitHubPRPayload,
  type ForgejoPRPayload,
} from './sync-pr.js';
import {
  handleGitHubIssueOpened,
  handleGitHubIssueClosed,
  handleForgejoIssueOpened,
  handleForgejoIssueClosed,
  type GitHubIssuePayload,
  type ForgejoIssuePayload,
} from './sync-issue.js';
import {
  handleForgejoStatusUpdate,
  type ForgejoStatusPayload,
} from './sync-status.js';

const config = loadConfig();

/** Read the full request body as a string. */
function readBody(req: IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = [];
    req.on('data', (chunk: Buffer) => chunks.push(chunk));
    req.on('end', () => resolve(Buffer.concat(chunks).toString('utf-8')));
    req.on('error', reject);
  });
}

/** Parse JSON body, returning null on failure. */
function parseJSON(body: string): unknown {
  try {
    return JSON.parse(body);
  } catch {
    return null;
  }
}

/** Send a JSON response. */
function sendJSON(
  res: ServerResponse,
  status: number,
  data: Record<string, unknown>,
): void {
  res.writeHead(status, { 'Content-Type': 'application/json' });
  res.end(JSON.stringify(data));
}

/** Route GitHub webhook events to the appropriate handler. */
async function handleGitHubWebhook(
  event: string,
  payload: unknown,
): Promise<string> {
  switch (event) {
    case 'pull_request': {
      const pr = payload as GitHubPRPayload;
      if (pr.action === 'opened' || pr.action === 'reopened') {
        await handleGitHubPROpened(pr, config);
        return `Processed GitHub PR #${pr.number} ${pr.action}`;
      }
      return `Ignored GitHub PR action: ${pr.action}`;
    }

    case 'issues': {
      const issue = payload as GitHubIssuePayload;
      if (issue.action === 'opened') {
        await handleGitHubIssueOpened(issue, config);
        return `Processed GitHub issue #${issue.issue.number} opened`;
      }
      if (issue.action === 'closed') {
        await handleGitHubIssueClosed(issue, config);
        return `Processed GitHub issue #${issue.issue.number} closed`;
      }
      return `Ignored GitHub issue action: ${issue.action}`;
    }

    default:
      return `Ignored GitHub event: ${event}`;
  }
}

/** Route Forgejo webhook events to the appropriate handler. */
async function handleForgejoWebhook(
  event: string,
  payload: unknown,
): Promise<string> {
  switch (event) {
    case 'pull_request': {
      const pr = payload as ForgejoPRPayload;
      if (pr.action === 'closed' && pr.pull_request.merged) {
        await handleForgejoPRMerged(pr, config);
        return `Processed Forgejo PR #${pr.number} merged`;
      }
      return `Ignored Forgejo PR action: ${pr.action}`;
    }

    case 'issues': {
      const issue = payload as ForgejoIssuePayload;
      if (issue.action === 'opened') {
        await handleForgejoIssueOpened(issue, config);
        return `Processed Forgejo issue #${issue.issue.number} opened`;
      }
      if (issue.action === 'closed') {
        await handleForgejoIssueClosed(issue, config);
        return `Processed Forgejo issue #${issue.issue.number} closed`;
      }
      return `Ignored Forgejo issue action: ${issue.action}`;
    }

    case 'status': {
      const status = payload as ForgejoStatusPayload;
      await handleForgejoStatusUpdate(status, config);
      return `Processed Forgejo status for ${status.sha.slice(0, 8)}`;
    }

    default:
      return `Ignored Forgejo event: ${event}`;
  }
}

/** Main request handler. */
async function handleRequest(
  req: IncomingMessage,
  res: ServerResponse,
): Promise<void> {
  const url = req.url ?? '/';
  const method = req.method ?? 'GET';

  // Health check
  if (method === 'GET' && url === '/health') {
    sendJSON(res, 200, { status: 'ok', uptime: process.uptime() });
    return;
  }

  // GitHub webhook
  if (method === 'POST' && url === '/webhook/github') {
    const body = await readBody(req);
    const payload = parseJSON(body);
    if (payload === null) {
      sendJSON(res, 400, { error: 'Invalid JSON' });
      return;
    }

    // GitHub sends the event type in the X-GitHub-Event header
    const event = req.headers['x-github-event'] as string | undefined;
    if (!event) {
      sendJSON(res, 400, { error: 'Missing X-GitHub-Event header' });
      return;
    }

    const message = await handleGitHubWebhook(event, payload);
    console.log(`[github] ${message}`);
    sendJSON(res, 200, { ok: true, message });
    return;
  }

  // Forgejo webhook
  if (method === 'POST' && url === '/webhook/forgejo') {
    const body = await readBody(req);
    const payload = parseJSON(body);
    if (payload === null) {
      sendJSON(res, 400, { error: 'Invalid JSON' });
      return;
    }

    // Forgejo sends the event type in the X-Forgejo-Event header
    // (also supports X-Gitea-Event for compatibility)
    const event =
      (req.headers['x-forgejo-event'] as string | undefined) ??
      (req.headers['x-gitea-event'] as string | undefined);
    if (!event) {
      sendJSON(res, 400, { error: 'Missing X-Forgejo-Event header' });
      return;
    }

    const message = await handleForgejoWebhook(event, payload);
    console.log(`[forgejo] ${message}`);
    sendJSON(res, 200, { ok: true, message });
    return;
  }

  // Not found
  sendJSON(res, 404, { error: 'Not found' });
}

// Start the server
const server = createServer((req, res) => {
  handleRequest(req, res).catch((err) => {
    console.error('[server] Unhandled error:', err);
    sendJSON(res, 500, { error: 'Internal server error' });
  });
});

server.listen(config.port, () => {
  console.log(`[sync-bridge] Listening on port ${config.port}`);
  console.log(`[sync-bridge] Forgejo: ${config.forgejo.url} (${config.forgejo.org}/${config.forgejo.repo})`);
  console.log(`[sync-bridge] GitHub: ${config.github.owner}/${config.github.repo}`);
  console.log(`[sync-bridge] Endpoints:`);
  console.log(`  POST /webhook/github`);
  console.log(`  POST /webhook/forgejo`);
  console.log(`  GET  /health`);
});
