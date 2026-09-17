# Repository Guidelines

## Project Structure & Module Organization

Keyarchy is a Rust desktop application built with Iced. `src/main.rs` parses CLI options and starts the application. Keep feature code in the existing focused modules:

- `src/app/`: application state, messages, subscriptions, and update logic.
- `src/ui/`: Learn, Practice, Weak Keys, Progress, and Settings views.
- `src/omarchy/`: Hyprland configuration and binding parsing.
- `src/input/`, `src/learning/`, `src/storage/`: hotkey normalization, scoring/selection, and local progress persistence.
- `assets/`: desktop integration files; `packaging/arch/`: Arch package recipe.
- `.github/workflows/`: CI and release automation.

Unit tests live beside the code they cover in `#[cfg(test)]` modules. The `examples/`, `examples2/`, `yaml/`, `PLUGIN-command_to_file/`, and `other/` directories contain ancillary imported examples or tools; avoid coupling application code to them.

## Build, Test, and Development Commands

- `cargo run --release` runs Keyarchy against `~/.config/hypr/hyprland.conf`.
- `cargo run --release -- --config /path/to/hyprland.conf` uses another file or configuration directory.
- `cargo build --locked` builds with the committed dependency lockfile.
- `cargo test --locked --all-targets` runs all unit tests.
- `cargo fmt --all --check` verifies Rust formatting.
- `cargo clippy --locked --all-targets --all-features -- -D warnings` applies the same strict lint gate as CI.

Run formatting, Clippy, and tests before opening a pull request.

## Coding Style & Naming Conventions

Use standard `rustfmt` output and four-space indentation. Follow Rust naming conventions: `snake_case` for modules, functions, and tests; `PascalCase` for structs and enums; `SCREAMING_SNAKE_CASE` for constants. Prefer small module-local helpers and existing types over new abstractions. Surface filesystem errors with useful path context.

## Testing Guidelines

Add a focused unit test for parser, scoring, persistence, or selection behavior whenever logic changes. Name tests after observable behavior, such as `fast_correct_attempt_gets_bonus`. Use `tempfile` for filesystem tests and avoid depending on the user's Hyprland configuration. No coverage threshold is enforced, but changed branches should be exercised.

## Commit & Pull Request Guidelines

History follows concise Conventional Commit-style subjects: `feat: ...`, `fix(hotkeys): ...`, and `ci: ...`. Keep each commit scoped to one change. Pull requests should explain the user-visible effect, list verification commands, and link relevant issues. Include screenshots for UI changes and call out changes to progress-file compatibility, permissions, packaging, or release behavior.

## Security & Configuration

Do not add global keyboard capture or network access without explicit review. Preserve focused-window input handling and atomic writes to the XDG-local `progress.json`. Never commit personal Hyprland configuration or generated `target/` artifacts.
