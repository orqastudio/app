/** Forgejo REST API client. */

export interface ForgejoClient {
  url: string;
  token: string;
  org: string;
  repo: string;
}

async function forgejoFetch(
  client: ForgejoClient,
  path: string,
  init?: RequestInit,
): Promise<Response> {
  const url = `${client.url}/api/v1${path}`;
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    Accept: 'application/json',
    ...(init?.headers as Record<string, string> | undefined),
  };
  if (client.token) {
    headers.Authorization = `token ${client.token}`;
  }
  return fetch(url, { ...init, headers });
}

/** Create a pull request on Forgejo. */
export async function createPullRequest(
  client: ForgejoClient,
  title: string,
  body: string,
  head: string,
  base: string,
): Promise<{ number: number; html_url: string }> {
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/pulls`,
    {
      method: 'POST',
      body: JSON.stringify({ title, body, head, base }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(
      `Forgejo createPullRequest failed (${res.status}): ${text}`,
    );
  }
  const data = (await res.json()) as { number: number; html_url: string };
  return { number: data.number, html_url: data.html_url };
}

/** Create an issue on Forgejo. */
export async function createIssue(
  client: ForgejoClient,
  title: string,
  body: string,
  labels?: number[],
): Promise<{ number: number; html_url: string }> {
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/issues`,
    {
      method: 'POST',
      body: JSON.stringify({ title, body, labels }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Forgejo createIssue failed (${res.status}): ${text}`);
  }
  const data = (await res.json()) as { number: number; html_url: string };
  return { number: data.number, html_url: data.html_url };
}

/** Close an issue on Forgejo. */
export async function closeIssue(
  client: ForgejoClient,
  issueNumber: number,
  comment?: string,
): Promise<void> {
  if (comment) {
    await forgejoFetch(
      client,
      `/repos/${client.org}/${client.repo}/issues/${issueNumber}/comments`,
      {
        method: 'POST',
        body: JSON.stringify({ body: comment }),
      },
    );
  }
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/issues/${issueNumber}`,
    {
      method: 'PATCH',
      body: JSON.stringify({ state: 'closed' }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Forgejo closeIssue failed (${res.status}): ${text}`);
  }
}

/** Comment on a Forgejo issue or PR. */
export async function addComment(
  client: ForgejoClient,
  issueNumber: number,
  body: string,
): Promise<void> {
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/issues/${issueNumber}/comments`,
    {
      method: 'POST',
      body: JSON.stringify({ body }),
    },
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Forgejo addComment failed (${res.status}): ${text}`);
  }
}

/** List open pull requests on Forgejo, optionally filtered by title search. */
export async function listPullRequests(
  client: ForgejoClient,
  state: 'open' | 'closed' | 'all' = 'open',
): Promise<Array<{ number: number; title: string; head: { ref: string } }>> {
  const params = new URLSearchParams({ state });
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/pulls?${params}`,
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(
      `Forgejo listPullRequests failed (${res.status}): ${text}`,
    );
  }
  return (await res.json()) as Array<{
    number: number;
    title: string;
    head: { ref: string };
  }>;
}

/** List open issues on Forgejo. */
export async function listIssues(
  client: ForgejoClient,
  state: 'open' | 'closed' | 'all' = 'open',
): Promise<Array<{ number: number; title: string }>> {
  const params = new URLSearchParams({ state, type: 'issues' });
  const res = await forgejoFetch(
    client,
    `/repos/${client.org}/${client.repo}/issues?${params}`,
  );
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Forgejo listIssues failed (${res.status}): ${text}`);
  }
  return (await res.json()) as Array<{ number: number; title: string }>;
}
