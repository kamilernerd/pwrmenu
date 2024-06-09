## Pwrmenu
A power menu build with GTK4. Works with wayland, supports theming and custom
lock commands.

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
