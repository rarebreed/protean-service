
let cfg = "mkdir ~/.cache/starship
starship init nu | save ~/.cache/starship/init.nu"

echo $cfg | save --raw --append $nu.env-path

let txt = "source ~/.cache/starship/init.nu"
echo $txt | save --raw --append $nu.config-path