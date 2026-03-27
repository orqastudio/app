---
name: orqa
description: "Run OrqaStudio governance commands. Routes to subcommands: save, create, validate, and more."
user-invocable: true
---

# OrqaStudio CLI

Run governance commands via the `orqa` CLI. Available subcommands:

## Commands

| Command                      | Description                                                                    |
| ---------------------------- | ------------------------------------------------------------------------------ |
| `orqa save`                  | Save current context to governance artifacts                                   |
| `orqa create <type>`         | Create a new governance artifact (task, epic, decision, rule, etc.)            |
| `orqa validate`              | Validate all artifacts against the composed schema                             |
| `orqa install`               | Set up the entire dev environment (runs `orqa plugin install` for each plugin) |
| `orqa verify`                | Verify installation integrity                                                  |
| `orqa check`                 | Run all quality checks                                                         |
| `orqa test`                  | Run test suite                                                                 |
| `orqa graph`                 | Query the artifact graph                                                       |
| `orqa plugin install <name>` | Install/update a specific plugin (triggers composition if it affects schema)   |
| `orqa plugin list`           | List installed plugins                                                         |
| `orqa version`               | Show CLI version                                                               |
| `orqa enforce`               | Run enforcement checks (use `--fix` for auto-remediation)                      |
| `orqa daemon start`          | Start the governance daemon                                                    |

## Usage

Run any command via Bash:

```bash
orqa <command> [options]
```

All governance logic runs through the daemon. If the daemon is not running, start it first with `orqa daemon start`.
