# awesome-star-count

Given an [awesome](https://github.com/sindresorhus/awesome) list print out the
repositories with the most stars while preserving the heading tree structure.

## Example usage

```
export GITHUB_TOKEN="<token>"

cargo run -- https://raw.githubusercontent.com/unixorn/awesome-zsh-plugins/master/README.md
```

Output

```
 awesome-zsh-plugins:
     Other Resources:
         Other Useful Lists:
             awesome-sysadmin: https://github.com/n1trux/awesome-sysadmin ★ 10597
             Terminals Are Sexy: https://github.com/k4m4/terminals-are-sexy ★ 9176
             awesome-devenv: https://github.com/jondot/awesome-devenv ★ 1837
         ZSH Tools:
             zshdb: https://github.com/rocky/zshdb ★ 196
             zunit: https://github.com/zunit-zsh/zunit ★ 122
     Installation:
         Prezto: https://github.com/sorin-ionescu/prezto ★ 11329
         Antigen: https://github.com/zsh-users/antigen ★ 5954
         zplug: https://github.com/zplug/zplug ★ 3950
         Antibody: https://github.com/getantibody/antibody ★ 1391
         Zgen: https://github.com/tarjoilija/zgen ★ 1221
         dotzsh: https://github.com/dotphiles/dotzsh ★ 183
         zpm: https://github.com/zpm-zsh/zpm ★ 85
     Themes:
         Fonts:
             Nerd Fonts: https://github.com/ryanoasis/nerd-fonts ★ 22284
             Powerline patched font collection: https://github.com/powerline/fonts ★ 19669
             Iosevka: https://github.com/be5invis/Iosevka ★ 10321
             Fantasque-sans: https://github.com/belluzj/fantasque-sans ★ 4958
             Awesome Terminal Fonts: https://github.com/gabrielelana/awesome-terminal-fonts ★ 2098
             Fantasque Awesome Font: https://github.com/ztomer/fantasque_awesome_powerline ★ 28
         powerlevel9k: https://github.com/bhilburn/powerlevel9k ★ 12763
         spaceship: https://github.com/denysdovhan/spaceship-prompt ★ 12306
         pure: https://github.com/sindresorhus/pure ★ 9036
         powerlevel10k: https://github.com/romkatv/powerlevel10k ★ 8286
         powerline-shell (banga): https://github.com/b-ryan/powerline-shell ★ 5265
         powerline-shell (b-ryan): https://github.com/b-ryan/powerline-shell ★ 5265
         liquidprompt: https://github.com/nojhan/liquidprompt ★ 3940
         oh-my-git: https://github.com/arialdomartini/oh-my-git ★ 3348
         bullet-train: https://github.com/caiogondim/bullet-train.zsh ★ 2459
     Completions:
         completions: https://github.com/zsh-users/zsh-completions ★ 3638
         git-flow-completion: https://github.com/bobthecow/git-flow-completion ★ 2606
         gradle-completion (gradle): https://github.com/gradle/gradle-completion ★ 694
         yarn: https://github.com/g-plane/zsh-yarn-autocompletions ★ 428
         better-npm-completion: https://github.com/lukechilds/zsh-better-npm-completion ★ 307
         docker (felixr): https://github.com/felixr/docker-zsh-completion ★ 254
         conda-zsh-completion: https://github.com/esc/conda-zsh-completion ★ 134
         nix-zsh-completions: https://github.com/spwhitt/nix-zsh-completions ★ 81
         salesforce-cli: https://github.com/wadewegner/salesforce-cli-zsh-completion ★ 77
         gcloud-zsh-completion: https://github.com/littleq0903/gcloud-zsh-completion ★ 71
     Plugins:
         autosuggestions: https://github.com/zsh-users/zsh-autosuggestions ★ 13930
         autojump: https://github.com/wting/autojump ★ 11362
         syntax-highlighting: https://github.com/zsh-users/zsh-syntax-highlighting ★ 9702
         blackbox: https://github.com/StackExchange/blackbox ★ 5337
         git-secret: https://github.com/sobolevn/git-secret ★ 1894
         kube-ps1: https://github.com/jonmosco/kube-ps1 ★ 1760
         z.lua: https://github.com/skywind3000/z.lua ★ 1617
         enhancd: https://github.com/b4b4r07/enhancd ★ 1592
         ansiweather: https://github.com/fcambus/ansiweather ★ 1543
         k: https://github.com/supercrabtree/k ★ 1333
     Tutorials:
         Zinit (née zplugin):
             BlaCk-Void-Zsh: https://github.com/black7375/BlaCk-Void-Zsh ★ 146
             zinit-configs: https://github.com/zdharma/zinit-configs ★ 81
         Zgen:
             zsh-quickstart-kit: https://github.com/unixorn/zsh-quickstart-kit ★ 289
             rad-shell: https://github.com/brandon-fryslie/rad-shell ★ 30
         Antigen:
             belak/zsh-utils: https://github.com/belak/zsh-utils ★ 19
         Generic ZSH:
             ZSH Pony: https://github.com/mika/zsh-pony ★ 144
             xVanjaZ: https://github.com/xVanjaZ/xVanjaZ-ZSH-Theme ★ 0
     Frameworks:
         Zinit:
             services:
             packages:
         prezto: https://github.com/sorin-ionescu/prezto ★ 11329
         antigen: https://github.com/zsh-users/antigen ★ 5954
         zplug: https://github.com/zplug/zplug ★ 3950
         zim: https://github.com/zimfw/zimfw ★ 1646
         antibody: https://github.com/getantibody/antibody ★ 1391
         zgen: https://github.com/tarjoilija/zgen ★ 1221
         fresh: https://github.com/freshshell/fresh ★ 974
         ztanesh: https://github.com/miohtama/ztanesh ★ 262
         antigen-hs: https://github.com/Tarrasch/antigen-hs ★ 197```
