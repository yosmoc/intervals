# AGENTS.md

## Development Workflow

- **TDD first**: Write tests, then implement. Use `wiremock` for HTTP mocking.
- **After implementing a new function**: `cargo fmt --all` → `cargo test` →
  update `TODO.md`, `README.md`, and `skills/intervals-cli/SKILL.md`.
- **Pre-commit hooks**: secretlint, cargo-fmt, cargo-test, pinact, markdownlint.
  All must pass.

## Key Commands

```bash
cargo test              # Run all tests
cargo test <pattern>    # Run matching tests
cargo fmt --all         # Format code
cargo fmt --all -- --check  # Check formatting (CI)
```

## Project Structure

- Single crate: `intervals` (binary name `intervals`, **not** `intervals-cli`)
- `src/main.rs` — CLI entry point with `clap` subcommands
- `src/commands/<feature>.rs` — one file per command, tests inline in same file
- `src/client.rs` — `ApiClient` with HTTP Basic Auth
- `skills/intervals-cli/` — Claude skill for using this CLI

## API Quirks

- **Auth**: HTTP Basic Auth. Username: `API_KEY`, Password: actual API key value.
- **Response wrappers** — many endpoints nest data:
  - `list-athlete-hr-curves`, `list-athlete-pace-curves` → `{ "list": [...] }`
  - `list-activity-intervals` → `{ "icu_intervals": [...] }`
  - `get-weather-forecast` → `{ "forecasts": [...] }`
  - `get-athlete-profile` → `{ "athlete": { ... } }`
- `SportSettings.id` is `i64`, not `String`.
- `list-activities` requires `--oldest` flag.
- `list-athlete-power-curves` requires `activity_type` param (e.g., `Ride`, `Run`).

## Testing

- Tests use `wiremock` for HTTP mocking.
- Shared auth header constant: `crate::commands::TEST_AUTH_HEADER`
- Each command file has its own `#[cfg(test)]` module.

## Release

- Push `v*` tag to trigger cross-platform build (Linux, macOS, Windows).
- Uses `dtolnay/rust-toolchain` and `taiki-e/install-action` (cross).
- **Do not use** `actions-rs/*` — deprecated.

## Conventions

- Athlete ID is the owner's ID. **Never** hardcode it in README,
  examples, or public docs — use `<athlete-id>`.
- `skills/`, `node_modules/`, `TODO.md` are excluded from markdownlint
  (see `.markdownlintignore`).
- `package.json` exists only for dev tooling (secretlint, markdownlint-cli).
