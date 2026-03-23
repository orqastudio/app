/** GitHub REST API client. */

export interface GitHubClient {
  token: string;
  owner: string;
  repo: string;
}

const API_BASE = 'https://api.github.com';

async function ghFetch(
  client: GitHubClient,
  path: string,
  init?: RequestInit,
): Promise<Response> {
  const url = `${API_BASE}${path}`;
  const headers: Record<string, string> = {
    Accept: 'application/vnd.github+json',
    'X-GitHub-Api-Version': '2022-11-28',
    ...(init?.headers as Record<string, string> | undefined),
  };
  if (client.token) {
    headers.Authorization = `Bearer ${client.token}`;
  }
  return fetch(url, { ...init, headers });
}

/** Create an issue on GitHub. */
export async function createIssue(
  client: GitHubClient,
  title: string,
  body: string,
  labels?: string[],
): Promise<{ number: number; html_url: string }> {
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/issues`,
    {
      method: 'POST',
      body: JSON.stringify({ title, body, labels }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`GitHub createIssue failed (${res.status}): ${text}`);
  }
  const data = (await res.json()) as { number: number; html_url: string };
  return { number: data.number, html_url: data.html_url };
}

/** Close an issue on GitHub. */
export async function closeIssue(
  client: GitHubClient,
  issueNumber: number,
  comment?: string,
): Promise<void> {
  if (comment) {
    await ghFetch(
      client,
      `/repos/${client.owner}/${client.repo}/issues/${issueNumber}/comments`,
      {
        method: 'POST',
        body: JSON.stringify({ body: comment }),
      },
    );
  }
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/issues/${issueNumber}`,
    {
      method: 'PATCH',
      body: JSON.stringify({ state: 'closed' }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`GitHub closeIssue failed (${res.status}): ${text}`);
  }
}

/** Close a pull request on GitHub with an optional comment. */
export async function closePullRequest(
  client: GitHubClient,
  prNumber: number,
  comment?: string,
): Promise<void> {
  if (comment) {
    await ghFetch(
      client,
      `/repos/${client.owner}/${client.repo}/issues/${prNumber}/comments`,
      {
        method: 'POST',
        body: JSON.stringify({ body: comment }),
      },
    );
  }
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/pulls/${prNumber}`,
    {
      method: 'PATCH',
      body: JSON.stringify({ state: 'closed' }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`GitHub closePullRequest failed (${res.status}): ${text}`);
  }
}

/** Comment on a GitHub issue or PR. */
export async function addComment(
  client: GitHubClient,
  issueNumber: number,
  body: string,
): Promise<void> {
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/issues/${issueNumber}/comments`,
    {
      method: 'POST',
      body: JSON.stringify({ body }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`GitHub addComment failed (${res.status}): ${text}`);
  }
}

/** Post a commit status to GitHub. */
export async function createCommitStatus(
  client: GitHubClient,
  sha: string,
  state: 'error' | 'failure' | 'pending' | 'success',
  options?: {
    target_url?: string;
    description?: string;
    context?: string;
  },
): Promise<void> {
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/statuses/${sha}`,
    {
      method: 'POST',
      body: JSON.stringify({
        state,
        target_url: options?.target_url,
        description: options?.description,
        context: options?.context ?? 'forgejo-ci',
      }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(
      `GitHub createCommitStatus failed (${res.status}): ${text}`,
    );
  }
}

/** List open PRs matching a head branch. */
export async function listPullRequests(
  client: GitHubClient,
  head?: string,
  state: 'open' | 'closed' | 'all' = 'open',
): Promise<Array<{ number: number; title: string; head: { ref: string } }>> {
  const params = new URLSearchParams({ state });
  if (head) {
    params.set('head', `${client.owner}:${head}`);
  }
  const res = await ghFetch(
    client,
    `/repos/${client.owner}/${client.repo}/pulls?${params}`,
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(
      `GitHub listPullRequests failed (${res.status}): ${text}`,
    );
  }
  return (await res.json()) as Array<{
    number: number;
    title: string;
    head: { ref: string };
  }>;
}
