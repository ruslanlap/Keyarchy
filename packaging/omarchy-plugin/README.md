# Keyarchy Omarchy Plugin

A thin Omarchy bar widget for launching an already-installed `keyarchy`
application. It does not install packages, download files, or invoke `sudo`.

## Local development

Validate the plugin from the Keyarchy checkout:

```bash
omarchy plugin validate ./packaging/omarchy-plugin
```

For a manual installation, copy this directory to Omarchy's user plugin path,
rescan, and enable it:

```bash
cp -r packaging/omarchy-plugin ~/.config/omarchy/plugins/dev.keyarchy.launcher
omarchy-shell shell rescanPlugins
omarchy plugin enable dev.keyarchy.launcher
```

The widget checks whether `keyarchy` is on `PATH`. Clicking **Open Keyarchy**
only launches that binary; a missing installation is reported in the panel.

## Publishing

`omarchy plugin add URL` expects `manifest.json` at the root of a Git
repository. Publish the contents of this directory as a separate plugin
repository before advertising a command such as:

```bash
omarchy plugin add https://github.com/ruslanlap/omarchy-keyarchy-plugin.git --enable
```
