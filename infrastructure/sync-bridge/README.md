# Sync Bridge

Bidirectional webhook relay between Forgejo (local git server) and GitHub (cloud mirror).

## What it does

| Event | Direction | Action |
|-------|-----------|--------|
| PR opened on GitHub | GitHub -> Forgejo | Push branch to Forgejo, create corresponding PR |
| PR merged on Forgejo | Forgejo -> GitHub | Close the corresponding GitHub PR |
| Issue opened on GitHub | GitHub -> Forgejo | Create corresponding Forgejo issue |
| Issue opened on Forgejo | Forgejo -> GitHub | Create corresponding GitHub issue |
| Issue closed on either | Bidirectional | Close the mirror issue |
| CI status on Forgejo | Forgejo -> GitHub | Post commit status to GitHub |

## Sync loop prevention

Synced items carry marker tags in their titles:
- `[GitHub PR #N]` / `[Synced from GitHub #N]` for items originating on GitHub
- `[Forgejo PR #N]` / `[Synced from Forgejo #N]` for items originating on Forgejo

The bridge checks for these markers and skips items that were created by itself.

## Endpoints

| Method | Path | Purpose |
|--------|------|---------|
| POST | `/webhook/github` | Receives GitHub webhook events |
| POST | `/webhook/forgejo` | Receives Forgejo webhook events |
| GET | `/health` | Health check |

## Configuration

All configuration is via environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `3001` | HTTP server port |
| `FORGEJO_URL` | `http://localhost:3000` | Forgejo base URL |
| `FORGEJO_TOKEN` | (empty) | Forgejo API token |
| `FORGEJO_WEBHOOK_SECRET` | `dev-secret` | Webhook signature secret |
| `FORGEJO_ORG` | `orqastudio` | Forgejo organization |
| `FORGEJO_REPO` | `orqastudio` | Forgejo repository |
| `GITHUB_TOKEN` | (empty) | GitHub PAT with repo scope |
| `GITHUB_WEBHOOK_SECRET` | `dev-secret` | Webhook signature secret |
| `GITHUB_OWNER` | `orqastudio` | GitHub repository owner |
| `GITHUB_REPO` | `orqastudio` | GitHub repository |

## Development

```bash
cd infrastructure/sync-bridge
npm install
npm run build
npm start
```

## Docker

The sync bridge runs as a service in the Forgejo Docker Compose stack:

```bash
cd infrastructure/forgejo
docker compose up -d
```

## Webhook setup

### GitHub

1. Go to repo Settings > Webhooks > Add webhook
2. Payload URL: `http://<your-server>:3001/webhook/github`
3. Content type: `application/json`
4. Secret: match `GITHUB_WEBHOOK_SECRET`
5. Events: Pull requests, Issues

### Forgejo

1. Go to repo Settings > Webhooks > Add webhook (Forgejo)
2. Target URL: `http://sync-bridge:3001/webhook/forgejo` (Docker network)
3. Secret: match `FORGEJO_WEBHOOK_SECRET`
4. Events: Pull requests, Issues, Status
