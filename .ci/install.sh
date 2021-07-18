#!/bin/sh

set -eu

INSTALL_DIR=${INSTALL_DIR:-"$HOME/.fnm"}
RELEASE="latest"
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"

FORCE_INSTALL=
SKIP_SHELL=
USE_HOMEBREW=

# Parse Flags
parse_args() {
  while [ $# -gt 0 ]; do
    key="$1"

    case $key in
      -d | --install-dir)
        INSTALL_DIR="$2"
        shift # past argument
        shift # past value
        ;;
      -s | --skip-shell)
        SKIP_SHELL=1
        shift # past argument
        ;;
      --force-install | --force-no-brew)
        echo "\`--force-install\`: I hope you know what you're doing." >&2
        FORCE_INSTALL=1
        shift
        ;;
      -r | --release)
        RELEASE="$2"
        shift # past release argument
        shift # past release value
        ;;
      *)
        echo "Unrecognized argument $key"
        exit 1
        ;;
    esac
  done
}

set_filename() {
  if [ "$OS" = "linux" ]; then
    # Based on https://stackoverflow.com/a/45125525
    case $(uname -m) in
      arm | armv7*) FILENAME="fnm-arm32" ;;
      aarch* | armv8*) FILENAME="fnm-arm64" ;;
      *) FILENAME="fnm-linux" ;;
    esac
  elif [ "$OS" = "darwin" ] && [ -n "$FORCE_INSTALL" ]; then
    FILENAME="fnm-macos"
    echo "Downloading the latest fnm binary from GitHub..."
    echo "  Pro tip: it's easier to use Homebrew for managing fnm in macOS."
    echo "           Remove the \`--force-no-brew\` so it will be easy to upgrade."
  elif [ "$OS" = "darwin" ]; then
    USE_HOMEBREW=1
    echo "Downloading fnm using Homebrew..."
  else
    echo "OS $OS is not supported."
    echo "If you think that's a bug - please file an issue to https://github.com/Schniz/fnm/issues"
    exit 1
  fi
}

download_fnm() {
  if [ -n "$USE_HOMEBREW" ]; then
    brew install fnm
    return
  fi

  case $RELEASE in
    latest) URL="https://github.com/Schniz/fnm/releases/latest/download/$FILENAME.zip" ;;
    *) URL="https://github.com/Schniz/fnm/releases/download/$RELEASE/$FILENAME.zip" ;;
  esac

  if ! DOWNLOAD_DIR=$(mktemp -d); then
    echo "Creating temporary directory failed."
    exit 1
  else
    trap 'rm -rf $DOWNLOAD_DIR' EXIT
  fi

  echo "Downloading $URL..."

  mkdir -p "$INSTALL_DIR" >/dev/null 2>&1

  if ! curl --progress-bar --fail -L "$URL" -o "$DOWNLOAD_DIR/$FILENAME.zip"; then
    echo "Download failed.  Check that the release/filename are correct."
    exit 1
  fi

  unzip -q "$DOWNLOAD_DIR/$FILENAME.zip" -d "$DOWNLOAD_DIR"

  if [ -f "$DOWNLOAD_DIR/fnm" ]; then
    install "$DOWNLOAD_DIR/fnm" "$INSTALL_DIR/fnm"
  else
    install "$DOWNLOAD_DIR/$FILENAME/fnm" "$INSTALL_DIR/fnm"
  fi
}

check_dependencies() {
  echo "Checking dependencies for the installation script..."
  SHOULD_EXIT=

  printf "Checking availability of curl... "
  if command -v curl >/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT=1
  fi

  printf "Checking availability of unzip... "
  if command -v unzip >/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT=1
  fi

  if [ -n "$USE_HOMEBREW" ]; then
    printf "Checking availability of Homebrew (brew)... "
    if command -v brew >/dev/null; then
      echo "OK!"
    else
      echo "Missing!"
      SHOULD_EXIT=1
    fi
  fi

  test -z "$SHOULD_EXIT"
}

ensure_containing_dir_exists() {
  CONTAINING_DIR="$(dirname "$1")"
  if [ ! -d "$CONTAINING_DIR" ]; then
    echo " >> Creating directory $CONTAINING_DIR"
    mkdir -p "$CONTAINING_DIR"
  fi
}

setup_shell() {
  CURRENT_SHELL="$(basename "$SHELL")"

  if [ "$CURRENT_SHELL" = "zsh" ]; then
    CONF_FILE="${ZDOTDIR:-$HOME}/.zshrc"
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Zsh. Appending the following to $CONF_FILE:"
    echo ""
    echo "  # fnm"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo "  eval \"\$(fnm env)\""

    {
      echo ""
      echo "# fnm"
      echo "export PATH=\"$INSTALL_DIR:\$PATH\""
      echo "eval \"\$(fnm env)\""
    } >>"$CONF_FILE"

  elif [ "$CURRENT_SHELL" = "fish" ]; then
    CONF_FILE="$HOME/.config/fish/conf.d/fnm.fish"
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Fish. Appending the following to $CONF_FILE:"
    echo ""
    echo "  # fnm"
    echo "  set PATH $INSTALL_DIR \$PATH"
    echo "  fnm env | source"

    {
      echo "# fnm"
      echo "set PATH $INSTALL_DIR \$PATH"
      echo "fnm env | source"
    } >>"$CONF_FILE"

  elif [ "$CURRENT_SHELL" = "bash" ]; then
    case $OS in
      darwin) CONF_FILE="$HOME/.profile" ;;
      *) CONF_FILE="$HOME/.bashrc" ;;
    esac
    ensure_containing_dir_exists "$CONF_FILE"
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo "  # fnm"
    echo "  export PATH=\"$INSTALL_DIR:\$PATH\""
    echo "  eval \"\$(fnm env)\""

    {
      echo ""
      echo "# fnm"
      echo "export PATH=\"$INSTALL_DIR:\$PATH\""
      echo "eval \"\$(fnm env)\""
    } >>"$CONF_FILE"

  else
    echo "Could not infer shell type. Please set up manually."
    exit 1
  fi

  echo ""
  echo "In order to apply the changes, open a new terminal or run the following command:"
  echo ""
  echo "  source $CONF_FILE"
}

parse_args "$@"
set_filename
check_dependencies
download_fnm
if [ -z "$SKIP_SHELL" ]; then
  setup_shell
fi
