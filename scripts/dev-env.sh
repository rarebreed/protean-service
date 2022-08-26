#!/bin/zsh
#
# This script will install some useful rust dev tools for you
# - rustup check
# - starship prompt editor
# - nushell

# Make sure we have rustup and therefore rust tools installed
rustup_check=`rustup update`
if [ $? != 0 ]; then
  echo "Please install rustup"
  echo "Go to https://rustup.rs/ and follow directions"
  exit 1
fi

# Check version OS


# install nushell


# install mononoki nerd font
