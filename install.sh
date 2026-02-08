#!/usr/bin/env bash
set -e

REPO="JoseMaurette1/jump"
INSTALL_DIR="${HOME}/.local/bin"
SHELL_SCRIPT_DIR="${HOME}/.local/share/jump"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)  os="linux" ;;
        Darwin*) os="darwin" ;;
        *)       echo -e "${RED}Error: Unsupported OS: $(uname -s)${NC}" >&2; exit 1 ;;
    esac

    case "$(uname -m)" in
        arm64|aarch64) arch="aarch64" ;;
        x86_64|amd64)
            if [[ "${os}" == "darwin" ]]; then
                echo -e "${RED}Error: macOS Intel is not supported. Please use an Apple Silicon Mac.${NC}" >&2
                exit 1
            fi
            arch="x86_64"
            ;;
        *)             echo -e "${RED}Error: Unsupported architecture: $(uname -m)${NC}" >&2; exit 1 ;;
    esac

    echo "${os}-${arch}"
}

get_latest_version() {
    local version
    version=$(curl -sSL "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
    echo "${version}"
}

install_via_curl() {
    local platform="$1"
    local version="$2"
    local url="https://github.com/${REPO}/releases/download/${version}/jump-${platform}.tar.gz"
    local tmp_dir

    echo -e "${YELLOW}Installing via curl...${NC}"

    tmp_dir=$(mktemp -d)
    trap "rm -rf ${tmp_dir}" EXIT

    echo "Downloading jump ${version} for ${platform}..."

    # Check if release exists
    local http_code
    http_code=$(curl -sSL -o /dev/null -w "%{http_code}" "${url}")
    if [[ "${http_code}" != "200" ]]; then
        echo -e "${RED}Error: Release not found for ${platform}${NC}"
        echo "The release may still be building. Wait a few minutes and try again."
        exit 1
    fi

    curl -sSL "${url}" | tar xz -C "${tmp_dir}"

    if [[ ! -f "${tmp_dir}/jump" ]]; then
        echo -e "${RED}Error: Failed to extract jump binary${NC}"
        exit 1
    fi

    mkdir -p "${INSTALL_DIR}"
    mv "${tmp_dir}/jump" "${INSTALL_DIR}/jump"
    chmod +x "${INSTALL_DIR}/jump"

    echo -e "${GREEN}Installed jump to ${INSTALL_DIR}/jump${NC}"
}

install_via_npm() {
    echo -e "${YELLOW}Installing via npm...${NC}"

    if ! command -v npm &> /dev/null; then
        echo -e "${RED}Error: npm is not installed.${NC}"
        echo "Install Node.js first: https://nodejs.org/"
        exit 1
    fi

    npm install -g @josemaurette1/jump

    echo -e "${GREEN}Installed jump via npm${NC}"
}

install_via_homebrew() {
    echo -e "${YELLOW}Installing via Homebrew...${NC}"

    if ! command -v brew &> /dev/null; then
        echo -e "${RED}Error: Homebrew is not installed.${NC}"
        echo "Install Homebrew first: https://brew.sh/"
        exit 1
    fi

    # Check if tap exists, if not suggest adding it
    if ! brew tap | grep -q "josemaurette1/jump"; then
        echo "Adding Homebrew tap..."
        brew tap josemaurette1/jump
    fi

    brew install jump

    echo -e "${GREEN}Installed jump via Homebrew${NC}"
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

    echo -e "${GREEN}Installed shell integration to ${SHELL_SCRIPT_DIR}/jump.sh${NC}"
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
            echo -e "${GREEN}Added jump to ${rc_file}${NC}"
        else
            echo -e "${YELLOW}jump already configured in ${rc_file}${NC}"
        fi
    fi
}

show_menu() {
    echo ""
    echo "How would you like to install jump?"
    echo ""
    echo "  1) curl     - Download binary directly (fastest)"
    echo "  2) npm      - Install via npm package"
    echo "  3) homebrew - Install via Homebrew (macOS)"
    echo ""
    echo -n "Enter your choice [1-3]: "
}

main() {
    echo -e "${GREEN}Installing jump...${NC}"
    echo ""

    # Check if running in automated mode
    if [[ -n "${JUMP_INSTALL_METHOD}" ]]; then
        local method="${JUMP_INSTALL_METHOD}"
    else
        show_menu
        read method
    fi

    local platform version

    case "${method}" in
        1|curl)
            # Check for curl
            if ! command -v curl &> /dev/null; then
                echo -e "${RED}Error: curl is required for this installation method.${NC}"
                echo "Install curl or choose another method."
                exit 1
            fi
            platform=$(detect_platform)
            version=$(get_latest_version)
            if [[ -z "${version}" ]]; then
                echo -e "${RED}Error: Could not determine latest version.${NC}"
                exit 1
            fi
            install_via_curl "${platform}" "${version}"
            ;;
        2|npm)
            install_via_npm
            ;;
        3|homebrew)
            install_via_homebrew
            ;;
        *)
            echo -e "${RED}Invalid choice: ${method}${NC}"
            exit 1
            ;;
    esac

    # Only add shell integration for curl install (npm/homebrew handle this)
    if [[ "${method}" == "1" ]] || [[ "${method}" == "curl" ]]; then
        install_shell_integration
        add_to_shell_rc
    fi

    echo ""
    echo -e "${GREEN}Done!${NC}"
    echo ""
    echo "Restart your shell or run:"
    echo "  source ~/.bashrc  # or ~/.zshrc"
    echo ""
    echo "Then use 'jump' or 'j' to navigate directories."
}

main
