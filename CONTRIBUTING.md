# Contributing to OrqaStudio

## Getting Started

```bash
git clone git@github.com:orqastudio/app.git
cd app
make install
```

See the [README](README.md) for full setup instructions.

## Workflow

1. Fork [orqastudio/app](https://github.com/orqastudio/app)
2. `make install` to bootstrap
3. Create a branch: `git checkout -b feat/my-feature`
4. Make changes, commit with sign-off: `git commit -s -m "your message"`
5. Push and open a PR

We use DCO (Developer Certificate of Origin). Sign off every commit. You retain copyright. No CLA required.

## Pre-commit Checks

These must pass before committing:

```bash
orqa check     # lint, typecheck, format
orqa test      # test suites
```

## Quality Standards

- **Rust**: `cargo clippy -- -D warnings`, `cargo fmt --check`, no `unwrap()` in production
- **TypeScript**: strict mode, no `any`, Svelte 5 runes only
- **All**: tests for new functionality, no TODOs, no commented-out code

## Issues and Discussions

File bugs and feature requests on the [monorepo issues](https://github.com/orqastudio/app/issues).

## Community Plugins

Community plugins are maintained independently. To list yours in the registry, submit a PR to [community-plugins](https://github.com/orqastudio/community-plugins) with your plugin's metadata.

## License

BSL-1.1 with Ethical Use Addendum — see [LICENSE](LICENSE).

Copyright (c) 2026 Bobbi Byrne-Graham
