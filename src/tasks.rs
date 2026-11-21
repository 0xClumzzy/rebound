use std::fs;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Task {
    pub name: String,
    pub desc: String,
    pub selected: bool,
    pub commands: Vec<String>,
    pub deploy_files: Vec<(String, String)>, // (relative_path, content)
    pub is_wallpapers: bool,
}

#[derive(Clone)]
pub struct Category {
    pub name: String,
    pub icon: String,
    pub tasks: Vec<Task>,
}

impl Task {
    pub fn new(name: &str, desc: &str, commands: Vec<&str>) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            selected: false,
            commands: commands.into_iter().map(String::from).collect(),
            deploy_files: Vec::new(),
            is_wallpapers: false,
        }
    }

    pub fn with_files(mut self, files: Vec<(&str, &str)>) -> Self {
        self.deploy_files = files.into_iter().map(|(p, c)| (p.to_string(), c.to_string())).collect();
        self
    }

    pub fn with_wallpapers(mut self) -> Self {
        self.is_wallpapers = true;
        self
    }

    pub fn deploy(&self, home: &str) -> bool {
        let mut ok = true;
        for (rel_path, content) in &self.deploy_files {
            let path = PathBuf::from(home).join(rel_path);
            if let Some(parent) = path.parent() {
                if fs::create_dir_all(parent).is_err() {
                    ok = false;
                    continue;
                }
            }
            if fs::write(&path, content).is_err() {
                ok = false;
            }
        }
        ok
    }
}

fn zshrc() -> &'static str {
    include_str!("dotfiles/.zshrc")
}

fn bashrc() -> &'static str {
    include_str!("dotfiles/.bashrc")
}

fn starship() -> &'static str {
    include_str!("dotfiles/starship.toml")
}

fn gitconfig() -> &'static str {
    "[user]\n\temail = sustee848@gmail.com\n\tname = 0xClumzzy\n[credential \"https://github.com\"]\n\thelper = \n\thelper = !/usr/bin/gh auth git-credential\n[credential \"https://gist.github.com\"]\n\thelper = \n\thelper = !/usr/bin/gh auth git-credential\n"
}

fn noctalia() -> &'static str {
    include_str!("dotfiles/noctalia.toml")
}

fn kdeglobals() -> &'static str {
    include_str!("dotfiles/kdeglobals")
}

fn kwinrc() -> &'static str {
    include_str!("dotfiles/kwinrc")
}

fn btop_conf() -> &'static str {
    include_str!("dotfiles/btop.conf")
}

fn terminator() -> &'static str {
    include_str!("dotfiles/terminator")
}

pub fn base_packages() -> Category {
    Category {
        name: "Base Packages".into(),
        icon: "⚡".into(),
        tasks: vec![
            Task::new("Core tools", "git, curl, wget, bat, eza, fzf, ripgrep, btop, fastfetch",
                vec!["sudo pacman -S --needed --noconfirm base-devel git curl wget unzip 7zip htop btop tree ripgrep fd fzf bat eza tmux fastfetch zoxide trash-cli man-db man-pages less"]),
            Task::new("Shell", "zsh, starship, autosuggestions, syntax-highlighting, thefuck",
                vec!["sudo pacman -S --needed --noconfirm zsh zsh-autosuggestions zsh-syntax-highlighting starship thefuck"]),
            Task::new("Terminal", "ghostty - GPU-accelerated terminal",
                vec!["paru -S --needed --noconfirm ghostty || true"]),
            Task::new("Editors", "micro, vim",
                vec!["sudo pacman -S --needed --noconfirm micro vim"]),
            Task::new("Networking", "NetworkManager, nmap, smbclient",
                vec!["sudo pacman -S --needed --noconfirm networkmanager nmap openbsd-netcat smbclient dnsutils"]),
            Task::new("System", "audio, bluetooth, printing, polkit",
                vec!["sudo pacman -S --needed --noconfirm sudo polkit udisks2 pavucontrol pulseaudio bluez bluez-utils cups"]),
            Task::new("Fonts", "Noto, JetBrains Mono Nerd, FiraCode Nerd",
                vec!["sudo pacman -S --needed --noconfirm ttf-dejavu ttf-liberation noto-fonts noto-fonts-emoji ttf-firacode-nerd ttf-jetbrains-mono-nerd"]),
        ],
    }
}

