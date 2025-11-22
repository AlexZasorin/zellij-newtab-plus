# Zellij New-Tab Plus

![demo](https://github.com/user-attachments/assets/3cf817c2-83b6-49c6-a0a0-4b83afc5baa6)

## About

This plugin allows you to name a new tab when creating it in Zellij, instead of
having to hit another keybinding to rename it afterward. Soon, this will also
include `zoxide` integration to quickly create tabs and navigate to the result
of the zoxide query.

## Example Configuration

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.1.1/zellij-newtab-plus.wasm" {
        floating true
    };
}
```

You can also download the plugin binary and reference it locally:

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "file:/path/to/zellij-newtab-plus.wasm" {
        floating true
    };
}
```

## Usage

1. Press the keybinding you set up to launch the plugin (e.g., `Ctrl+n`).

2. Type your desired tab name.

3. Press `Enter` to create the new tab with the specified name.

Additional keybinds:

- `Esc`: Close the plugin.
