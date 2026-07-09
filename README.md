# rebound

TUI tool to set up 0xClumzZy's Arch Linux environment on any PC.

## Requirements
A pc booted up with Arch linux

## Features

- Interactive TUI with dark Catppuccin theme
- 7 setup categories: Base Packages, AUR Helper, BlackArch, Dev Tools, Shell Config, Desktop, Services
- Non-interactive `--auto` mode for full install
- Embedded dotfiles (zsh, starship, zoxide, noctalia shell, KDE, btop, terminator)

## Install

### Auto (curl)

```bash
curl -sL https://raw.githubusercontent.com/0xClumzzy/rebound-/main/install.sh | bash
```

### Manual

```bash
git clone https://github.com/0xClumzzy/rebound-.git
cd rebound-
cargo build --release
cp target/release/rebound ~/.local/bin/
```

### Requirements

- Arch Linux (or Arch-based distro)
- Rust/Cargo (install script handles this)

## Usage

```bash
rebound           # Interactive mode
rebound --auto    # Install everything automatically
rebound --help    # Show help
```

## What it installs

| Category | Packages |
|----------|----------|
| Base Packages | git, curl, wget, vim, nano, htop, btop, unzip, base-devel |
| AUR Helper | paru |
| BlackArch | Full repo setup |
| Dev Tools | rustup, nodejs, npm, python, docker, go |
| Shell Config | zsh, oh-my-zsh, starship, zoxide, thefuck |
| Desktop | Noctalia Shell, KDE Plasma, Wayland |
| Services | NetworkManager, Bluetooth, docker |

## License

MIT
