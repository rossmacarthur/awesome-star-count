# awesome-star-count

Given an [awesome](https://github.com/sindresorhus/awesome) list print out the
repositories with the most stars.


## Example usage

```
export GITHUB_TOKEN="<token>"

cargo run -- https://raw.githubusercontent.com/unixorn/awesome-zsh-plugins/master/README.md
```

Output

```
 awesome-zsh-plugins: ; stars = None
     Other Resources: ; stars = None
         Other Useful Lists: ; stars = None
             awesome-sysadmin: https://github.com/n1trux/awesome-sysadmin; stars = Some(10578)
             Terminals Are Sexy: https://github.com/k4m4/terminals-are-sexy; stars = Some(9178)
             awesome-devenv: https://github.com/jondot/awesome-devenv; stars = Some(1836)
         ZSH Tools: ; stars = None
             zshdb: https://github.com/rocky/zshdb; stars = Some(195)
             zunit: https://github.com/zunit-zsh/zunit; stars = Some(122)
     Installation: ; stars = None
         Prezto: https://github.com/sorin-ionescu/prezto; stars = Some(11324)
         Antigen: https://github.com/zsh-users/antigen; stars = Some(5950)
         zplug: https://github.com/zplug/zplug; stars = Some(3947)
         Antibody: https://github.com/getantibody/antibody; stars = Some(1389)
         Zgen: https://github.com/tarjoilija/zgen; stars = Some(1221)
         dotzsh: https://github.com/dotphiles/dotzsh; stars = Some(183)
         zpm: https://github.com/zpm-zsh/zpm; stars = Some(85)
     Themes: ; stars = None
         Fonts: ; stars = None
             Nerd Fonts: https://github.com/ryanoasis/nerd-fonts; stars = Some(22247)
             Powerline patched font collection: https://github.com/powerline/fonts; stars = Some(19653)
             Iosevka: https://github.com/be5invis/Iosevka; stars = Some(10313)
             Fantasque-sans: https://github.com/belluzj/fantasque-sans; stars = Some(4957)
             Awesome Terminal Fonts: https://github.com/gabrielelana/awesome-terminal-fonts; stars = Some(2096)
             Fantasque Awesome Font: https://github.com/ztomer/fantasque_awesome_powerline; stars = Some(28)
         powerlevel9k: https://github.com/bhilburn/powerlevel9k; stars = Some(12757)
         spaceship: https://github.com/denysdovhan/spaceship-prompt; stars = Some(12291)
         pure: https://github.com/sindresorhus/pure; stars = Some(9026)
         powerlevel10k: https://github.com/romkatv/powerlevel10k; stars = Some(8233)
         powerline-shell (banga): https://github.com/b-ryan/powerline-shell; stars = Some(5264)
         powerline-shell (b-ryan): https://github.com/b-ryan/powerline-shell; stars = Some(5264)
         liquidprompt: https://github.com/nojhan/liquidprompt; stars = Some(3940)
         oh-my-git: https://github.com/arialdomartini/oh-my-git; stars = Some(3347)
         bullet-train: https://github.com/caiogondim/bullet-train.zsh; stars = Some(2457)
     Completions: ; stars = None
         completions: https://github.com/zsh-users/zsh-completions; stars = Some(3636)
         git-flow-completion: https://github.com/bobthecow/git-flow-completion; stars = Some(2604)
         gradle-completion (gradle): https://github.com/gradle/gradle-completion; stars = Some(693)
         yarn: https://github.com/g-plane/zsh-yarn-autocompletions; stars = Some(427)
         better-npm-completion: https://github.com/lukechilds/zsh-better-npm-completion; stars = Some(307)
         docker (felixr): https://github.com/felixr/docker-zsh-completion; stars = Some(254)
         conda-zsh-completion: https://github.com/esc/conda-zsh-completion; stars = Some(134)
         nix-zsh-completions: https://github.com/spwhitt/nix-zsh-completions; stars = Some(81)
         salesforce-cli: https://github.com/wadewegner/salesforce-cli-zsh-completion; stars = Some(77)
         gcloud-zsh-completion: https://github.com/littleq0903/gcloud-zsh-completion; stars = Some(71)
     Plugins: ; stars = None
         autosuggestions: https://github.com/zsh-users/zsh-autosuggestions; stars = Some(13909)
         autojump: https://github.com/wting/autojump; stars = Some(11348)
         syntax-highlighting: https://github.com/zsh-users/zsh-syntax-highlighting; stars = Some(9689)
         blackbox: https://github.com/StackExchange/blackbox; stars = Some(5333)
         git-secret: https://github.com/sobolevn/git-secret; stars = Some(1890)
         kube-ps1: https://github.com/jonmosco/kube-ps1; stars = Some(1757)
         z.lua: https://github.com/skywind3000/z.lua; stars = Some(1615)
         enhancd: https://github.com/b4b4r07/enhancd; stars = Some(1590)
         ansiweather: https://github.com/fcambus/ansiweather; stars = Some(1541)
         k: https://github.com/supercrabtree/k; stars = Some(1333)
     Tutorials: ; stars = None
         Zinit (née zplugin): ; stars = None
             BlaCk-Void-Zsh: https://github.com/black7375/BlaCk-Void-Zsh; stars = Some(146)
             zinit-configs: https://github.com/zdharma/zinit-configs; stars = Some(78)
         Zgen: ; stars = None
             zsh-quickstart-kit: https://github.com/unixorn/zsh-quickstart-kit; stars = Some(289)
             rad-shell: https://github.com/brandon-fryslie/rad-shell; stars = Some(30)
         Antigen: ; stars = None
             belak/zsh-utils: https://github.com/belak/zsh-utils; stars = Some(19)
         Generic ZSH: ; stars = None
             ZSH Pony: https://github.com/mika/zsh-pony; stars = Some(144)
             xVanjaZ: https://github.com/xVanjaZ/xVanjaZ-ZSH-Theme; stars = Some(0)
     Frameworks: ; stars = None
         Zinit: https://github.com/zdharma/zinit; stars = None
             services: https://github.com/zservices; stars = None
             packages: https://github.com/Zsh-Packages; stars = None
         prezto: https://github.com/sorin-ionescu/prezto; stars = Some(11324)
         antigen: https://github.com/zsh-users/antigen; stars = Some(5950)
         zplug: https://github.com/zplug/zplug; stars = Some(3947)
         zim: https://github.com/zimfw/zimfw; stars = Some(1643)
         antibody: https://github.com/getantibody/antibody; stars = Some(1389)
         zgen: https://github.com/tarjoilija/zgen; stars = Some(1221)
         fresh: https://github.com/freshshell/fresh; stars = Some(974)
         ztanesh: https://github.com/miohtama/ztanesh; stars = Some(262)
         antigen-hs: https://github.com/Tarrasch/antigen-hs; stars = Some(197)
```
