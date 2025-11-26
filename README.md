# Zellij New-Tab Plus

![demo](https://raw.githubusercontent.com/AlexZasorin/zellij-newtab-plus/refs/heads/trunk/demo.gif)

## About

This plugin allows you to quickly name a new tab when creating it and to set
it's directory using `zoxide`.

## Example Configuration

```kdl
bind "Ctrl n" {
    LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.2.1/zellij-newtab-plus.wasm" {
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
        LaunchOrFocusPlugin "https://github.com/AlexZasorin/zellij-newtab-plus/releases/download/v0.2.1/zellij-newtab-plus.wasm" {
            floating true

            use_zoxide true
        };
        SwitchToMode "Normal"
    }
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
