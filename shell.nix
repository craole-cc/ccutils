{pkgs ? import <nixpkgs> {config.allowUnfree = true;}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    # pre-commit
    eza
    bat
    fd
    btop
    ripgrep
    fzf
    lsd
    tokei
    onefetch
    zoxide
    # treefmt
    # shfmt # Shellscript Formatter
    # shellcheck # Shellscript Linter
    # taplo # Toml Formatter
    # stylua # Lua Formatter
    # yamlfmt # YamlFormatter
    # typst # Typesetting system
    # typstfmt # Typst Formatter
    # typstyle # Typst Linter
    # actionlint # GitHub Actions
    # editorconfig-checker # EditorConfig
    # eclint # EditorConfig
    markdownlint-cli2 # Markdown
    # keep-sorted # Sorter
  ];
}
