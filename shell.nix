{pkgs ? import <nixpkgs> {config.allowUnfree = true;}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    getoptions
    pre-commit
    eza
    bat
    fd
    btop
    ripgrep
    fzf
    lsd
    delta
    jujutsu
    yazi
    tlrc
    tokei
    onefetch
    thefuck
    zoxide
    tldr
    lesspipe
    glib
    treefmt
    coreutils-prefixed
    shfmt # Shellscript Formatter
    shellcheck # Shellscript Linter
    taplo # Toml Formatter
    stylua # Lua Formatter
    yamlfmt # YamlFormatter
    typst # Typesetting system
    typstfmt # Typst Formatter
    typstyle # Typst Linter
    actionlint # GitHub Actions
    editorconfig-checker # EditorConfig
    eclint # EditorConfig
    markdownlint-cli2 # Markdown
    keep-sorted # Sorter
  ];
}
