export interface BridgeConfig {
  port: number;
  forgejo: {
    url: string;
    token: string;
    webhookSecret: string;
    org: string;
    repo: string;
  };
  github: {
    token: string;
    webhookSecret: string;
    owner: string;
    repo: string;
  };
}

export function loadConfig(): BridgeConfig {
  return {
    port: parseInt(process.env.PORT ?? '10402', 10),
    forgejo: {
      url: process.env.FORGEJO_URL ?? 'http://localhost:10030',
      token: process.env.FORGEJO_TOKEN ?? '',
      webhookSecret: process.env.FORGEJO_WEBHOOK_SECRET ?? 'dev-secret',
      org: process.env.FORGEJO_ORG ?? 'orqastudio',
      repo: process.env.FORGEJO_REPO ?? 'app',
    },
    github: {
      token: process.env.GITHUB_TOKEN ?? '',
      webhookSecret: process.env.GITHUB_WEBHOOK_SECRET ?? 'dev-secret',
      owner: process.env.GITHUB_OWNER ?? 'orqastudio',
      repo: process.env.GITHUB_REPO ?? 'app',
    },
  };
}