fn aur_helper() -> Category {
    Category {
        name: "AUR Helper".into(),
        icon: "🌟".into(),
        tasks: vec![
            Task::new("Install paru", "Build AUR helper from source",
                vec!["command -v paru || { git clone https://aur.archlinux.org/paru.git /tmp/paru && cd /tmp/paru && makepkg -si --noconfirm && rm -rf /tmp/paru; }"]),
            Task::new("AUR packages", "noctalia-shell, papirus-icons, terminator",
                vec!["paru -S --needed --noconfirm noctalia-shell-bin papirus-icon-theme terminator"]),
        ],
    }
}

fn blackarch() -> Category {
    Category {
        name: "BlackArch".into(),
        icon: "🕷️".into(),
        tasks: vec![
            Task::new("Setup repo", "Add BlackArch penetration testing repository",
                vec!["test -f /etc/pacman.d/blackarch-mirrorlist || { curl -Os https://blackarch.org/strap.sh && chmod +x strap.sh && sudo ./strap.sh && rm strap.sh; }"]),
            Task::new("Pentest tools", "aircrack-ng, gobuster, ffuf, nmap, sqlmap, nikto",
                vec!["sudo pacman -S --needed --noconfirm aircrack-ng cewl chisel gobuster feroxbuster ffuf nmap sqlmap nikto hydra john hashcat || true"]),
        ],
    }
}

fn devtools() -> Category {
    Category {
        name: "Dev Tools".into(),
        icon: "🛠️".into(),
        tasks: vec![
            Task::new("Rust", "Install via rustup",
                vec!["command -v rustc || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"]),
            Task::new("Node.js", "nvm + latest LTS",
                vec!["export NVM_DIR=\"$HOME/.nvm\"",
                     "test -s \"$NVM_DIR/nvm.sh\" || curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash",
                     ". \"$NVM_DIR/nvm.sh\" && nvm install --lts"]),
            Task::new("Go", "Install via pacman",
                vec!["sudo pacman -S --needed --noconfirm go"]),
            Task::new("Python", "Python 3, pip, pipx",
                vec!["sudo pacman -S --needed --noconfirm python python-pip python-pipx"]),
            Task::new("Ruby", "Install via pacman",
                vec!["sudo pacman -S --needed --noconfirm ruby"]),
            Task::new("Build tools", "gcc, g++, cmake, make, clang",
                vec!["sudo pacman -S --needed --noconfirm gcc g++ cmake make clang"]),
        ],
    }
}

fn shell_config() -> Category {
    Category {
        name: "Shell Config".into(),
        icon: "✨".into(),
        tasks: vec![
            Task::new("Oh My Zsh", "Install Zsh framework",
                vec!["test -d \"$HOME/.oh-my-zsh\" || sh -c \"$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)\" \"\" --unattended"]),
            Task::new(".zshrc", "Deploy zsh config with aliases & functions", vec![])
                .with_files(vec![(".zshrc", zshrc())]),
            Task::new(".gitconfig", "Set git identity", vec![])
                .with_files(vec![(".gitconfig", gitconfig())]),
            Task::new("starship.toml", "Catppuccin prompt config", vec![])
                .with_files(vec![(".config/starship.toml", starship())]),
            Task::new(".bashrc", "Deploy bash config", vec![])
                .with_files(vec![(".bashrc", bashrc())]),
        ],
    }
}

fn desktop_config() -> Category {
    Category {
        name: "Desktop".into(),
        icon: "🌈".into(),
        tasks: vec![
            Task::new("Noctalia", "Desktop shell config", vec![])
                .with_files(vec![(".config/noctalia/config.toml", noctalia())]),
            Task::new("KDE colors", "Noctalia color scheme + Monochrome", vec![])
                .with_files(vec![(".config/kdeglobals", kdeglobals())]),
            Task::new("KWin", "Workspaces, wobbly windows, CurvedVolatile", vec![])
                .with_files(vec![(".config/kwinrc", kwinrc())]),
            Task::new("btop", "Noctalia-themed system monitor", vec![])
                .with_files(vec![(".config/btop/btop.conf", btop_conf())]),
            Task::new("Terminator", "Transparent terminal + Gruvbox palette", vec![])
                .with_files(vec![(".config/terminator/config", terminator())]),
        ],
    }
}

