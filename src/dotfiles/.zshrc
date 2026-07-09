# ~/.zshrc — powered by oh-my-zsh + starship

export ZSH="$HOME/.oh-my-zsh"

ZSH_THEME=""

plugins=(
  git
  zsh-autosuggestions
  zsh-syntax-highlighting
)

source "$ZSH/oh-my-zsh.sh"

# --- PATH ---
path_add_first() {
  for p in "$@"; do
    case ":$PATH:" in
      *":$p:"*) ;;
      *) PATH="$p:$PATH" ;;
    esac
  done
}

path_add_first \
  "$HOME/.local/bin" \
  "/home/linuxbrew/.linuxbrew/bin" \
  "/home/linuxbrew/.linuxbrew/sbin" \
  "$HOME/.linuxbrew/bin" \
  "$HOME/.linuxbrew/sbin" \
  "$HOME/.local/share/pnpm" \
  "$HOME/.npm-global/bin" \
  "$HOME/.yarn/bin" \
  "$HOME/.yarn/global/node_modules/.bin" \
  "$HOME/go/bin" \
  "$HOME/.deno/bin" \
  "$HOME/.cargo/bin" \
  "/var/lib/flatpak/exports/bin" \
  "$HOME/.local/share/flatpak/exports/bin" \
  "$HOME/.opencode/bin" \
  "$HOME/scripts/bashs"

# --- Starship prompt ---
eval "$(starship init zsh)"

# --- Zoxide ---
if command -v zoxide &>/dev/null; then
  eval "$(zoxide init zsh)"
fi

# --- thefuck ---
if command -v thefuck &>/dev/null; then
  eval "$(thefuck --alias)"
fi

# --- History ---
HISTSIZE=50000
SAVEHIST=50000
setopt HIST_IGNORE_ALL_DUPS
setopt HIST_FIND_NO_DUPS
setopt SHARE_HISTORY
setopt EXTENDED_HISTORY

# --- Options ---
setopt AUTO_CD
setopt EXTENDED_GLOB
setopt COMPLETE_IN_WORD
setopt NO_BEEP
setopt AUTO_LIST
setopt AUTO_MENU
setopt ALWAYS_TO_END

# --- Colors ---
export CLICOLOR=1
export LS_COLORS='no=00:fi=00:di=00;34:ln=01;36:pi=40;33:so=01;35:do=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:ex=01;32:*.tar=01;31:*.tgz=01;31:*.arj=01;31:*.taz=01;31:*.lzh=01;31:*.zip=01;31:*.z=01;31:*.Z=01;31:*.gz=01;31:*.bz2=01;31:*.deb=01;31:*.rpm=01;31:*.jar=01;31:*.jpg=01;35:*.jpeg=01;35:*.gif=01;35:*.bmp=01;35:*.pbm=01;35:*.pgm=01;35:*.ppm=01;35:*.tga=01;35:*.xbm=01;35:*.xpm=01;35:*.tif=01;35:*.tiff=01;35:*.png=01;35:*.mov=01;35:*.mpg=01;35:*.mpeg=01;35:*.avi=01;35:*.fli=01;35:*.gl=01;35:*.dl=01;35:*.xcf=01;35:*.xwd=01;35:*.ogg=01;35:*.mp3=01;35:*.wav=01;35:*.xml=00;31:'
export GREP_COLORS='mt=01;31'
export GREP_OPTIONS='--color=auto'

# --- Color for manpages ---
export LESS_TERMCAP_mb=$'\E[01;31m'
export LESS_TERMCAP_md=$'\E[01;31m'
export LESS_TERMCAP_me=$'\E[0m'
export LESS_TERMCAP_se=$'\E[0m'
export LESS_TERMCAP_so=$'\E[01;44;33m'
export LESS_TERMCAP_ue=$'\E[0m'
export LESS_TERMCAP_us=$'\E[01;32m'

# --- Editor ---
if command -v nvim &>/dev/null; then
  export EDITOR="nvim"
  export VISUAL="nvim"
elif command -v micro &>/dev/null; then
  export EDITOR="micro"
  export VISUAL="micro"
fi

# =============================================================================
# ALIASES (ported from ~/.bashrc)
# =============================================================================

