#!/usr/bin/env bash
#
# setup.sh — First-time git server setup
#
# Creates the admin user, orqastudio organisation, and monorepo.
# Run once after the container starts for the first time.
#
# Prerequisites:
#   - Server container running (orqa hosting up)
#   - Wait ~10 seconds for initialisation
#
# Usage:
#   orqa hosting setup

set -euo pipefail

SERVER_URL="http://localhost:3030"
CONTAINER="orqastudio"
ADMIN_USER="orqa-admin"
ADMIN_PASS="admin123"  # Change this after first login
ADMIN_EMAIL="admin@orqastudio.dev"
ORG_NAME="orqastudio"

echo "=== OrqaStudio Git Server Setup ==="
echo ""

# ---------------------------------------------------------------------------
# Step 1: Create admin user via CLI inside the container
# ---------------------------------------------------------------------------

echo "--- Creating admin user ---"
docker exec --user git "$CONTAINER" forgejo admin user create \
  --username "$ADMIN_USER" \
  --password "$ADMIN_PASS" \
  --email "$ADMIN_EMAIL" \
  --admin \
  --must-change-password=false 2>&1 || echo "  (user may already exist)"

echo "  Admin: $ADMIN_USER / $ADMIN_PASS"
echo "  IMPORTANT: Change the password after first login!"
echo ""

# ---------------------------------------------------------------------------
# Step 2: Create the orqastudio organisation via API
# ---------------------------------------------------------------------------

echo "--- Creating organisation: $ORG_NAME ---"
curl -s -X POST "$SERVER_URL/api/v1/orgs" \
  -H "Content-Type: application/json" \
  -u "$ADMIN_USER:$ADMIN_PASS" \
  -d "{
    \"username\": \"$ORG_NAME\",
    \"full_name\": \"OrqaStudio\",
    \"description\": \"AI-assisted structured thinking for complex projects\",
    \"visibility\": \"public\"
  }" > /dev/null 2>&1 || echo "  (org may already exist)"

echo "  Organisation: $ORG_NAME"
echo ""

# ---------------------------------------------------------------------------
# Step 3: Create the monorepo via API
# ---------------------------------------------------------------------------

echo "--- Creating repository: $ORG_NAME/app ---"
curl -s -X POST "$SERVER_URL/api/v1/orgs/$ORG_NAME/repos" \
  -H "Content-Type: application/json" \
  -u "$ADMIN_USER:$ADMIN_PASS" \
  -d "{
    \"name\": \"app\",
    \"description\": \"OrqaStudio monorepo — app, libraries, plugins, connectors\",
    \"private\": false,
    \"default_branch\": \"main\",
    \"auto_init\": false
  }" > /dev/null 2>&1 || echo "  (repo may already exist)"

echo "  Repo: $ORG_NAME/app"
echo ""

# ---------------------------------------------------------------------------
# Step 4: Add local server as a git remote and push
# ---------------------------------------------------------------------------

echo "--- Pushing monorepo to local server ---"

cd "$(git rev-parse --show-toplevel)"

REMOTE_NAME="local"

if git remote get-url "$REMOTE_NAME" > /dev/null 2>&1; then
  echo "  Remote '$REMOTE_NAME' already exists"
else
  git remote add "$REMOTE_NAME" "http://$ADMIN_USER:$ADMIN_PASS@localhost:3030/$ORG_NAME/orqastudio.git"
  echo "  Remote '$REMOTE_NAME' added"
fi

git push "$REMOTE_NAME" main 2>&1 || echo "  (push may require force on first time)"
echo "  Monorepo pushed to local server"
echo ""

# ---------------------------------------------------------------------------
# Step 5: Configure push mirror to GitHub
# ---------------------------------------------------------------------------

echo "--- Configuring push mirror to GitHub ---"
echo ""
echo "  Push mirror must be configured via the web UI:"
echo "  1. Go to: $SERVER_URL/$ORG_NAME/orqastudio/settings"
echo "  2. Click 'Mirror Settings'"
echo "  3. Add push mirror:"
echo "     URL: https://github.com/orqastudio/app.git"
echo "     Auth: GitHub personal access token with repo scope"
echo "  4. Set sync interval (e.g., every push or hourly)"
echo ""
echo "  NOTE: Create the target repo on GitHub first."
echo ""

# ---------------------------------------------------------------------------
# Step 6: Configure branch protection
# ---------------------------------------------------------------------------

echo "--- Configuring branch protection ---"
curl -s -X POST "$SERVER_URL/api/v1/repos/$ORG_NAME/orqastudio/branch_protections" \
  -H "Content-Type: application/json" \
  -u "$ADMIN_USER:$ADMIN_PASS" \
  -d "{
    \"branch_name\": \"main\",
    \"enable_push\": false,
    \"enable_push_whitelist\": true,
    \"push_whitelist_usernames\": [\"$ADMIN_USER\"],
    \"require_signed_commits\": false,
    \"enable_merge_whitelist\": false
  }" > /dev/null 2>&1 || echo "  (protection may already exist)"

echo "  Branch protection on 'main': PRs required, admin can push"
echo ""

echo "=== Setup Complete ==="
echo ""
echo "  Web UI:     $SERVER_URL"
echo "  Repo:       $SERVER_URL/$ORG_NAME/orqastudio"
echo "  SSH clone:  ssh://git@localhost:222/$ORG_NAME/orqastudio.git"
echo "  HTTP clone: $SERVER_URL/$ORG_NAME/orqastudio.git"
echo ""
echo "  Next steps:"
echo "  1. Change admin password at $SERVER_URL/user/settings/account"
echo "  2. Configure push mirror to GitHub via repo settings"
echo "  3. Set up SSH key for the admin user"
