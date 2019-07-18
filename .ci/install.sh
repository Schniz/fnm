#!/bin/bash

set -e

OS=$(uname -s)

if [ "$OS" == "Darwin" ]; then
  FILENAME="fnm-macos"
elif [ "$OS" == "Linux" ]; then
  FILENAME="fnm-linux"
else
  echo "OS $OS is not supported."
  echo "If you think that's a bug - please file an issue to https://github.com/Schniz/fnm/issues"
  exit 1
fi

INSTALL_DIR="$HOME/.fnm"

# Parse Flags
parse_args() {
  while [[ $# -gt 0 ]]; do
    key="$1"

    case $key in
    -d | --install-dir)
      INSTALL_DIR="$2"
      shift # past argument
      shift # past value
      ;;
    -s | --skip-shell)
      SKIP_SHELL="true"
      shift # past argument
      ;;
    *)
      echo "Unrecognized argument $key"
      exit 1
      ;;
    esac
  done
}

download_fnm() {
  URL=https://github.com/Schniz/fnm/releases/latest/download/$FILENAME.zip
  DOWNLOAD_DIR=$(mktemp -d)

  echo "Downloading $URL..."

  mkdir -p $INSTALL_DIR &>/dev/null
  curl --progress-bar -L $URL -o $DOWNLOAD_DIR/$FILENAME.zip
  unzip -q $DOWNLOAD_DIR/$FILENAME.zip -d $DOWNLOAD_DIR
  mv $DOWNLOAD_DIR/$FILENAME/fnm $INSTALL_DIR/fnm
  chmod u+x $INSTALL_DIR/fnm
}

check_dependencies() {
  echo "Checking dependencies for the installation script..."

  echo -n "Checking availability of curl... "
  if hash curl 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  echo -n "Checking availability of unzip... "
  if hash unzip 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  if [ "$SHOULD_EXIT" = "true" ]; then
    exit 1
  fi
}

setup_shell() {
  CURRENT_SHELL=$(basename $SHELL)

  if [ "$CURRENT_SHELL" == "zsh" ]; then
    CONF_FILE=$HOME/.zshrc
    echo "Installing for Zsh. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # fnm'
    echo '  export PATH='"$INSTALL_DIR"':$PATH'
    echo '  eval "`fnm env --multi`"'

    echo '' >>$CONF_FILE
    echo '# fnm' >>$CONF_FILE
    echo 'export PATH='$INSTALL_DIR':$PATH' >>$CONF_FILE
    echo 'eval "`fnm env --multi`"' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" == "fish" ]; then
    CONF_FILE=$HOME/.config/fish/config.fish
    echo "Installing for Fish. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # fnm'
    echo '  set PATH '"$INSTALL_DIR"' $PATH'
    echo '  fnm env --multi | source'

    echo '' >>$CONF_FILE
    echo '# fnm' >>$CONF_FILE
    echo 'set PATH '"$INSTALL_DIR"' $PATH' >>$CONF_FILE
    echo 'fnm env --multi | source' >>$CONF_FILE

  elif [ "$CURRENT_SHELL" == "bash" ]; then
    if [ "$OS" == "Darwin" ]; then
      CONF_FILE=$HOME/.profile
    else
      CONF_FILE=$HOME/.bashrc
    fi
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # fnm'
    echo '  export PATH='"$INSTALL_DIR"':$PATH'
    echo '  eval "`fnm env --multi`"'

    echo '' >>$CONF_FILE
    echo '# fnm' >>$CONF_FILE
    echo 'export PATH='"$INSTALL_DIR"':$PATH' >>$CONF_FILE
    echo 'eval "`fnm env --multi`"' >>$CONF_FILE

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
check_dependencies
download_fnm
if [ "$SKIP_SHELL" != "true" ]; then
  setup_shell
fi
