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

# Check version OS and install nushell
os=`uname`
if [ $os == "Darwin" ]; then
    brew install nushell
elif [ $os == "Linux" ]; then
    # Check if apt exists
    packman=`which apt`
    if [ $? != 0 ]; then
        sudo dnf install libxcb openssl-devel libX11-devel
    else
        sudo apt install pkg-config libssl-dev libxcb openssl-devel libX11-devel
    fi
    cargo install nu --features=extra
fi

# install mononoki nerd font
if [ $os == "Darwin" ]; then
    brew tap homebrew/cask-fonts && brew install --cask font-mononoki-nerd-font
else
    wget https://github.com/ryanoasis/nerd-fonts/releases/download/v2.1.0/Mononoki.zip
    unzip Mononoki.zip -e ~/.local/share/fonts
fi

# install starship rs
cargo install starship --locked
nu ./starship.nu

# install git-delta
cargo install git-delta

# install helix
git clone https://github.com/helix-editor/helix
cd helix
cargo install --path helix-term
mkdir ~/.config/helix
ln -s $PWD/runtime ~/.config/helix/runtime
cd -

# install misc rust tools
cargo install exa 
cargo install ripgrep
cargo install procs
cargo install mdbook
cargo install fd-find
cargo install --locked broot