# --- Editor aliases ---
command -v nvim &>/dev/null && alias vim='nvim' && alias vi='nvim' && alias vis='nvim "+set si"'
command -v pico &>/dev/null && alias spico='sudo pico'
command -v nano &>/dev/null && alias snano='sudo nano'

# --- Long-running command alert ---
alias alert='notify-send --urgency=low -i "$([ $? = 0 ] && echo terminal || echo error)" "$(history|tail -n1|sed -e '\''s/^\s*[0-9]\+\s*//;s/[;&|]\s*alert$//'\'')"'

# --- Date ---
alias da='date "+%Y-%m-%d %A %T %Z"'

# --- Modified commands ---
alias cp='cp -i'
alias mv='mv -i'
alias mkdir='mkdir -p'
alias ps='ps auxf'
alias ping='ping -c 10'
alias less='less -R'
alias cls='clear'

# --- rm: prefer trash ---
if command -v trash-put &>/dev/null; then
  alias rm='trash-put'
elif command -v trash &>/dev/null; then
  alias rm='trash -v'
elif command -v gio &>/dev/null; then
  alias rm='gio trash'
else
  alias rm='rm -i'
fi

command -v multitail &>/dev/null && alias multitail='multitail --no-repeat -c'
command -v freshclam &>/dev/null && alias freshclam='sudo freshclam'
command -v kitty &>/dev/null && alias kssh='kitty +kitten ssh'

# --- AUR picker ---
if command -v yay &>/dev/null && command -v fzf &>/dev/null; then
  alias yayf="yay -Slq | fzf --multi --preview 'yay -Sii {1}' --preview-window=down:75% | xargs -ro yay -S"
fi

# --- Change directory aliases ---
alias web='cd /var/www/html'
alias home='cd ~'
alias cd..='cd ..'
alias ..='cd ..'
alias ...='cd ../..'
alias ....='cd ../../..'
alias .....='cd ../../../..'
alias bd='cd "$OLDPWD"'
alias rmd='/bin/rm --recursive --force --verbose --one-file-system --'

# --- Directory listing aliases ---
alias la='ls -Alh'
alias ls='ls --color=auto'
alias lx='ls -lXBh'
alias lk='ls -lSrh'
alias lc='ls -ltcrh'
alias lu='ls -lturh'
alias lr='ls -lRh'
alias lt='ls -ltrh'
alias lm='ls -alh | more'
alias lw='ls -xAh'
alias ll='ls -Fls'
alias labc='ls -lap'
alias lf="ls -l | grep -Ev '^d'"
alias ldir="ls -l | grep -E '^d'"
alias lla='ls -Al'
alias las='ls -A'
alias lls='ls -l'

# --- chmod shorthands ---
alias mx='chmod a+x'
alias 000='chmod -R 000'
alias 644='chmod -R 644'
alias 666='chmod -R 666'
alias 755='chmod -R 755'
alias 777='chmod -R 777'

# --- Search helpers ---
alias h="history | grep "
alias p="ps aux | grep "
alias topcpu="/bin/ps -eo pcpu,pid,user,args | sort -k 1 -r | head -10"
alias f="find . | grep "
alias checkcommand="type -t"

# --- Network ---
if command -v ss &>/dev/null; then
  alias openports='ss -tulpen'
elif command -v netstat &>/dev/null; then
  alias openports='netstat -nape --inet'
fi

# --- Reboot ---
alias rebootsafe='sudo shutdown -r now'
alias rebootforce='sudo shutdown -r -n now'

# --- Disk space ---
alias diskspace="du -S | sort -n -r | more"
alias folders='du -h --max-depth=1'
alias folderssort='find . -maxdepth 1 -type d -print0 | xargs -0 du -sk | sort -rn'
alias mountedinfo='df -hT'

# --- tree ---
command -v tree &>/dev/null && alias tree='tree -CAhF --dirsfirst'
command -v tree &>/dev/null && alias treed='tree -CAFd'

# --- Archives ---
alias mktar='tar -cvf'
alias mkbz2='tar -cvjf'
alias mkgz='tar -cvzf'
alias untar='tar -xvf'
alias unbz2='tar -xvjf'
alias ungz='tar -xvzf'

# --- Logs ---
alias logs="sudo find /var/log -type f -exec file {} \; | grep 'text' | cut -d' ' -f1 | sed -e's/:$//g' | grep -v '[0-9]$' | xargs tail -f"

