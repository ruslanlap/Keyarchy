# Keyarchy

Keyarchy is a Wayland-first desktop trainer for learning Omarchy and Hyprland
keyboard shortcuts. It reads the shortcuts you actually use, presents focused
practice challenges, and stores progress locally.

## Features

- Native Rust desktop UI powered by [Iced](https://iced.rs/).
- Parses `bind*` declarations, variables, and recursively sourced Hyprland
  configuration files.
- Captures shortcuts only while the trainer has focus—no global keylogger and
  no compositor-specific permissions.
- Adaptive practice prioritizes shortcuts with the lowest mastery.
- XP, response-time scoring, streaks, per-shortcut accuracy, and weak-key view.
- XDG-compliant JSON progress storage with atomic writes.
- Dark Tokyo Night theme suitable for the Omarchy desktop.

## Install and run

Install the Arch build dependencies and run from source:

```bash
sudo pacman -S --needed base-devel rust
cargo run --release
```

By default Keyarchy reads `~/.config/hypr/hyprland.conf`. Pass a different
file or Hyprland configuration directory when needed:

```bash
cargo run --release -- --config ~/.config/hypr/hyprland.conf
```

While practicing, keep the Keyarchy window focused. The compositor may still
consume reserved shortcuts before the application receives them. For a fully
isolated session, use a temporary Hyprland submap or duplicate a binding with
an unreserved training combination.

## Data and privacy

Keyarchy does not use network access and does not capture keyboard input while
unfocused. Progress is saved under the platform XDG data directory, normally:

```text
~/.local/share/keyarchy/Keyarchy/progress.json
```

## Development

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

The application is split into state/update logic, UI pages, input
normalization, Hyprland parsing, learning/scoring, and persistence modules under
`src/`.

## CI/CD

The GitHub Actions pipelines use a small set of explicit quality gates:

1. **CI** runs on pull requests, pushes to `main`/`work`, and manual dispatches.
   Formatting and Clippy run in parallel with the unit tests. Only after both
   jobs pass are release binaries built for Linux x86-64 and ARM64 and retained
   as workflow artifacts for seven days.
2. **Release validation** starts for semantic-version tags such as `v0.1.0`.
   It verifies that the tag matches the version in `Cargo.toml`, then repeats
   formatting, lint, and test gates against the tagged source.
3. **Packaging** creates per-architecture tarballs containing the executable,
   desktop entry, README, and license. Every archive gets a SHA-256 checksum.
4. **Publishing** downloads the packaged artifacts and creates the GitHub
   Release with generated release notes. Only this final job receives
   `contents: write`; all other jobs are read-only.

To publish a release, update `Cargo.toml` and `Cargo.lock`, merge the change,
and push the matching tag:

```bash
git tag -s v0.1.0 -m "Keyarchy v0.1.0"
git push origin v0.1.0
```

The release workflow can also be run manually for an existing matching tag.

## Distribution

The validated Arch package recipe and Omarchy plugin scaffold live under
`packaging/`. See [`packaging/README.md`](packaging/README.md) for local checks,
release steps, and the current publication status of both channels.

## License

MIT