fn services_config() -> Category {
    Category {
        name: "Services".into(),
        icon: "🚀".into(),
        tasks: vec![
            Task::new("NetworkManager", "Enable network management",
                vec!["sudo systemctl enable --now NetworkManager"]),
            Task::new("Bluetooth", "Enable bluetooth service",
                vec!["sudo systemctl enable --now bluetooth"]),
            Task::new("CUPS", "Enable printing",
                vec!["sudo systemctl enable --now cups"]),
            Task::new("Samba", "Enable SMB/NMB file sharing",
                vec!["sudo systemctl enable --now smb nmb"]),
            Task::new("Ollama", "Enable local AI service",
                vec!["sudo systemctl enable --now ollama || true"]),
        ],
    }
}

fn pipx_apps() -> Category {
    Category {
        name: "Pipx Apps".into(),
        icon: "🧪".into(),
        tasks: vec![
            Task::new("Install pipx", "Python app installer",
                vec!["sudo pacman -S --needed --noconfirm python-pipx || pip install pipx && pipx ensurepath"]),
            Task::new("Ranger", "Terminal file manager",
                vec!["pipx install ranger-fm"]),
            Task::new("Tavily CLI", "AI search tool",
                vec!["pipx install tavily-cli || true"]),
            Task::new("Glance", "RSS/news reader",
                vec!["pipx install glance || true"]),
            Task::new("Httpie", "HTTP client",
                vec!["pipx install httpie"]),
            Task::new("Bpytop", "Resource monitor",
                vec!["pipx install bpytop"]),
        ],
    }
}

fn kde_install() -> Category {
    Category {
        name: "KDE Plasma".into(),
        icon: "💎".into(),
        tasks: vec![
            Task::new("Plasma Desktop", "Full KDE Plasma desktop environment",
                vec!["sudo pacman -S --needed --noconfirm plasma plasma-wayland-session kde-gtk-config breeze-gtk sddm"]),
            Task::new("KDE Apps", "dolphin, konsole, kate, ark, spectacle, okular",
                vec!["sudo pacman -S --needed --noconfirm dolphin konsole kate ark spectacle okular gwenview kcalc"]),
            Task::new("KDE Extras", "Widgets, themes, system settings",
                vec!["sudo pacman -S --needed --noconfirm plasma-nvidia powerdevil bluedevil kinfocenter partitionmanager"]),
            Task::new("Display manager", "Enable SDDM login screen",
                vec!["sudo systemctl enable sddm"]),
            Task::new("Wayland session", "Enable Plasma Wayland",
                vec!["sudo ln -sf /usr/share/wayland-sessions/plasma-wayland.desktop /usr/share/wayland-sessions/default-wayland.desktop || true"]),
        ],
    }
}

fn wallpapers_config() -> Category {
    Category {
        name: "Wallpapers".into(),
        icon: "🎭".into(),
        tasks: vec![
            Task::new("Wallpapers", "Deploy 34 dark aesthetic wallpapers to ~/Pictures/wallpapers", vec![])
                .with_wallpapers(),
        ],
    }
}

fn updates_upgrades() -> Category {
    Category {
        name: "Updates & Upgrades".into(),
        icon: "🔥".into(),
        tasks: vec![
            Task::new("System update", "Full system upgrade with pacman",
                vec!["sudo pacman -Syu --noconfirm"]),
            Task::new("AUR packages update", "Upgrade AUR packages with paru",
                vec!["paru -Syu --noconfirm || true"]),
            Task::new("Flatpak update", "Update Flatpak applications",
                vec!["flatpak update -y || true"]),
            Task::new("Rust update", "Update Rust toolchain via rustup",
                vec!["rustup update || true"]),
            Task::new("Node.js update", "Update Node.js via nvm",
                vec!["export NVM_DIR=\"$HOME/.nvm\"", ". \"$NVM_DIR/nvm.sh\" && nvm install --lts --reinstall-packages-from=current || true"]),
            Task::new("Pipx upgrade all", "Upgrade all pipx-installed applications",
                vec!["pipx upgrade-all || true"]),
            Task::new("Clean package cache", "Remove orphaned packages and clean cache",
                vec!["sudo pacman -Rns $(pacman -Qtdq) --noconfirm 2>/dev/null || true", "sudo pacman -Sc --noconfirm", "paru -Sc --noconfirm || true"]),
        ],
    }
}

