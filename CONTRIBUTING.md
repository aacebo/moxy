# Contributing To Moxy

Contributions are welcome as bug reports, feature requests, documentation
improvements, and pull requests.

## Before Opening A Pull Request

Open an issue before implementing a substantial change to a public API, parser
behavior, architecture, CI, or project-wide behavior. Keep pull requests small
and focused so they can be reviewed independently.

For bug fixes, include a minimal reproduction and a regression test. For new
behavior, explain the problem, the proposed API or behavior, and alternatives
considered.

## Validation

Run the relevant checks before submitting a pull request:

```console
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo doc --workspace --all-features --no-deps --locked
```

## Pull Requests

Describe the problem, the approach, and the validation performed. Do not mix
unrelated changes in one pull request. Public API changes should identify
compatibility implications.

Moxy's [AI-assisted contribution policy](AI_POLICY.md) applies to all issues,
pull requests, and review discussions.