# --- Misc ---
alias sha1='openssl sha1'
alias whatismyip="whatsmyip"

# --- User services ---
alias hug="systemctl --user restart hugo"
alias lanm="systemctl --user restart lan-mouse"

# --- Basics ---
alias grep='grep --color=auto'
alias c='clear'
alias x='exit'
alias r='reboot'
alias fucking='sudo'

# --- Edit config ---
alias e='micro ~/.zshrc'
alias zshrc='micro ~/.zshrc'
alias src='source ~/.zshrc'
alias hosts='micro /etc/hosts'

# --- Personal scripts ---
alias ass='bash $HOME/scripts/bashs/asm2.sh'
alias ms='bash $HOME/scripts/bashs/combo.sh'
alias vpn='bash $HOME/scripts/bashs/vpnc.sh'
alias rs='bash $HOME/scripts/bashs/rustscan.sh'
alias gs='bash $HOME/scripts/bashs/gobuster.sh'

# --- systemctl shortcuts ---
alias status='sudo systemctl status'
alias enable='sudo systemctl enable --now'
alias start='sudo systemctl start'
alias stop='sudo systemctl stop'
alias restart='sudo systemctl restart'
alias disable='sudo systemctl disable'
alias network='sudo systemctl start --now NetworkManager'

# --- Pacman / AUR ---
alias fuckup='sudo pacman -Rns --noconfirm'
alias lookfor='sudo pacman -Ss'
alias need='sudo pacman -Sy --noconfirm'
alias get='sudo pacman -Sy --noconfirm'
alias update='sudo pacman -Syu --noconfirm'
alias ysearch='yay -Ss'
alias yget='yay -S'
alias psearch='paru -Ss --noconfirm'
alias pget='paru -S --noconfirm'

# --- Python venv ---
alias build='python -m venv dookie'
alias dookie='python3 -m venv dookie && source dookie/bin/activate'
alias activate='source dookie/bin/activate'
alias delete='rm -rf dookie'

# --- SSH / CTF ---
alias pwnc='cd .k3ys && ssh -i key hacker@dojo.pwn.college'
alias hmv='bash $HOME/scripts/bashs/hmv.sh'
alias hmvc='bash $HOME/scripts/bashs/hmvc.sh'

# =============================================================================
# FUNCTIONS (ported from ~/.bashrc)
# =============================================================================

# --- Extract archives ---
extract() {
  for archive in "$@"; do
    if [ -f "$archive" ]; then
      case $archive in
        *.tar.bz2) tar xvjf "$archive" ;;
        *.tar.gz)  tar xvzf "$archive" ;;
        *.bz2)     bunzip2 "$archive" ;;
        *.rar)     rar x "$archive" ;;
        *.gz)      gunzip "$archive" ;;
        *.tar)     tar xvf "$archive" ;;
        *.tbz2)    tar xvjf "$archive" ;;
        *.tgz)     tar xvzf "$archive" ;;
        *.zip)     unzip "$archive" ;;
        *.Z)       uncompress "$archive" ;;
        *.7z)      7z x "$archive" ;;
        *)         echo "don't know how to extract '$archive'..." ;;
      esac
    else
      echo "'$archive' is not a valid file!"
    fi
  done
}

# --- Search text in files ---
ftext() {
  grep -iIHrn --color=always "$1" . | less -r
}

# --- Copy with progress bar ---
cpp() {
  if [ "$#" -ne 2 ]; then
    echo "Usage: cpp SOURCE DESTINATION"
    return 2
  fi
  if ! command -v strace &>/dev/null; then
    echo "cpp requires strace."
    return 1
  fi
  strace -q -ewrite cp -- "$1" "$2" 2>&1 |
  awk '{
    count += $NF
    if (count % 10 == 0 && total_size > 0) {
      percent = int(count / total_size * 100)
      if (percent > 100) percent = 100
      printf "%3d%% [", percent
      for (i=0;i<=percent;i++) printf "="
      printf ">"
      for (i=percent;i<100;i++) printf " "
      printf "]\r"
    }
  }
  END { print "" }' total_size="$(stat -c '%s' "$1")" count=0
}

# --- Copy and go ---
cpg() {
  if [ -d "$2" ]; then
    cp "$1" "$2" && cd "$2"
  else
    cp "$1" "$2"
  fi
}

