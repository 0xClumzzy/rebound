#!/usr/bin/env zsh
# ~/.zshrc - Modular configuration
# Source order: exports -> path -> aliases -> prompt -> local

# If not running interactively, don't do anything
[[ -o interactive ]] || return

# ---- Source core modules ----
DOTFILES_DIR="${DOTFILES_DIR:-$HOME/.config}"

# Load shared exports
[ -f "$DOTFILES_DIR/bash/exports.sh" ] && source "$DOTFILES_DIR/bash/exports.sh"

# Load PATH management
[ -f "$DOTFILES_DIR/bash/path.sh" ] && source "$DOTFILES_DIR/bash/path.sh"

# Load shared aliases
[ -f "$DOTFILES_DIR/bash/aliases.sh" ] && source "$DOTFILES_DIR/bash/aliases.sh"

# ---- Oh My Zsh ----
export ZSH="$HOME/.oh-my-zsh"
ZSH_THEME=""  # Using starship instead

# Plugins
plugins=(
    git
    zsh-syntax-highlighting
    zsh-autosuggestions
    fzf-tab
    # Add more: zsh-completions, you-should-use, etc.
)

# Load Oh My Zsh
if [ -f "$ZSH/oh-my-zsh.sh" ]; then
    source "$ZSH/oh-my-zsh.sh"
fi

# ---- Zsh-specific options ----
setopt AUTO_CD              # cd by typing directory name
setopt EXTENDED_GLOB        # Extended globbing
setopt COMPLETE_IN_WORD     # Complete from both ends
setopt NO_BEEP              # No bell
setopt AUTO_LIST            # Auto list choices
setopt AUTO_MENU            # Auto menu on second tab
setopt ALWAYS_TO_END        # Move cursor to end on completion
setopt HIST_IGNORE_ALL_DUPS # Remove duplicates
setopt HIST_FIND_NO_DUPS    # Don't show duplicates in search
setopt SHARE_HISTORY        # Share history across sessions
setopt EXTENDED_HISTORY     # Save timestamps
setopt HIST_EXPIRE_DUPS_FIRST
setopt HIST_IGNORE_SPACE    # Ignore commands starting with space
setopt HIST_VERIFY          # Show before executing history
setopt INC_APPEND_HISTORY   # Write immediately

# History settings
HISTSIZE=50000
SAVEHIST=50000
HISTFILE="$HOME/.config/zsh/.histfile"

# ---- Completion ----
autoload -Uz compinit
compinit -d "$HOME/.config/zsh/.zcompdump"
zstyle ':completion:*' menu select
zstyle ':completion:*' matcher-list 'm:{a-zA-Z}={A-Za-z}' 'r:|=*' 'l:|=* r:|=*'
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' use-cache on
zstyle ':completion:*' cache-path "$HOME/.config/zsh/.zcompcache"

# ---- Keybindings ----
bindkey -e  # Emacs mode
bindkey '^[[A' history-substring-search-up
bindkey '^[[B' history-substring-search-down
bindkey '^R' history-incremental-search-backward
bindkey '^S' history-incremental-search-forward

# ---- Starship prompt ----
if command -v starship >/dev/null 2>&1; then
    eval "$(starship init zsh)"
fi

# ---- Zoxide ----
if command -v zoxide >/dev/null 2>&1; then
    eval "$(zoxide init zsh)"
fi

# ---- FZF ----
if command -v fzf >/dev/null 2>&1; then
    if [ -f /usr/share/fzf/key-bindings.zsh ]; then
        source /usr/share/fzf/key-bindings.zsh
    elif [ -f /usr/share/doc/fzf/examples/key-bindings.zsh ]; then
        source /usr/share/doc/fzf/examples/key-bindings.zsh
    fi
    if [ -f /usr/share/fzf/completion.zsh ]; then
        source /usr/share/fzf/completion.zsh
    elif [ -f /usr/share/doc/fzf/examples/completion.zsh ]; then
        source /usr/share/doc/fzf/examples/completion.zsh
    fi
fi

# ---- Thefuck ----
if command -v thefuck >/dev/null 2>&1; then
    eval "$(thefuck --alias)"
fi

# ---- Direnv ----
if command -v direnv >/dev/null 2>&1; then
    eval "$(direnv hook zsh)"
fi

# ---- Atuin ----
if command -v atuin >/dev/null 2>&1; then
    eval "$(atuin init zsh)"
fi

# ---- Zsh-syntax-highlighting config ----
if command -v zsh-syntax-highlighting >/dev/null 2>&1 || [ -f /usr/share/zsh/plugins/zsh-syntax-highlighting/zsh-syntax-highlighting.zsh ]; then
    ZSH_HIGHLIGHT_HIGHLIGHTERS=(main brackets pattern regexp cursor)
    ZSH_HIGHLIGHT_PATTERNS+=('[0-9]##' 'fg=magenta,bold')
    ZSH_HIGHLIGHT_PATTERNS+=('([0-9]##\.){3}[0-9]##' 'fg=red,bold')
    ZSH_HIGHLIGHT_REGEXP+=('--?[[:alnum:]-]+' 'fg=cyan')
    ZSH_HIGHLIGHT_STYLES[path]='fg=blue,underline'
    ZSH_HIGHLIGHT_STYLES[path_prefix]='fg=blue'
fi

# ---- Fastfetch on startup ----
if command -v fastfetch >/dev/null 2>&1; then
    fastfetch
fi

# ---- Local/private overrides (not tracked in git) ----
if [ -f "$HOME/.zshrc.local" ]; then
    source "$HOME/.zshrc.local"
fi

# ---- ZSH autosuggestions config ----
if [ -f /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh ]; then
    source /usr/share/zsh/plugins/zsh-autosuggestions/zsh-autosuggestions.zsh
    ZSH_AUTOSUGGEST_STRATEGY=(history completion)
    ZSH_AUTOSUGGEST_BUFFER_MAX_SIZE=20
fi

# ---- FZF-tab config ----
if [ -f /usr/share/zsh/plugins/fzf-tab/fzf-tab.zsh ]; then
    source /usr/share/zsh/plugins/fzf-tab/fzf-tab.zsh
fi