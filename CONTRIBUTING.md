# Contributing Community Plugins

Community plugins are maintained independently in your own repos. To list yours in the OrqaStudio community registry, submit a PR to this repo.

**Contributing to the core project?** See [orqastudio-dev CONTRIBUTING.md](https://github.com/orqastudio/orqastudio-dev/blob/main/CONTRIBUTING.md).

## Requirements

Your plugin must have:
- A valid `orqa-plugin.json` manifest
- A README.md with the OrqaStudio banner, license badge, status badge, and language badges
- A LICENSE file

Your plugin can use any license you choose.

## How to Submit

1. Build and test your plugin
2. Publish a GitHub Release with a `.tar.gz` archive
3. Fork this repo
4. Add an entry to `registry.json`
5. Open a PR

## Registry Entry Format

Add to the `plugins` array in `registry.json`:

```json
{
  "name": "@yourorg/plugin-name",
  "displayName": "Your Plugin Name",
  "description": "What your plugin does.",
  "repo": "yourorg/your-repo",
  "category": "ai-provider|workflow|integration|custom",
  "icon": "lucide-icon-name",
  "capabilities": ["sidecar", "hooks", "cli-tools", "views", "widgets"],
  "requires": { "node": ">=22" }
}
```

## Review Process

- PRs are reviewed by the OrqaStudio maintainers for compatibility and ecosystem value
- Plugins must have a valid `orqa-plugin.json` manifest
- Plugins that extend core relationship keys will be checked for intent alignment
- Accepted plugins show a **Verified** indicator in the app — the registry is curated, not just a listing

Users can always install their own plugins locally without going through the registry. The registry exists to surface plugins that the maintainers have confirmed are compatible and add value to the ecosystem.