# --- Move and go ---
mvg() {
  if [ -d "$2" ]; then
    mv "$1" "$2" && cd "$2"
  else
    mv "$1" "$2"
  fi
}

# --- mkdir + cd ---
mkcd() {
  mkdir -p "$1" && cd "$1"
}
mkdirg() { mkcd "$@"; }

# --- Count files ---
countfiles() {
  local t
  for t in files links directories; do
    echo "$(find . -type "${t:0:1}" | wc -l)" "$t"
  done 2>/dev/null
}

# --- Go up N directories ---
up() {
  local d="" limit i
  limit=${1:-1}
  if ! [[ $limit =~ ^[0-9]+$ ]]; then
    echo "Usage: up [number]"
    return 2
  fi
  for ((i = 1; i <= limit; i++)); do
    d=$d/..
  done
  d=${d#/}
  cd "${d:-..}" || return
}

# --- Distribution ---
distribution() {
  local dtype="unknown"
  if [ -r /etc/os-release ]; then
    source /etc/os-release
    case $ID in
      fedora|rhel|centos)           dtype="redhat" ;;
      sles|opensuse*)               dtype="suse" ;;
      ubuntu|debian)                dtype="debian" ;;
      gentoo)                       dtype="gentoo" ;;
      arch|manjaro)                 dtype="arch" ;;
      slackware)                    dtype="slackware" ;;
      *)
        if [ -n "$ID_LIKE" ]; then
          case $ID_LIKE in
            *fedora*|*rhel*|*centos*) dtype="redhat" ;;
            *sles*|*opensuse*)        dtype="suse" ;;
            *ubuntu*|*debian*)        dtype="debian" ;;
            *gentoo*)                 dtype="gentoo" ;;
            *arch*)                   dtype="arch" ;;
            *slackware*)              dtype="slackware" ;;
          esac
        fi ;;
    esac
  fi
  echo "$dtype"
}

# --- OS version ---
ver() {
  local dtype
  dtype=$(distribution)
  case $dtype in
    redhat)   [ -s /etc/redhat-release ] && cat /etc/redhat-release || cat /etc/issue; uname -a ;;
    suse)     cat /etc/SuSE-release ;;
    debian)   lsb_release -a ;;
    gentoo)   cat /etc/gentoo-release ;;
    arch)     cat /etc/os-release ;;
    slackware) cat /etc/slackware-version ;;
    *)
      if [ -s /etc/issue ]; then cat /etc/issue
      else echo "Error: Unknown distribution"; exit 1
      fi ;;
  esac
}

# --- IP lookup ---
whatsmyip() {
  echo -n "Internal IP: "
  if command -v ip &>/dev/null; then
    ip -o -4 route get 1.1.1.1 2>/dev/null | awk '{for (i=1; i<=NF; i++) if ($i == "src") {print $(i+1); exit}}'
  elif command -v ifconfig &>/dev/null; then
    ifconfig | awk '/inet / && $2 != "127.0.0.1" {print $2; exit}'
  else
    echo "unknown"
  fi
  echo -n "External IP: "
  if command -v curl &>/dev/null; then
    curl -4fsS --max-time 5 https://ifconfig.me || echo "unknown"
  else
    echo "curl not installed"
  fi
}

# --- sudo edit ---
sedit() {
  sudo "${SUDO_EDITOR:-${EDITOR:-vi}}" "$@"
}

# --- Clickpaste ---
clickpaste() {
  sleep "${1:-3}"
  if command -v wl-paste &>/dev/null && command -v wtype &>/dev/null; then
    wl-paste | wtype -
  elif command -v xclip &>/dev/null && command -v xdotool &>/dev/null; then
    xdotool type "$(xclip -o -selection clipboard)"
  else
    echo "clickpaste requires wl-paste+wtype on Wayland or xclip+xdotool on X11."
    return 1
  fi
}

# --- Docker clean ---
docker-clean() {
  if ! command -v docker &>/dev/null; then
    echo "docker is not installed."
    return 1
  fi
  docker container prune -f
  docker image prune -f
  docker network prune -f
  docker volume prune -f
}

