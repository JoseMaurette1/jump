#!/bin/bash
set -e

REPO="JoseMaurette1/jump"
INSTALL_DIR="${HOME}/.local/bin"
SHELL_SCRIPT_DIR="${HOME}/.local/share/jump"

# Check for required tools
if ! command -v curl &> /dev/null; then
    echo "Error: curl is required but not installed."
    echo "Install curl first:"
    echo "  - macOS: brew install curl"
    echo "  - Linux: sudo apt install curl"
    exit 1
fi

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="darwin" ;;
        *)       echo "Unsupported OS: $(uname -s)"; exit 1 ;;
    esac

    case "$(uname -m)" in
        arm64|aarch64) arch="aarch64" ;;
        x86_64|amd64)
            if [[ "${os}" == "darwin" ]]; then
                echo "macOS Intel is not supported. Please use an Apple Silicon Mac."
                exit 1
            fi
            arch="x86_64"
            ;;
        *)             echo "Unsupported architecture: $(uname -m)"; exit 1 ;;
    esac

    echo "${os}-${arch}"
}

get_latest_version() {
    curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

download_and_install() {
    local platform="$1"
    local version="$2"
    local url="https://github.com/${REPO}/releases/download/${version}/jump-${platform}.tar.gz"
    local tmp_dir

    tmp_dir=$(mktemp -d)
    trap "rm -rf ${tmp_dir}" EXIT

    echo "Downloading jump ${version} for ${platform}..."

    # Check if release exists first
    if ! curl -sSL -o /dev/null -w "%{http_code}" "${url}" | grep -q "200"; then
        echo "Error: Release not found for ${platform}"
        echo "URL: ${url}"
        echo "The release may still be building. Wait a few minutes and try again."
        exit 1
    fi

    curl -sSL "${url}" | tar xz -C "${tmp_dir}"

    if [[ ! -f "${tmp_dir}/jump" ]]; then
        echo "Error: Failed to extract jump binary"
        exit 1
    fi

    mkdir -p "${INSTALL_DIR}"
    mv "${tmp_dir}/jump" "${INSTALL_DIR}/jump"
    chmod +x "${INSTALL_DIR}/jump"

    echo "Installed jump to ${INSTALL_DIR}/jump"
}

install_shell_integration() {
    mkdir -p "${SHELL_SCRIPT_DIR}"

    cat > "${SHELL_SCRIPT_DIR}/jump.sh" << 'SHELL_EOF'
jump() {
    local target
    target="$(command jump "$@")"
    if [[ -n "$target" && -d "$target" ]]; then
        cd "$target" || return 1
    fi
}
j() { jump "$@"; }
SHELL_EOF

    echo "Installed shell integration to ${SHELL_SCRIPT_DIR}/jump.sh"
}

add_to_shell_rc() {
    local rc_file=""
    local source_line="source \"${SHELL_SCRIPT_DIR}/jump.sh\""
    local path_line="export PATH=\"\${HOME}/.local/bin:\${PATH}\""

    if [[ -n "${ZSH_VERSION}" ]] || [[ "${SHELL}" == *"zsh"* ]]; then
        rc_file="${HOME}/.zshrc"
    elif [[ -n "${BASH_VERSION}" ]] || [[ "${SHELL}" == *"bash"* ]]; then
        rc_file="${HOME}/.bashrc"
    fi

    if [[ -n "${rc_file}" ]]; then
        if ! grep -q "jump.sh" "${rc_file}" 2>/dev/null; then
            echo "" >> "${rc_file}"
            echo "# jump - directory navigation" >> "${rc_file}"
            echo "${path_line}" >> "${rc_file}"
            echo "${source_line}" >> "${rc_file}"
            echo "Added jump to ${rc_file}"
        else
            echo "jump already in ${rc_file}"
        fi
    fi
}

main() {
    echo "Installing jump..."
    echo ""

    local platform version
    platform=$(detect_platform)
    version=$(get_latest_version)

    if [[ -z "${version}" ]]; then
        echo "Could not determine latest version. Check your internet connection."
        exit 1
    fi

    download_and_install "${platform}" "${version}"
    install_shell_integration
    add_to_shell_rc

    echo ""
    echo "Done! Restart your shell or run:"
    echo "  source ~/.bashrc  # or ~/.zshrc"
    echo ""
    echo "Then use 'jump' or 'j' to navigate directories."
}

main
