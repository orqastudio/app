# Contributing

All OrqaStudio development happens through the centralised dev environment:

```
git clone --recurse-submodules git@github.com:orqastudio/orqastudio-dev.git
cd orqastudio-dev
make install
make dev
```

**Do not develop in individual repositories directly.** Each repo is a submodule of [orqastudio-dev](https://github.com/orqastudio/orqastudio-dev) — the monorepo that provides the full development environment with all dependencies, tooling, and governance artifacts.

## How to Contribute

1. Clone the [orqastudio-dev](https://github.com/orqastudio/orqastudio-dev) repository
2. Make your changes in the appropriate submodule
3. Run `make check` to verify all tests and lints pass
4. Commit and push from the dev repo root

## License

By contributing, you agree that your contributions will be licensed under the same license as the component you are contributing to. See the LICENSE file in each repository for details.
