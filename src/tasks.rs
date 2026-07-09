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
    "[user]\n\tname = 0xClumzZy\n\temail = sustee848@gmail.com\n"
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
        icon: ">".into(),
        tasks: vec![
            Task::new("Core tools", "git, curl, wget, bat, eza, fzf, ripgrep, btop, fastfetch",
                vec!["sudo pacman -S --needed --noconfirm base-devel git curl wget unzip 7zip htop btop tree ripgrep fd fzf bat eza tmux fastfetch zoxide trash-cli man-db man-pages less"]),
            Task::new("Shell", "zsh, starship, autosuggestions, syntax-highlighting, thefuck",
                vec!["sudo pacman -S --needed --noconfirm zsh zsh-autosuggestions zsh-syntax-highlighting starship thefuck"]),
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
        icon: ">".into(),
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
        icon: ">".into(),
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
        icon: ">".into(),
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
        icon: ">".into(),
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
        icon: ">".into(),
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
        icon: ">".into(),
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

fn wallpapers_config() -> Category {
    Category {
        name: "Wallpapers".into(),
        icon: ">".into(),
        tasks: vec![
            Task::new("Wallpapers", "Deploy 34 dark aesthetic wallpapers to ~/Pictures/wallpapers", vec![])
                .with_wallpapers(),
        ],
    }
}

pub fn all_categories() -> Vec<Category> {
    vec![
        base_packages(),
        aur_helper(),
        blackarch(),
        devtools(),
        shell_config(),
        desktop_config(),
        services_config(),
        wallpapers_config(),
    ]
}