fn gaming() -> Category {
    Category {
        name: "Gaming".into(),
        icon: "🕹️".into(),
        tasks: vec![
            Task::new("Steam", "Steam client + Proton support",
                vec!["sudo pacman -S --needed --noconfirm steam"]),
            Task::new("Lutris", "Game manager for Linux",
                vec!["sudo pacman -S --needed --noconfirm lutris"]),
            Task::new("Wine & Proton", "Wine, winetricks, proton-ge",
                vec!["sudo pacman -S --needed --noconfirm wine winetricks giflib lib32-giflib libpng lib32-libpng libldap lib32-libldap gnutls lib32-gnutls mpg123 lib32-mpg123 openal lib32-openal v4l-utils lib32-v4l-utils libpulse lib32-libpulse libgpg-error lib32-libgpg-error alsa-plugins lib32-alsa-plugins alsa-lib lib32-alsa-lib libjpeg-turbo lib32-libjpeg-turbo sqlite lib32-sqlite libxcomposite lib32-libxcomposite libxinerama lib32-libxinerama ncurses lib32-ncurses opencl-icd-loader lib32-opencl-icd-loader libxslt lib32-libxslt libva lib32-libva gtk3 lib32-gtk3 gst-plugins-base-libs lib32-gst-plugins-base-libs vulkan-icd-loader lib32-vulkan-icd-loader"]),
            Task::new("Gaming tools", "MangoHud, goverlay, gamescope",
                vec!["sudo pacman -S --needed --noconfirm mangohud goverlay gamescope || true"]),
            Task::new("Discord", "Discord client",
                vec!["paru -S --needed --noconfirm discord || true"]),
            Task::new("OBS Studio", "Streaming/recording",
                vec!["sudo pacman -S --needed --noconfirm obs-studio"]),
        ],
    }
}

fn containers_cloud() -> Category {
    Category {
        name: "Containers & Cloud".into(),
        icon: "☁️".into(),
        tasks: vec![
            Task::new("Docker", "Docker Engine + Docker Compose",
                vec!["sudo pacman -S --needed --noconfirm docker docker-compose", "sudo systemctl enable --now docker", "sudo usermod -aG docker $USER"]),
            Task::new("Podman", "Rootless container engine",
                vec!["sudo pacman -S --needed --noconfirm podman podman-compose"]),
            Task::new("Kubernetes CLI", "kubectl, helm, k9s",
                vec!["sudo pacman -S --needed --noconfirm kubectl helm k9s || true"]),
            Task::new("Container tools", "ctop, dive, lazydocker",
                vec!["paru -S --needed --noconfirm ctop dive lazydocker || true"]),
            Task::new("Distrobox", "Container-based dev environments",
                vec!["sudo pacman -S --needed --noconfirm distrobox || true"]),
        ],
    }
}

fn security_privacy() -> Category {
    Category {
        name: "Security & Privacy".into(),
        icon: "🛡️".into(),
        tasks: vec![
            Task::new("Firewall", "UFW + GUFW",
                vec!["sudo pacman -S --needed --noconfirm ufw gufw", "sudo ufw enable"]),
            Task::new("VPN", "WireGuard, OpenVPN, NetworkManager VPN",
                vec!["sudo pacman -S --needed --noconfirm wireguard-tools openvpn networkmanager-openvpn networkmanager-wireguard"]),
            Task::new("Encryption", "VeraCrypt, Cryptsetup, GnuPG",
                vec!["sudo pacman -S --needed --noconfirm veracrypt cryptsetup gnupg"]),
            Task::new("Password manager", "Bitwarden CLI, KeePassXC",
                vec!["sudo pacman -S --needed --noconfirm bitwarden-cli keepassxc || true"]),
            Task::new("Hardening", "AppArmor, auditd, lynis",
                vec!["sudo pacman -S --needed --noconfirm apparmor audit lynis", "sudo systemctl enable --now apparmor"]),
            Task::new("Privacy tools", "Tor, torsocks, onionshare",
                vec!["sudo pacman -S --needed --noconfirm tor torsocks onionshare || true"]),
        ],
    }
}

