# Zellij New-Tab Plus

![demo](https://raw.githubusercontent.com/AlexZasorin/zellij-newtab-plus/refs/heads/trunk/demo.gif)

## About

This plugin allows you to quickly name a new tab when creating it and to set
it's directory using `zoxide`.

Requires Zellij >= 0.43.1.

## Features

- `zoxide` integration to navigate new tabs based on name
- Tab name history (like shell history!)
  - NOTE: The latest version of Zellij (0.43.1) (contains a bug that causes
    the history to NOT persist across Zellij sessions. This is fixed on
    Zellij main and will be included in the next (0.44.0) release)[https://github.com/zellij-org/zellij/issues/4776#issuecomment-3986803120].
- Convenient keybinds

## Example Configuration

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.5.0/zellij-newtab-plus.wasm" {
        floating true

        use_zoxide true
    };
}
```

You can also download the plugin binary and reference it locally:

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "file:/path/to/zellij-newtab-plus.wasm" {
        floating true

        use_zoxide true
    };
}
```

If you want to bind it to a particular mode, you will need to make sure you
switch back to normal mode after launching the plugin:

```kdl
tab {
    bind "n" {
        LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.5.0/zellij-newtab-plus.wasm" {
            floating true

            use_zoxide true
        };
        SwitchToMode "Normal"
    }
}
```

## Usage

1. Press the keybinding you set up to launch the plugin (e.g., `Ctrl+n`).

2. Type your desired tab name OR press up to select a previous entry.

3. Press `Enter` to create the new tab with the specified name.

Additional keybinds:

- `Esc`, `Ctrl + c`, or `Ctrl + d`: Close the plugin.
- `Alt + Backspace`: Delete previous word.
- `Up`, `Down`: Navigate tab history
- `Alt + d`: Delete history entry or clear input.
