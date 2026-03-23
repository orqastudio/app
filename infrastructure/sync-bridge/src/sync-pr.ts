/**
 * PR sync logic.
 *
 * GitHub PR opened  -> push branch to Forgejo, create Forgejo PR
 * Forgejo PR merged -> close corresponding GitHub PR
 */

import type { BridgeConfig } from './config.js';
import type { GitHubClient } from './github.js';
import type { ForgejoClient } from './forgejo.js';
import * as github from './github.js';
import * as forgejo from './forgejo.js';

/** Marker prefix used in Forgejo PR titles for PRs synced from GitHub. */
const GITHUB_PR_MARKER = '[GitHub PR #';

/** Marker prefix used in GitHub PR titles for PRs synced from Forgejo. */
const FORGEJO_PR_MARKER = '[Forgejo PR #';

/** Check whether a PR title indicates it was synced from the other platform. */
function isSyncedPR(title: string): boolean {
  return title.includes(GITHUB_PR_MARKER) || title.includes(FORGEJO_PR_MARKER);
}

/**
 * Extract the origin PR number from a synced title marker.
 * Returns null if no marker is found.
 */
function extractOriginPRNumber(
  title: string,
  marker: string,
): number | null {
  const idx = title.indexOf(marker);
  if (idx === -1) return null;
  const start = idx + marker.length;
  const end = title.indexOf(']', start);
  if (end === -1) return null;
  const num = parseInt(title.slice(start, end), 10);
  return Number.isNaN(num) ? null : num;
}

export interface GitHubPRPayload {
  action: string;
  number: number;
  pull_request: {
    number: number;
    title: string;
    body: string | null;
    head: {
      ref: string;
      sha: string;
    };
    base: {
      ref: string;
    };
    html_url: string;
  };
}

export interface ForgejoPRPayload {
  action: string;
  number: number;
  pull_request: {
    number: number;
    title: string;
    body: string;
    head: {
      ref: string;
      sha: string;
    };
    base: {
      ref: string;
    };
    html_url: string;
    merged: boolean;
  };
}

/**
 * Handle a GitHub PR being opened.
 *
 * 1. The branch must be fetched from GitHub and pushed to Forgejo
 *    (handled externally via git CLI or mirror — logged here as a note).
 * 2. Create a corresponding PR on Forgejo.
 * 3. Comment on the GitHub PR to confirm the sync.
 */
export async function handleGitHubPROpened(
  payload: GitHubPRPayload,
  config: BridgeConfig,
): Promise<void> {
  const pr = payload.pull_request;
  const ghClient: GitHubClient = {
    token: config.github.token,
    owner: config.github.owner,
    repo: config.github.repo,
  };
  const fjClient: ForgejoClient = {
    url: config.forgejo.url,
    token: config.forgejo.token,
    org: config.forgejo.org,
    repo: config.forgejo.repo,
  };

  // Prevent sync loops — if this PR was created by the bridge, skip
  if (isSyncedPR(pr.title)) {
    console.log(
      `[sync-pr] Skipping GitHub PR #${pr.number} — synced from Forgejo`,
    );
    return;
  }

  console.log(
    `[sync-pr] GitHub PR #${pr.number} opened: "${pr.title}" (${pr.head.ref} -> ${pr.base.ref})`,
  );

  // NOTE: The contributor's branch needs to exist on Forgejo for the PR
  // creation to succeed. In production this is handled by fetching the
  // branch from GitHub and pushing it to Forgejo via git CLI.
  // For now we assume the push mirror or a separate job handles this.
  console.log(
    `[sync-pr] Branch "${pr.head.ref}" must be available on Forgejo (via mirror or manual push)`,
  );

  // Create PR on Forgejo
  const forgejoTitle = `${GITHUB_PR_MARKER}${pr.number}] ${pr.title}`;
  const forgejoBody = [
    `Synced from GitHub: ${pr.html_url}`,
    '',
    pr.body ?? '',
  ].join('\n');

  try {
    const fjPR = await forgejo.createPullRequest(
      fjClient,
      forgejoTitle,
      forgejoBody,
      pr.head.ref,
      pr.base.ref,
    );
    console.log(
      `[sync-pr] Created Forgejo PR #${fjPR.number}: ${fjPR.html_url}`,
    );

    // Comment on the GitHub PR to confirm
    await github.addComment(
      ghClient,
      pr.number,
      `Synced to local Forgejo server as [PR #${fjPR.number}](${fjPR.html_url}).`,
    );
  } catch (err) {
    console.error(`[sync-pr] Failed to sync GitHub PR #${pr.number}:`, err);
  }
}

/**
 * Handle a Forgejo PR being merged.
 *
 * 1. Push mirror updates GitHub automatically.
 * 2. Find the corresponding GitHub PR (via marker in title).
 * 3. Close the GitHub PR with a comment.
 */
export async function handleForgejoPRMerged(
  payload: ForgejoPRPayload,
  config: BridgeConfig,
): Promise<void> {
  const pr = payload.pull_request;

  if (!pr.merged) {
    console.log(
      `[sync-pr] Forgejo PR #${pr.number} closed but not merged — skipping`,
    );
    return;
  }

  console.log(
    `[sync-pr] Forgejo PR #${pr.number} merged: "${pr.title}"`,
  );

  // Check if this Forgejo PR was synced from a GitHub PR
  const ghPRNumber = extractOriginPRNumber(pr.title, GITHUB_PR_MARKER);

  if (ghPRNumber === null) {
    // This PR originated on Forgejo, no corresponding GitHub PR
    console.log(
      `[sync-pr] Forgejo PR #${pr.number} is not synced from GitHub — skipping GitHub close`,
    );
    return;
  }

  const ghClient: GitHubClient = {
    token: config.github.token,
    owner: config.github.owner,
    repo: config.github.repo,
  };

  try {
    await github.closePullRequest(
      ghClient,
      ghPRNumber,
      `Merged via local Forgejo server in [PR #${pr.number}](${pr.html_url}).`,
    );
    console.log(
      `[sync-pr] Closed GitHub PR #${ghPRNumber} (merged on Forgejo)`,
    );
  } catch (err) {
    console.error(
      `[sync-pr] Failed to close GitHub PR #${ghPRNumber}:`,
      err,
    );
  }
}