fn media_creative() -> Category {
    Category {
        name: "Media & Creative".into(),
        icon: "🎬".into(),
        tasks: vec![
            Task::new("Video", "mpv, ffmpeg, handbrake",
                vec!["sudo pacman -S --needed --noconfirm mpv ffmpeg handbrake"]),
            Task::new("Audio", "pipewire, pulseeffects, audacity",
                vec!["sudo pacman -S --needed --noconfirm pipewire wireplumber pipewire-pulse pavucontrol pulseeffects audacity"]),
            Task::new("Graphics", "GIMP, Inkscape, Krita, Blender",
                vec!["sudo pacman -S --needed --noconfirm gimp inkscape krita blender || true"]),
            Task::new("Screen recording", "OBS, wf-recorder, simplescreenrecorder",
                vec!["sudo pacman -S --needed --noconfirm obs-studio wf-recorder simplescreenrecorder || true"]),
            Task::new("Image tools", "ImageMagick, exiftool, gthumb",
                vec!["sudo pacman -S --needed --noconfirm imagemagick exiftool gthumb"]),
        ],
    }
}

fn devtools_extended() -> Category {
    Category {
        name: "Dev Tools Extended".into(),
        icon: "🧰".into(),
        tasks: vec![
            Task::new("Databases", "PostgreSQL, MySQL, Redis, SQLite",
                vec!["sudo pacman -S --needed --noconfirm postgresql mysql redis sqlite"]),
            Task::new("Database tools", "DBeaver, pgAdmin, RedisInsight",
                vec!["paru -S --needed --noconfirm dbeaver pgadmin4 redisinsight || true"]),
            Task::new("API testing", "Postman, Insomnia, HTTPie",
                vec!["paru -S --needed --noconfirm postman-bin insomnia httpie || true"]),
            Task::new("Neovim", "Modern Vim + LSP config",
                vec!["sudo pacman -S --needed --noconfirm neovim"]),
            Task::new("VS Code", "Code editor",
                vec!["paru -S --needed --noconfirm visual-studio-code-bin || true"]),
            Task::new("Terminal tools", "zellij, tmux, wezterm",
                vec!["sudo pacman -S --needed --noconfirm zellij tmux wezterm || true"]),
            Task::new("Git tools", "lazygit, gitui, git-delta",
                vec!["sudo pacman -S --needed --noconfirm lazygit gitui git-delta || true"]),
            Task::new("Language servers", "rust-analyzer, gopls, pyright, typescript",
                vec!["sudo pacman -S --needed --noconfirm rust-analyzer gopls pyright typescript-language-server || true"]),
        ],
    }
}

fn system_hardening() -> Category {
    Category {
        name: "System Hardening".into(),
        icon: "🔒".into(),
        tasks: vec![
            Task::new("Kernel params", "Sysctl hardening",
                vec!["sudo tee /etc/sysctl.d/99-hardening.conf > /dev/null << 'EOF'", "kernel.unprivileged_bpf_disabled=1", "kernel.kptr_restrict=2", "kernel.dmesg_restrict=1", "kernel.yama.ptrace_scope=2", "vm.mmap_rnd_bits=32", "vm.mmap_rnd_compat_bits=16", "net.ipv4.tcp_syncookies=1", "net.ipv4.conf.all.rp_filter=1", "net.ipv4.conf.default.rp_filter=1", "net.ipv4.conf.all.accept_redirects=0", "net.ipv4.conf.default.accept_redirects=0", "net.ipv4.conf.all.send_redirects=0", "net.ipv4.icmp_echo_ignore_broadcasts=1", "net.ipv4.icmp_ignore_bogus_error_responses=1", "fs.protected_hardlinks=1", "fs.protected_symlinks=1", "EOF", "sudo sysctl --system"]),
            Task::new("Makepkg hardening", "Compiler flags for security",
                vec!["sudo sed -i 's/^BUILDENV=.*/BUILDENV=(!distcc color !ccache check !sign)/' /etc/makepkg.conf", "sudo sed -i 's/^#DEBUG_CFLAGS.*/DEBUG_CFLAGS=\"-g -fvar-tracking-assignments\"/' /etc/makepkg.conf", "sudo sed -i 's/^#DEBUG_CXXFLAGS.*/DEBUG_CXXFLAGS=\"-g -fvar-tracking-assignments\"/' /etc/makepkg.conf"]),
            Task::new("Pacman hardening", "Signature verification",
                vec!["sudo sed -i 's/^#SigLevel.*/SigLevel = Required DatabaseOptional/' /etc/pacman.conf", "sudo sed -i 's/^#LocalFileSigLevel.*/LocalFileSigLevel = Required/' /etc/pacman.conf"]),
            Task::new("Bootloader", "GRUB password + secure boot",
                vec!["echo 'Set GRUB password manually: grub-mkpasswd-pbkdf2'"]),
            Task::new("Mandatory access control", "AppArmor profiles",
                vec!["sudo pacman -S --needed --noconfirm apparmor apparmor-profiles", "sudo systemctl enable --now apparmor", "sudo aa-enforce /etc/apparmor.d/*"]),
        ],
    }
}

