# Zellij New-Tab Plus

![demo](https://raw.githubusercontent.com/AlexZasorin/zellij-newtab-plus/refs/heads/trunk/demo.gif)

## About

This plugin allows you to name a new tab when creating it in Zellij, instead of
having to hit another keybinding to rename it afterward. If `zoxide` is
installed, it will be used to set the directory of the new tab.

## Example Configuration

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.2.1/zellij-newtab-plus.wasm" {
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

## To Do

- [ ] Add configuration option to enable/disable `zoxide` navigation
