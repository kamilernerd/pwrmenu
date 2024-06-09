## Pwrmenu
A power menu built with GTK4. Works with wayland, supports theming and custom screen locker command.

## Requirements
Rust v1.76
Cargo v1.76

## Installation
Clone the project
```git clone git@github.com:kamilernerd/pwrmenu.git```

Simply run following in the project root
```./install.sh```

If the script asks for password, provide it. The binary expects to have
CAP_SYS_BOOT capabilities.
Read more here: https://man7.org/linux/man-pages/man7/capabilities.7.html

The binary will be installed in $HOME/.cargo/bin/pwrmenu
Now run ```pwrmenu```

When executed for the first time a ```theme.css``` and a ```config.json``` file will be created at ```$HOME/.config/pwrmenu```.
These files contain the theme which can be modified or turned off in the config file.
You can also provide custom screen-lock command in the config file if you're using a different screen-locker.
