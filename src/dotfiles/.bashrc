#!/usr/bin/env bash
# ~/.bashrc - Modular configuration
# Source order: exports -> path -> aliases -> prompt -> local

# If not running interactively, don't do anything
[[ $- != *i* ]] && return

# ---- Source core modules ----
DOTFILES_DIR="${DOTFILES_DIR:-$HOME/.config}"

# Load shared exports
[ -f "$DOTFILES_DIR/bash/exports.sh" ] && source "$DOTFILES_DIR/bash/exports.sh"

# Load PATH management
[ -f "$DOTFILES_DIR/bash/path.sh" ] && source "$DOTFILES_DIR/bash/path.sh"

# Load shared aliases
[ -f "$DOTFILES_DIR/bash/aliases.sh" ] && source "$DOTFILES_DIR/bash/aliases.sh"

# ---- Bash-specific settings ----

# Better history sync
_bashrc_history_sync() {
    history -a
    history -n
}
case ";$PROMPT_COMMAND;" in
    *";_bashrc_history_sync;"*) ;;
    *) PROMPT_COMMAND="${PROMPT_COMMAND:+$PROMPT_COMMAND;}_bashrc_history_sync" ;;
esac

# Enable bash completion
if [ -f /usr/share/bash-completion/bash_completion ]; then
    # shellcheck source=/dev/null
    . /usr/share/bash-completion/bash_completion
elif [ -f /etc/bash_completion ]; then
    # shellcheck source=/dev/null
    . /etc/bash_completion
fi

# ---- Starship prompt ----
if command -v starship >/dev/null 2>&1; then
    eval "$(starship init bash)"
fi

# ---- Zoxide ----
if command -v zoxide >/dev/null 2>&1; then
    eval "$(zoxide init bash)"
    # Bind Ctrl+f to insert 'zi' (zoxide interactive)
    bind '"\C-f":"zi\n"'
fi

# ---- FZF ----
if command -v fzf >/dev/null 2>&1; then
    # Source fzf keybindings and completion
    if [ -f /usr/share/fzf/key-bindings.bash ]; then
        source /usr/share/fzf/key-bindings.bash
    elif [ -f /usr/share/doc/fzf/examples/key-bindings.bash ]; then
        source /usr/share/doc/fzf/examples/key-bindings.bash
    fi
    if [ -f /usr/share/fzf/completion.bash ]; then
        source /usr/share/fzf/completion.bash
    elif [ -f /usr/share/doc/fzf/examples/completion.bash ]; then
        source /usr/share/doc/fzf/examples/completion.bash
    fi
fi

# ---- Thefuck ----
if command -v thefuck >/dev/null 2>&1; then
    eval "$(thefuck --alias)"
fi

# ---- Direnv ----
if command -v direnv >/dev/null 2>&1; then
    eval "$(direnv hook bash)"
fi

# ---- Atuin (history) ----
if command -v atuin >/dev/null 2>&1; then
    eval "$(atuin init bash)"
fi

# ---- Fastfetch on startup ----
if command -v fastfetch >/dev/null 2>&1; then
    fastfetch
fi

# ---- Local/private overrides (not tracked in git) ----
if [ -f "$HOME/.bashrc.local" ]; then
    source "$HOME/.bashrc.local"
fi