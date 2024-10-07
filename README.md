## Pwrmenu

A power menu built with GTK4. Works with wayland/xwayland, supports theming and custom screen locker command.

## Requirements

- Rust v1.81
- Cargo v1.81

## Development dependencies

- gtk4-devel
- gtk4-layer-shell-devel

## Installation

Clone the project
`git clone git@github.com:kamilernerd/pwrmenu.git`

Simply run following in the project root
`./install.sh`

If the script asks for password, provide it. The binary expects to have
CAP_SYS_BOOT capabilities.

Read more here: https://man7.org/linux/man-pages/man7/capabilities.7.html

The binary will be installed in $HOME/.cargo/bin/pwrmenu

Now run `pwrmenu`

When executed for the first time a `theme.css` and a `config.json` file will be created at `$HOME/.config/pwrmenu`.

These files contain the theme which can be modified or turned off in the config file.

You can also provide custom screen-lock command in the config file if you're using a different screen-locker.

## Preview

Running on Fedora 40 (Gnome) with Hyprland (xwayland).

![alt text](https://github.com/kamilernerd/pwrmenu/blob/master/Screenshot%20from%202024-06-10%2000-09-09.png?raw=true)

## Options

```
{
    "use_system_theme": true,          // Use default system theme or allow for theme.css to override
    "lock_screen": "",                 // Custom command to trigger lock-screen
    "buttons_layout": {
        "orientation": "horizontal",   // horizontal | vertical
        "vertical_align": "center",    // start | end | center | baseline
        "horizontal_align": "center"   // start | end | center | baseline
    },
    "anchor": {
        "left": true,                  // anchor window to left
        "right": true,                 // anchor window to right
        "top": false,                  // anchor window to top
        "bottom": true                 // anchor window to bottom
    },
    "size": {
        "width": "500",                // "number" | "screen" uses screen width
        "height": "300"                // "number" | "screen" uses screen height
    }
}
```

## Theming

You can override the styles as you wish using gtk css https://docs.gtk.org/gtk4/css-properties.html .
Including each button separately. Simply reference their ID in the css.

Button ids:

- lock
- logout
- suspend
- reboot
- shutdown

Example:

```
#lock {
    background-color: red;
}

#lock:hover {
    background-color: pink;
}
```
