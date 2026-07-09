# rebound

TUI tool to set up 0xClumzZy's Arch Linux environment on any PC.

```
  ╔══════════════════════════════════════╗
  ║ 0x██╗  REBOUND  v1.0                ║
  ║   ╚╝╚═══╝                           ║
  ║   Made by 0xClumzZy                 ║
  ║   @github.com/0xClumzzy             ║
  ╚══════════════════════════════════════╝
```

## Requirements

- Arch Linux (or Arch-based distro like EndeavourOS, Garuda, Manjaro)
- Internet connection
- ~500MB disk space for installed packages

## Features

- Interactive TUI with dark Catppuccin-inspired theme
- 8 setup categories with granular task selection
- Non-interactive `--auto` mode for full install
- Embedded dotfiles deployed automatically
- 34 dark aesthetic wallpapers included
- Password prompt with hidden input (stars)
- Live progress bar with animated spinner
- Sudo cached at launch for passwordless installs

## Install

### One-liner (recommended)

```bash
curl -sL https://raw.githubusercontent.com/0xClumzzy/rebound-/main/install.sh | bash
```

### Direct binary download

```bash
curl -sL https://github.com/0xClumzzy/rebound-/releases/download/v1.0.0/rebound-x86_64 -o ~/.local/bin/rebound
chmod +x ~/.local/bin/rebound
```

### Build from source

```bash
git clone https://github.com/0xClumzzy/rebound-.git
cd rebound-
cargo build --release
cp target/release/rebound ~/.local/bin/
```

### Ensure `~/.local/bin` is in your PATH

```bash
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## Usage

```bash
rebound              # Interactive TUI mode
rebound --auto       # Install everything automatically (no TUI)
rebound -a           # Same as --auto
rebound --help       # Show help message
rebound -h           # Same as --help
```

### Interactive mode

1. Launch `rebound`
2. Enter sudo password when prompted (shown as `***`)
3. Browse categories with arrow keys
4. Select/deselect tasks with spacebar
5. Press enter to start installation
6. Watch live progress with output log

### Auto mode

```bash
rebound --auto
```

Selects all tasks and runs immediately without any interaction. Useful for scripting or automated setups.

## Categories

### Base Packages

Core system utilities and tools.

| Task | Packages |
|------|----------|
| Core tools | base-devel, git, curl, wget, bat, eza, fzf, ripgrep, btop, fastfetch, tmux, trash-cli |
| Shell | zsh, zsh-autosuggestions, zsh-syntax-highlighting, starship, thefuck |
| Editors | micro, vim |
| Networking | NetworkManager, nmap, smbclient, dnsutils |
| System | polkit, udisks2, pavucontrol, pulseaudio, bluez, cups |
| Fonts | Noto, JetBrains Mono Nerd, FiraCode Nerd, DejaVu, Liberation |

### AUR Helper

| Task | Description |
|------|-------------|
| Install paru | Builds paru AUR helper from source |
| AUR packages | noctalia-shell-bin, papirus-icon-theme, terminator |

### BlackArch

| Task | Description |
|------|-------------|
| Setup repo | Adds BlackArch penetration testing repository |
| Pentest tools | aircrack-ng, gobuster, ffuf, nmap, sqlmap, nikto, hydra, john, hashcat |

### Dev Tools

| Task | Description |
|------|-------------|
| Rust | Installs via rustup |
| Node.js | nvm + latest LTS |
| Go | Install via pacman |
| Python | Python 3, pip, pipx |
| Ruby | Install via pacman |
| Build tools | gcc, g++, cmake, make, clang |

### Shell Config

| Task | Description |
|------|-------------|
| Oh My Zsh | Installs Zsh framework |
| .zshrc | Deploys zsh config with aliases & functions |
| .gitconfig | Sets git identity (0xClumzZy) |
| starship.toml | Catppuccin prompt config |
| .bashrc | Deploys bash config |

### Desktop

| Task | Description |
|------|-------------|
| Noctalia Shell | Wayland desktop shell (AUR) |
| KDE Plasma | Full KDE desktop with Wayland |
| KWin | Window manager config |
| btop | Noctalia-themed system monitor |
| Terminator | Transparent terminal + Gruvbox palette |

### Services

| Task | Service |
|------|---------|
| NetworkManager | `systemctl enable --now NetworkManager` |
| Bluetooth | `systemctl enable --now bluetooth` |
| CUPS | `systemctl enable --now cups` |
| Samba | `systemctl enable --now smb nmb` |
| Ollama | `systemctl enable --now ollama` |

### Wallpapers

Deploys 34 dark aesthetic wallpapers to `~/Pictures/wallpapers/`.

## Embedded Dotfiles

All dotfiles are embedded in the binary and deployed automatically:

| File | Location | Description |
|------|----------|-------------|
| `.zshrc` | `~/.zshrc` | Zsh config with aliases, functions, plugin loader |
| `.bashrc` | `~/.bashrc` | Bash config with aliases and functions |
| `starship.toml` | `~/.config/starship.toml` | Catppuccin-prompt theme |
| `.gitconfig` | `~/.gitconfig` | Git identity (0xClumzZy) |
| `noctalia.toml` | `~/.config/noctalia/noctalia.toml` | Noctalia Shell config |
| `kdeglobals` | `~/.config/kdeglobals` | KDE appearance settings |
| `kwinrc` | `~/.config/kwinrc` | KWin window manager config |
| `btop.conf` | `~/.config/btop/btop.conf` | System monitor theme |
| `terminator` | `~/.config/terminator/config` | Terminal emulator config |

## Key Aliases (from .zshrc)

| Alias | Command |
|-------|---------|
| `update` | `sudo pacman -Syu --noconfirm` |
| `get` | `sudo pacman -Sy --noconfirm` |
| `need` | `sudo pacman -Sy --noconfirm` |
| `fuckup` | `sudo pacman -Rns --noconfirm` |
| `lookfor` | `sudo pacman -Ss` |
| `fucking` | `sudo` |
| `rebootsafe` | `sudo shutdown -r now` |
| `network` | `sudo systemctl start --now NetworkManager` |

## Theme

Dark Catppuccin-inspired palette:

| Name | Hex | Usage |
|------|-----|-------|
| Crust | `#0a0a0f` | Background |
| Surface0 | `#1a1a2e` | Card background |
| Surface2 | `#3a3a5c` | Borders |
| Text | `#e0e0f0` | Primary text |
| Subtext0 | `#808098` | Secondary text |
| Overlay0 | `#606078` | Dimmed text |
| Maue | `#cba6f7` | Accent/highlight |
| Green | `#a6e3a1` | Success |
| Red | `#f38ba8` | Error |
| Yellow | `#f9e2af` | Warning |
| Blue | `#89b4fe` | Links/info |

## Uninstall

```bash
rm ~/.local/bin/rebound
```

Dotfiles are deployed to standard locations and can be removed manually.

## License

MIT

## Author

**0xClumzZy** - [github.com/0xClumzzy](https://github.com/0xClumzzy)
