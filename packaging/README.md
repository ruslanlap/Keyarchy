# Distribution

Keyarchy targets two complementary distribution channels. Neither the AUR
package nor the Omarchy plugin repository is published yet; the commands marked
as eventual will work only after their respective releases.

## Arch Linux and AUR

The package recipe lives in `packaging/arch/`. From an Arch Linux checkout,
build and inspect it locally before publishing:

```bash
cd packaging/arch
makepkg --syncdeps --install
namcap PKGBUILD keyarchy-*.pkg.tar.zst
```

Before every release, update `pkgver`, reset `pkgrel` to `1`, replace the source
checksum, and regenerate `.SRCINFO`:

```bash
makepkg --printsrcinfo > .SRCINFO
```

Commit `PKGBUILD` and `.SRCINFO` to the separate AUR Git repository only after
the matching `vX.Y.Z` source tag exists. Once published, users can install with
an AUR helper:

```bash
yay -S keyarchy
```

Plain `sudo pacman -S keyarchy` requires Keyarchy to enter an official Arch
repository, or users to configure a signed third-party repository first. Do not
advertise that command as available until one of those paths is live.

## Omarchy Plugin

The plugin should remain a small, separate repository that launches the
installed `keyarchy` binary and optionally displays progress. It must not
download binaries or run `sudo`; when Keyarchy is missing, show installation
instructions instead.

Develop the plugin against a local Keyarchy build:

```bash
cargo build --release
./target/release/keyarchy
omarchy plugin validate /path/to/omarchy-keyarchy-plugin
```

Test installation from the plugin checkout before publishing it. After its Git
repository is public, the eventual user command is:

```bash
omarchy plugin add https://github.com/ruslanlap/omarchy-keyarchy-plugin.git --enable
```

Keep releases independent: the Arch package owns the application and desktop
entry; the Omarchy plugin only integrates and launches it.