# --- Git helpers ---
gcom() {
  if [ -z "$1" ]; then
    echo "Usage: gcom \"commit message\""
    return 2
  fi
  git add .
  git commit -m "$1"
}
lazyg() {
  if [ -z "$1" ]; then
    echo "Usage: lazyg \"commit message\""
    return 2
  fi
  git add .
  git commit -m "$1"
  git push
}

# --- Clone and cd ---
clone() {
  git clone "$1" || return 1
  cd "$(basename "$1" .git)" || return 1
}

# --- HTB dir ---
htbdir() {
  cd "$HOME/ctfs/htb" && mkdir -p "$1" && cd "$1"
}

# --- Create file ---
make() {
  read "fileName?filename: "
  touch "$fileName" && chmod 700 "$fileName"
  ${EDITOR:-micro} "$fileName"
}

# --- Create temp file ---
tmp() {
  read "fileName?filename: "
  [[ -z "$fileName" ]] && echo "bro enter a filename" && return 1
  touch "/tmp/$fileName" && chmod 700 "/tmp/$fileName"
  ${EDITOR:-micro} "/tmp/$fileName"
  read "catit?wanna cat it? (y/n): "
  [[ "$catit" == "y" ]] && cat "/tmp/$fileName"
}

# --- pwd tail ---
pwdtail() {
  pwd | awk -F/ '{nlast = NF -1; print $nlast"/"$NF}'
}

# --- Trim whitespace ---
trim() {
  local var=$*
  var="${var#"${var%%[![:space:]]*}"}"
  var="${var%"${var##*[![:space:]]}"}"
  echo -n "$var"
}

# --- Install support tools ---
install_bashrc_support() {
  local dtype
  dtype=$(distribution)
  case $dtype in
    redhat)
      if command -v dnf &>/dev/null; then
        sudo dnf install multitail tree zoxide trash-cli fzf bash-completion fastfetch
      else
        sudo yum install multitail tree zoxide trash-cli fzf bash-completion fastfetch
      fi ;;
    suse)    sudo zypper install multitail tree zoxide trash-cli fzf bash-completion fastfetch ;;
    debian)  sudo apt-get update && sudo apt-get install multitail tree zoxide trash-cli fzf bash-completion fastfetch ;;
    arch)    sudo pacman -S --needed multitail tree zoxide trash-cli fzf bash-completion fastfetch ;;
    slackware) echo "No install support for Slackware" ;;
    *)       echo "Unknown distribution" ;;
  esac
}

