# Contributing

## Prerequisites

- Rust 1.85+ (edition 2024)
- [pre-commit](https://pre-commit.com/) — enforces formatting, clippy, and commit conventions
- A [SquareCloud](https://squarecloud.app) account and API token (integration tests only)

## Setup

```sh
git clone https://github.com/robert-nogueira/squarecloud-rs
cd squarecloud-rs
pre-commit install --hook-type commit-msg --hook-type pre-commit
```

For integration tests, copy `.env.test.example` to `.env.test` and fill in your token:

```sh
cp .env.test.example .env.test
# edit .env.test and set API_TOKEN
```

## Running tests

```sh
# Unit and internal tests (no credentials needed)
cargo test --lib

# Mock tests: isolated behavior, no real API calls
cargo test-mock

# Integration tests: hits the real SquareCloud API, runs sequentially to avoid rate limits
cargo test-integration

# Contract tests: validates implemented endpoints against the OpenAPI spec
cargo test-contract
```

## Checks

The pre-commit hooks run automatically on every commit:

```sh
pre-commit run --all-files   # run manually against all files
cargo fmt --check            # formatting (max 79 chars, see rustfmt.toml)
cargo clippy -- -D warnings  # lints
```

## Commit convention

Commits follow `cz-conventional-gitmoji` (enforced by the commit-msg hook):

```
<emoji> <type>(<scope>): <description>
```

Use `git cz` or `cz commit` to get an interactive prompt. Common types: `feat`, `fix`, `refactor`, `docs`, `test`, `chore`, `ci`.

## Pull requests

- Open a PR against `main`
- All CI jobs (fmt, clippy, tests, doc) must pass
- Integration tests are not run in CI (require credentials); cover new endpoints with mock tests instead (`cargo test-mock`)
- One logical change per PR; keep the scope focused
