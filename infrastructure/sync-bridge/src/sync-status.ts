/**
 * CI status sync logic.
 *
 * When Forgejo CI completes a pipeline, post the commit status to GitHub
 * so contributors see green/red checks on their PRs.
 */

import type { BridgeConfig } from './config.js';
import type { GitHubClient } from './github.js';
import * as github from './github.js';

/**
 * Forgejo commit status event payload.
 *
 * Forgejo sends a `status` webhook event when a commit status is created
 * or updated (e.g., by Forgejo Actions CI).
 */
export interface ForgejoStatusPayload {
  sha: string;
  state: string; // 'pending' | 'success' | 'failure' | 'error'
  description: string;
  target_url: string;
  context: string;
  repository: {
    full_name: string;
  };
}

/** Map Forgejo status states to GitHub-compatible states. */
function mapState(
  forgejoState: string,
): 'error' | 'failure' | 'pending' | 'success' {
  switch (forgejoState) {
    case 'success':
      return 'success';
    case 'failure':
      return 'failure';
    case 'error':
      return 'error';
    case 'pending':
    case 'running':
      return 'pending';
    default:
      return 'error';
  }
}

/**
 * Handle a Forgejo commit status update.
 * Posts the equivalent status to GitHub for the same commit SHA.
 */
export async function handleForgejoStatusUpdate(
  payload: ForgejoStatusPayload,
  config: BridgeConfig,
): Promise<void> {
  const { sha, state, description, target_url, context } = payload;

  console.log(
    `[sync-status] Forgejo status for ${sha.slice(0, 8)}: ${state} (${context})`,
  );

  const ghClient: GitHubClient = {
    token: config.github.token,
    owner: config.github.owner,
    repo: config.github.repo,
  };

  const ghState = mapState(state);

  try {
    await github.createCommitStatus(ghClient, sha, ghState, {
      target_url,
      description: description || `Forgejo CI: ${context}`,
      context: `forgejo/${context}`,
    });
    console.log(
      `[sync-status] Posted GitHub status for ${sha.slice(0, 8)}: ${ghState}`,
    );
  } catch (err) {
    console.error(
      `[sync-status] Failed to post GitHub status for ${sha.slice(0, 8)}:`,
      err,
    );
  }
}