# --- Apache log viewer ---
apachelog() {
  if [ -f /etc/httpd/conf/httpd.conf ]; then
    cd /var/log/httpd && ls -xAh || return
    if command -v multitail &>/dev/null; then
      multitail --no-repeat -c -s 2 /var/log/httpd/*_log
    else
      tail -f /var/log/httpd/*_log
    fi
  elif [ -d /var/log/apache2 ]; then
    cd /var/log/apache2 && ls -xAh || return
    if command -v multitail &>/dev/null; then
      multitail --no-repeat -c -s 2 /var/log/apache2/*.log
    else
      tail -f /var/log/apache2/*.log
    fi
  else
    echo "Error: Apache log directory could not be found."
  fi
}

find_config_file() {
  local name=$1
  if command -v locate &>/dev/null; then
    sudo updatedb && locate "$name"
  else
    echo "locate is not installed."
  fi
}
apacheconfig() {
  if [ -f /etc/httpd/conf/httpd.conf ]; then
    sedit /etc/httpd/conf/httpd.conf
  elif [ -f /etc/apache2/apache2.conf ]; then
    sedit /etc/apache2/apache2.conf
  else
    echo "Error: Apache config file could not be found."
    find_config_file httpd.conf
    find_config_file apache2.conf
  fi
}
phpconfig() {
  if [ -f /etc/php.ini ]; then
    sedit /etc/php.ini
  elif [ -f /etc/php/php.ini ]; then
    sedit /etc/php/php.ini
  elif [ -f /etc/php5/php.ini ]; then
    sedit /etc/php5/php.ini
  elif [ -f /usr/bin/php5/bin/php.ini ]; then
    sedit /usr/bin/php5/bin/php.ini
  elif [ -f /etc/php5/apache2/php.ini ]; then
    sedit /etc/php5/apache2/php.ini
  else
    echo "Error: php.ini file could not be found."
    find_config_file php.ini
  fi
}
mysqlconfig() {
  if [ -f /etc/my.cnf ]; then
    sedit /etc/my.cnf
  elif [ -f /etc/mysql/my.cnf ]; then
    sedit /etc/mysql/my.cnf
  elif [ -f /usr/local/etc/my.cnf ]; then
    sedit /usr/local/etc/my.cnf
  elif [ -f /usr/bin/mysql/my.cnf ]; then
    sedit /usr/bin/mysql/my.cnf
  elif [ -f "$HOME/my.cnf" ]; then
    sedit "$HOME/my.cnf"
  elif [ -f "$HOME/.my.cnf" ]; then
    sedit "$HOME/.my.cnf"
  else
    echo "Error: my.cnf file could not be found."
    find_config_file my.cnf
  fi
}

# --- Paru install ---
paruinstall() {
  set -euo pipefail
  git clone https://aur.archlinux.org/paru.git
  cd paru
  makepkg -si
}

# --- Tool installers ---
wafinstall() {
  python3 -m venv ~/.venvs/wafw00f
  ~/.venvs/wafw00f/bin/pip install wafw00f
}
waf() { dookie && wafw00f "$@"; }

cminstall() {
  dookie && git clone https://github.com/Tuhinshubhra/CMSeeK &&
  cd CMSeeK && pip install -r requirements.txt
}
cmseek() { dookie && cd ~/CMSeeK && python3 cmseek.py; }

freconinstall() {
  dookie && git clone https://github.com/thewhiteh4t/FinalRecon.git &&
  cd FinalRecon && pip install -r requirements.txt &&
  chmod +x finalrecon.py
}
frecon() { dookie && cd ~/FinalRecon && ./finalrecon.py "$@"; }

strikeinstall() {
  python3 -m venv venv && source venv/bin/activate &&
  git clone https://github.com/s0md3v/XSStrike.git &&
  cd XSStrike && pip install -r requirements.txt
}
strike() { dookie && cd ~/XSStrike && python3 xsstrike.py -u "$@"; }

arjuninstall() { pipx install arjun; }

fuzzing() {
  set -euo pipefail
  sudo pacman -Syu --needed --noconfirm go git base-devel
  command -v paru || {
    git clone https://aur.archlinux.org/paru.git
    cd paru && makepkg -si --noconfirm && cd ..
  }
  paru -S --needed --noconfirm python-pipx
  pipx ensurepath
  sudo pipx ensurepath --global
  go install github.com/ffuf/ffuf/v2@latest
  go install github.com/OJ/gobuster/v3@latest
  curl -sL https://raw.githubusercontent.com/epi052/feroxbuster/main/install-nix.sh | bash -s "$HOME/.local/bin"
  pipx install git+https://github.com/WebFuzzForge/wenum
  pipx runpip wenum install setuptools
}

wapifinstall() {
  dookie && git clone https://github.com/PandaSt0rm/webfuzz_api.git &&
  cd webfuzz_api && pip install -r requirements.txt
}
wapif() { dookie && cd ~/webfuzz_api && python3 api_fuzzer.py "$@"; }

addGef() {
  wget -O ~/.gdbinit-gef.py -q https://gef.blah.cat/py
  echo source ~/.gdbinit-gef.py >> ~/.gdbinit
}

# --- Auto ls after cd ---
cd() {
  builtin cd "$@" && ls
}

# --- fastfetch on terminal open ---
if command -v fastfetch &>/dev/null; then
  fastfetch
fi

# +---------------------------------------------------------------+
# |                _ __ ___  ___| |__   ___| | |                  |
# |               | '__/ _ \/ __| '_ \ / _ \ | |                  |
# |               | | |  __/\__ \ | | |  __/ | |                  |
# |               |_|  \___||___/_| |_|\___|_|_|                  |
# |                                                               |
# |       WARNING: THIS BLOCK IS AUTOMATICALLY MANAGED BY RESHELL. |
# |       DO NOT MANUALLY UPDATE OR EDIT THE CODE WITHIN IT.      |
# +---------------------------------------------------------------+
# >>> reshell initialize >>>
if [ -f "/home/clumzzy/.config/reshell/shell/reshell.sh" ]; then
    . "/home/clumzzy/.config/reshell/shell/reshell.sh"
fi
# <<< reshell initialize <<<
export PATH="$HOME/scripts/bashs:$PATH"
