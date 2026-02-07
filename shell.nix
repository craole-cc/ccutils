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
    TEMPLATES="$PRJ_ROOT/templates"
    "$PRJ_/templates/shellhook-root.sh}";
    # export MISE_TEMPLATE="${./templates/mise-root.toml}"

    chmod +x $TEMPLATES/shellhook-root.sh
    ./$TEMPLATES/shellhook-root.sh
  '';
}
