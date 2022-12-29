#!/usr/bin/env nu

echo "Checking if rustup exists"
let exists = (^which rustup | complete)
if $exists.exit_code != 0 {
    echo "Please install rustup"
    exit 1
}
rustup update


let os = (uname)
if $os == "Linux" {
    let packman = (^which apt | complete)
    if &packman.exit_code == 0 {
        sudo apt install pkg-config libssl-dev libxcb openssl-devel libX11-devel
    } else {
        sudo dnf install libxcb openssl-devel libX11-devel
    }
}