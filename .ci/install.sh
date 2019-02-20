#!/bin/bash

OS=$(uname -a | cut -d" " -f 1)

if [ "$OS" == "Darwin" ]; then
  FILENAME="fnm-macos"
elif [ "$OS" == "Linux" ]; then
  FILENAME="fnm-linux"
else
  echo "OS $OS is not supported."
  echo "If you think that's a bug - please file an issue to https://github.com/Schniz/fnm/issues"
  exit 1
fi

get_latest_release() {
  # Taken from https://gist.github.com/lukechilds/a83e1d7127b78fef38c2914c4ececc3c
  curl --silent "https://api.github.com/repos/$1/releases/latest" | # Get latest release from GitHub api
    grep '"tag_name":' |                                            # Get tag line
    sed -E 's/.*"([^"]+)".*/\1/'                                    # Pluck JSON value
}

download_fnm() {
  LATEST_RELEASE=$(get_latest_release Schniz/fnm)
  URL=https://github.com/Schniz/fnm/releases/download/$LATEST_RELEASE/$FILENAME.zip
  DOWNLOAD_DIR=$(mktemp -d)

  echo "Downloading $URL..."

  mkdir -p $HOME/.fnm &> /dev/null
  curl --progress-bar -L $URL -o $DOWNLOAD_DIR/$FILENAME.zip
  unzip -q $DOWNLOAD_DIR/$FILENAME.zip -d $DOWNLOAD_DIR
  mv $DOWNLOAD_DIR/$FILENAME/fnm $HOME/.fnm/fnm
  chmod u+x $HOME/.fnm/fnm
}

check_dependencies() {
  echo "Checking dependencies for the installation script..."

  echo -n "Checking availablity of curl... "
  if hash curl 2>/dev/null; then
    echo "OK!"
  else
    echo "Missing!"
    SHOULD_EXIT="true"
  fi

  echo -n "Checking availablity of unzip... "
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
    echo '  export PATH=$HOME/.fnm:$PATH'
    echo '  eval `fnm env --multi`'

    echo '' >> $CONF_FILE
    echo '# fnm' >> $CONF_FILE
    echo 'export PATH=$HOME/.fnm:$PATH' >> $CONF_FILE
    echo 'eval `fnm env --multi`' >> $CONF_FILE

  elif [ "$CURRENT_SHELL" == "fish" ]; then
    CONF_FILE=$HOME/.config/fish/config.fish
    echo "Installing for Fish. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # fnm'
    echo '  set PATH $HOME/.fnm $PATH'
    echo '  eval (fnm env --multi --fish)'

    echo '' >> $CONF_FILE
    echo '# fnm' >> $CONF_FILE
    echo 'set PATH $HOME/.fnm $PATH' >> $CONF_FILE
    echo 'eval (fnm env --multi --fish)' >> $CONF_FILE

  elif [ "$CURRENT_SHELL" == "bash" ]; then
    if [ "$OS" == "Darwin" ]; then
      CONF_FILE=$HOME/.profile
    else
      CONF_FILE=$HOME/.bashrc
    fi
    echo "Installing for Bash. Appending the following to $CONF_FILE:"
    echo ""
    echo '  # fnm'
    echo '  export PATH=$HOME/.fnm:$PATH'
    echo '  eval `fnm env --multi`'

    echo '' >> $CONF_FILE
    echo '# fnm' >> $CONF_FILE
    echo 'export PATH=$HOME/.fnm:$PATH' >> $CONF_FILE
    echo 'eval `fnm env --multi`' >> $CONF_FILE

  else
    echo "Could not infer shell type. Please set up manually."
    exit 1
  fi

  echo ""
  echo "In order to apply the changes, open a new terminal or run the following command:"
  echo ""
  echo "  source $CONF_FILE"
}

check_dependencies
download_fnm
setup_shell
