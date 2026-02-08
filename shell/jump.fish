# Fish shell integration for jump
# Add to ~/.config/fish/config.fish:
#   source /path/to/jump.fish

function jump --description "Quick directory navigation"
    set -l target (command jump $argv)
    if test -n "$target" -a -d "$target"
        cd $target
    end
end

# Ctrl+F keybinding
function _jump_fish
    set -l target (command jump)
    if test -n "$target" -a -d "$target"
        cd $target
    end
    commandline -f repaint
end

bind \cf _jump_fish
