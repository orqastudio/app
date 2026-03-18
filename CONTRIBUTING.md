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
  "version": "1.0.0",
  "releaseTag": "v1.0.0",
  "sha256": "abc123...",
  "category": "ai-provider|workflow|integration|custom",
  "icon": "lucide-icon-name",
  "capabilities": ["sidecar", "hooks", "cli-tools", "views", "widgets"],
  "requires": { "node": ">=22" }
}
```

The `version`, `releaseTag`, and `sha256` fields pin the entry to a specific verified release. This is a security measure — without it, a plugin author could push arbitrary code to all users who installed via the registry. Updating to a new version requires a new PR for re-verification. The installer checks the SHA256 hash on download.

## Review Process

- PRs are reviewed by the OrqaStudio maintainers for compatibility and ecosystem value
- Plugins must have a valid `orqa-plugin.json` manifest
- Plugins that extend core relationship keys will be checked for intent alignment
- Accepted plugins show a **Verified** indicator in the app — the registry is curated, not just a listing

Users can always install their own plugins locally without going through the registry. The registry exists to surface plugins that the maintainers have confirmed are compatible and add value to the ecosystem.

## Submission Terms

By submitting a plugin to the community registry, you agree to the following:

### Purpose Stability

The stated purpose and description of your plugin at the time of submission is what was verified. If the plugin's purpose fundamentally changes after acceptance, it must be re-submitted for review. Changing a "project management" plugin into a "data scraping" tool without re-verification will result in removal from the registry.

### Breaking Changes Require Re-verification

Any release that introduces breaking changes must be flagged to the maintainers before publication. Breaking changes include:

- Changing or removing artifact types or relationship keys
- Changing the semantic intent of existing relationships
- Removing functionality that users depend on
- Changing the plugin's license to something more restrictive

Submit a PR updating your `registry.json` entry with the new version. The maintainers will re-verify compatibility before the update is accepted.

### Ethical Use Alignment

Plugins in the registry must not be designed to facilitate harm to individuals or communities. Plugins that violate the principles of the [Ethical Use Addendum](https://github.com/orqastudio/orqastudio-dev/blob/main/app/CHANGE-LICENSE) — including discrimination based on race, ethnicity, gender, gender identity, sexual orientation, disability, or any other protected characteristic — will be removed immediately.

### Maintenance Expectations

- Plugins should respond to compatibility issues within a reasonable timeframe
- If a plugin becomes unmaintained and causes issues for users, the maintainers may mark it as deprecated or remove it from the registry
- Archived or abandoned plugins will be flagged with a warning indicator in the app

### Relationship Key Collisions

If your plugin declares relationship or artifact type keys that collide with core or other registered plugins, the collision will be reviewed during the PR process. You may be asked to rename keys or confirm that the intent matches the existing definition. See [Plugin Collision Detection](https://github.com/orqastudio/orqastudio-dev/blob/main/.orqa/documentation/platform/plugin-manifest-schema.md) for details.

### Removal

The maintainers reserve the right to remove any plugin from the registry at any time for:
- Ethical use violations
- Undisclosed breaking changes
- Prolonged incompatibility with current platform versions
- Misrepresentation of purpose or capabilities
- Abandonment causing harm to users

Removal from the registry does not affect users who have already installed the plugin locally.

### Intellectual Property

You retain full ownership and copyright of your plugin. Listing in the registry does not transfer any rights to the OrqaStudio project. The registry entry (name, description, metadata in `registry.json`) is licensed under MIT as part of the registry index.
