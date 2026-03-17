# Contributing to OrqaStudio Community Plugins

## How to submit a plugin

1. Build your plugin using `orqa plugin create` (see the [template system](../../templates/))
2. Publish a GitHub Release with a `.tar.gz` archive
3. Open a PR to this repo adding your plugin to `registry.json`

## Registry entry format

Add an entry to the `plugins` array in `registry.json`:

```json
{
  "name": "@yourorg/plugin-name",
  "displayName": "Your Plugin Name",
  "description": "What your plugin does.",
  "repo": "yourorg/your-repo",
  "category": "ai-provider|workflow|integration|custom",
  "icon": "lucide-icon-name",
  "capabilities": ["sidecar", "hooks", "cli-tools", "views", "widgets"],
  "requires": { "node": ">=20" }
}
```

## Review process

- PRs are reviewed by the OrqaStudio team
- Plugins must have a valid `orqa-plugin.json` manifest
- Plugins must have a LICENSE file
- Community plugins show an "Unverified" indicator in the app