fn ai_ml() -> Category {
    Category {
        name: "AI / ML".into(),
        icon: "🧠".into(),
        tasks: vec![
            Task::new("Ollama", "Local LLM server",
                vec!["paru -S --needed --noconfirm ollama || true", "sudo systemctl enable --now ollama || true"]),
            Task::new("Ollama models", "Pull popular models",
                vec!["ollama pull llama3.2 || true", "ollama pull codellama || true", "ollama pull mistral || true", "ollama pull phi3 || true"]),
            Task::new("llama.cpp", "GGUF model runner",
                vec!["paru -S --needed --noconfirm llama.cpp-git || true"]),
            Task::new("Python ML", "PyTorch, TensorFlow, transformers",
                vec!["pipx install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121 || true", "pipx install tensorflow transformers accelerate || true"]),
            Task::new("AI tools", "Continue, Aider, Ollama CLI",
                vec!["pipx install continue-cli aider-chat || true"]),
        ],
    }
}

fn dotfiles_extended() -> Category {
    Category {
        name: "Dotfiles Extended".into(),
        icon: "💫".into(),
        tasks: vec![
            Task::new("Install dotfiles", "Clone & deploy 0xClumzzy's dotfiles repo",
                vec!["test -d \"$HOME/.dotfiles\" || git clone https://github.com/0xClumzzy/Dotfiles.git ~/.dotfiles", "cd ~/.dotfiles && ./install.sh || true"]),
            Task::new("Neovim config", "LazyVim starter config",
                vec!["git clone https://github.com/LazyVim/starter ~/.config/nvim || true", "rm -rf ~/.config/nvim/.git"]),
            Task::new("Tmux config", "Catppuccin tmux config",
                vec!["git clone https://github.com/catppuccin/tmux ~/.config/tmux/plugins/catppuccin || true"]),
            Task::new("SSH config", "Hardened SSH client config",
                vec!["mkdir -p ~/.ssh", "chmod 700 ~/.ssh"]),
            Task::new("Git config extended", "Aliases, signing, GPG",
                vec!["git config --global alias.lg \"log --oneline --graph --decorate --all\"", "git config --global alias.st status", "git config --global alias.co checkout", "git config --global alias.br branch", "git config --global alias.ci commit", "git config --global commit.gpgsign true", "git config --global gpg.format ssh"]),
            Task::new("Shell aliases", "Extra aliases file",
                vec!["touch ~/.shell_aliases", "grep -q 'shell_aliases' ~/.zshrc || echo '[ -f ~/.shell_aliases ] && source ~/.shell_aliases' >> ~/.zshrc"]),
            Task::new("Ghostty config", "GPU terminal config with Catppuccin theme",
                vec!["mkdir -p ~/.config/ghostty", "cat > ~/.config/ghostty/config << 'EOF'", "# Ghostty Config - Catppuccin Mocha", "font-family = \"JetBrainsMono Nerd Font\"", "font-size = 12", "theme = catppuccin-mocha", "background-opacity = 0.95", "window-decorations = \"client\"", "window-padding-x = 12", "window-padding-y = 12", "cursor-style = \"underline\"", "cursor-style-blink = true", "shell-integration = \"detect\"", "mouse-hide-while-typing = true", "quit-after-last-window-closed = true", "EOF"]),
        ],
    }
}

pub fn all_categories() -> Vec<Category> {
    vec![
        base_packages(),
        aur_helper(),
        blackarch(),
        devtools(),
        devtools_extended(),
        shell_config(),
        dotfiles_extended(),
        desktop_config(),
        services_config(),
        wallpapers_config(),
        pipx_apps(),
        kde_install(),
        gaming(),
        containers_cloud(),
        security_privacy(),
        media_creative(),
        system_hardening(),
        ai_ml(),
        updates_upgrades(),
    ]
}
