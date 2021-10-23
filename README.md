## Run code
`cargo run`, `cargo watch -x run` for automatic recompilation.

## Clippy
You can mute a warning for the affected code block with  `#[allow(clippy::lint_name)]` and `#![allow(clippy::lint_name)]` for a project level directive.

`cargo clippy` to run, fail linter check if there is warning with `cargo clippy -- -D warnings`

## Rustfmt
`cargo fmt` to format the code, CI pipeline: `cargo fmt -- --check`

## Cargo Audit
`cargo audit` for both CI and local.