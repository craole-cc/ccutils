{pkgs ? import <nixpkgs> {config.allowUnfree = true;}}:
pkgs.mkShell {
  buildInputs = with pkgs; [
    pre-commit
    eza
    bat
    fd
    btop
    ripgrep
    mise
    fzf
    lsd
    tokei
    onefetch
    zoxide
    treefmt
    shfmt
    shellcheck
    taplo
    stylua
    yamlfmt
    typst
    typstyle
    actionlint
    editorconfig-checker
    eclint
    markdownlint-cli2
    keep-sorted
  ];

  shellHook = ''
    PRJ_ROOT="$(pwd -P)"
    PRJ_CODE="$PRJ_ROOT/src"
    PRJ_SCRIPTS="$PRJ_ROOT/scripts"
    PRJ_TEMPLATES="$PRJ_ROOT/templates"
    export PRJ_ROOT PRJ_CODE PRJ_TEMPLATES PRJ_SCRIPTS

    init="$PRJ_SCRIPTS/init.sh"
    [ -f "$init" ] && . "$init"
  '';
}
