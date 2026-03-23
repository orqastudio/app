/**
 * Issue sync logic.
 *
 * GitHub issue opened  -> create corresponding Forgejo issue
 * Forgejo issue closed -> close corresponding GitHub issue
 * Forgejo issue opened -> create corresponding GitHub issue
 * GitHub issue closed  -> close corresponding Forgejo issue
 */

import type { BridgeConfig } from './config.js';
import type { GitHubClient } from './github.js';
import type { ForgejoClient } from './forgejo.js';
import * as github from './github.js';
import * as forgejo from './forgejo.js';

/** Marker used in titles to identify synced issues and prevent loops. */
const GITHUB_ISSUE_MARKER = '[Synced from GitHub #';
const FORGEJO_ISSUE_MARKER = '[Synced from Forgejo #';

/** Check whether an issue title indicates it was synced from the other platform. */
function isSyncedIssue(title: string): boolean {
  return (
    title.includes(GITHUB_ISSUE_MARKER) ||
    title.includes(FORGEJO_ISSUE_MARKER)
  );
}

/**
 * Extract the origin issue number from a synced title marker.
 * Returns null if no marker is found.
 */
function extractOriginIssueNumber(
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

export interface GitHubIssuePayload {
  action: string;
  issue: {
    number: number;
    title: string;
    body: string | null;
    html_url: string;
    pull_request?: unknown;
  };
}

export interface ForgejoIssuePayload {
  action: string;
  issue: {
    number: number;
    title: string;
    body: string;
    html_url: string;
    pull_request: unknown | null;
  };
}

/**
 * Handle a GitHub issue being opened.
 * Creates a corresponding issue on Forgejo.
 */
export async function handleGitHubIssueOpened(
  payload: GitHubIssuePayload,
  config: BridgeConfig,
): Promise<void> {
  const issue = payload.issue;

  // Skip pull request events that come through the issues webhook
  if (issue.pull_request) {
    return;
  }

  // Prevent sync loops
  if (isSyncedIssue(issue.title)) {
    console.log(
      `[sync-issue] Skipping GitHub issue #${issue.number} — synced from Forgejo`,
    );
    return;
  }

  console.log(
    `[sync-issue] GitHub issue #${issue.number} opened: "${issue.title}"`,
  );

  const fjClient: ForgejoClient = {
    url: config.forgejo.url,
    token: config.forgejo.token,
    org: config.forgejo.org,
    repo: config.forgejo.repo,
  };

  const fjTitle = `${GITHUB_ISSUE_MARKER}${issue.number}] ${issue.title}`;
  const fjBody = [
    `Synced from GitHub: ${issue.html_url}`,
    '',
    issue.body ?? '',
  ].join('\n');

  try {
    const fjIssue = await forgejo.createIssue(fjClient, fjTitle, fjBody);
    console.log(
      `[sync-issue] Created Forgejo issue #${fjIssue.number}: ${fjIssue.html_url}`,
    );
  } catch (err) {
    console.error(
      `[sync-issue] Failed to sync GitHub issue #${issue.number}:`,
      err,
    );
  }
}

/**
 * Handle a GitHub issue being closed.
 * Closes the corresponding Forgejo issue if it was synced.
 */
export async function handleGitHubIssueClosed(
  payload: GitHubIssuePayload,
  config: BridgeConfig,
): Promise<void> {
  const issue = payload.issue;

  // Skip pull request events
  if (issue.pull_request) {
    return;
  }

  // Only act on issues that originated on GitHub (synced TO Forgejo).
  // We need to find the Forgejo issue with the matching marker.
  if (isSyncedIssue(issue.title)) {
    // This issue was synced FROM Forgejo — closing it was triggered by the
    // bridge itself. Skip to prevent loops.
    console.log(
      `[sync-issue] Skipping GitHub issue #${issue.number} close — synced from Forgejo`,
    );
    return;
  }

  console.log(
    `[sync-issue] GitHub issue #${issue.number} closed: "${issue.title}"`,
  );

  const fjClient: ForgejoClient = {
    url: config.forgejo.url,
    token: config.forgejo.token,
    org: config.forgejo.org,
    repo: config.forgejo.repo,
  };

  try {
    // Search Forgejo issues for the matching synced issue
    const issues = await forgejo.listIssues(fjClient, 'open');
    const marker = `${GITHUB_ISSUE_MARKER}${issue.number}]`;
    const matched = issues.find((i) => i.title.includes(marker));

    if (matched) {
      await forgejo.closeIssue(
        fjClient,
        matched.number,
        `Closed on GitHub: ${issue.html_url}`,
      );
      console.log(
        `[sync-issue] Closed Forgejo issue #${matched.number} (GitHub #${issue.number} closed)`,
      );
    } else {
      console.log(
        `[sync-issue] No matching Forgejo issue found for GitHub #${issue.number}`,
      );
    }
  } catch (err) {
    console.error(
      `[sync-issue] Failed to close Forgejo issue for GitHub #${issue.number}:`,
      err,
    );
  }
}

/**
 * Handle a Forgejo issue being opened.
 * Creates a corresponding issue on GitHub.
 */
export async function handleForgejoIssueOpened(
  payload: ForgejoIssuePayload,
  config: BridgeConfig,
): Promise<void> {
  const issue = payload.issue;

  // Skip pull request events
  if (issue.pull_request) {
    return;
  }

  // Prevent sync loops
  if (isSyncedIssue(issue.title)) {
    console.log(
      `[sync-issue] Skipping Forgejo issue #${issue.number} — synced from GitHub`,
    );
    return;
  }

  console.log(
    `[sync-issue] Forgejo issue #${issue.number} opened: "${issue.title}"`,
  );

  const ghClient: GitHubClient = {
    token: config.github.token,
    owner: config.github.owner,
    repo: config.github.repo,
  };

  const ghTitle = `${FORGEJO_ISSUE_MARKER}${issue.number}] ${issue.title}`;
  const ghBody = [
    `Synced from Forgejo: ${issue.html_url}`,
    '',
    issue.body ?? '',
  ].join('\n');

  try {
    const ghIssue = await github.createIssue(ghClient, ghTitle, ghBody);
    console.log(
      `[sync-issue] Created GitHub issue #${ghIssue.number}: ${ghIssue.html_url}`,
    );
  } catch (err) {
    console.error(
      `[sync-issue] Failed to sync Forgejo issue #${issue.number}:`,
      err,
    );
  }
}

/**
 * Handle a Forgejo issue being closed.
 * Closes the corresponding GitHub issue if it was synced.
 */
export async function handleForgejoIssueClosed(
  payload: ForgejoIssuePayload,
  config: BridgeConfig,
): Promise<void> {
  const issue = payload.issue;

  // Skip pull request events
  if (issue.pull_request) {
    return;
  }

  // Only act on issues that originated on Forgejo (synced TO GitHub).
  if (isSyncedIssue(issue.title)) {
    console.log(
      `[sync-issue] Skipping Forgejo issue #${issue.number} close — synced from GitHub`,
    );
    return;
  }

  console.log(
    `[sync-issue] Forgejo issue #${issue.number} closed: "${issue.title}"`,
  );

  const ghClient: GitHubClient = {
    token: config.github.token,
    owner: config.github.owner,
    repo: config.github.repo,
  };

  try {
    // Search GitHub issues for the matching synced issue
    const marker = `${FORGEJO_ISSUE_MARKER}${issue.number}]`;
    // GitHub search API is rate-limited, so we list open issues and filter
    // For a small repo this is fine; for large repos, use search API.
    const ghIssues = await github.listPullRequests(ghClient, undefined, 'open');
    // listPullRequests returns PRs — we need to look through issues instead.
    // Since GitHub's issues API includes PRs, we use the generic issue close
    // approach: find by extracting the number from the Forgejo title marker.
    const originNumber = extractOriginIssueNumber(
      issue.title,
      FORGEJO_ISSUE_MARKER,
    );

    // The Forgejo issue title does NOT contain the Forgejo marker (it's the
    // original issue). We need to search GitHub for issues with our marker.
    // For now, log and skip — a production implementation would use GitHub
    // search API: `GET /search/issues?q=repo:owner/repo+"[Synced from Forgejo #N]"`
    // Instead, we track the mapping in the Forgejo issue body.
    if (originNumber !== null) {
      // This should not happen for non-synced issues
      console.log(
        `[sync-issue] Unexpected: Forgejo issue has origin marker but was not detected as synced`,
      );
      return;
    }

    // For issues originating on Forgejo, we need to find the GitHub mirror.
    // The GitHub issue title will contain `[Synced from Forgejo #N]`.
    // Simple approach: list recent open issues and match.
    console.log(
      `[sync-issue] Looking for GitHub issue with marker "${marker}"`,
    );

    // Use the issues endpoint via a raw fetch since our github client
    // doesn't have a listIssues method (PRs and issues share the endpoint).
    // For production, add a dedicated listIssues to github.ts.
    // For now, close via the issue number if we can find it.
    const res = await fetch(
      `https://api.github.com/repos/${ghClient.owner}/${ghClient.repo}/issues?state=open&per_page=100`,
      {
        headers: {
          Accept: 'application/vnd.github+json',
          Authorization: `Bearer ${ghClient.token}`,
          'X-GitHub-Api-Version': '2022-11-28',
        },
      },
    );

    if (res.ok) {
      const issues = (await res.json()) as Array<{
        number: number;
        title: string;
        pull_request?: unknown;
      }>;
      const matched = issues.find(
        (i) => !i.pull_request && i.title.includes(marker),
      );

      if (matched) {
        await github.closeIssue(
          ghClient,
          matched.number,
          `Closed on Forgejo: ${issue.html_url}`,
        );
        console.log(
          `[sync-issue] Closed GitHub issue #${matched.number} (Forgejo #${issue.number} closed)`,
        );
      } else {
        console.log(
          `[sync-issue] No matching GitHub issue found for Forgejo #${issue.number}`,
        );
      }
    }
  } catch (err) {
    console.error(
      `[sync-issue] Failed to close GitHub issue for Forgejo #${issue.number}:`,
      err,
    );
  }
}
