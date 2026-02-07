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

  shellHook = let
    shellhook-root = "${./templates/shellhook-root.sh}";
  in ''
    # export MISE_TEMPLATE="${./templates/mise-root.toml}"
    chmod +x ${shellhook-root}
    ./${shellhook-root}
  '';
}
