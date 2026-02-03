{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    neovim.url = "github:nix-community/neovim-nightly-overlay";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    neovim,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
          overlays = [
            neovim.overlays.default
          ];
        };
      in {
        devShells.default = pkgs.mkShell {
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
            yazi
            tlrc
            tokei
            thefuck
            zoxide
            tldr
            neovim
            helix
            direnv
            mise
            fend
            fastfetch
            lesspipe
            glib
            treefmt
            coreutils-prefixed
          ];
          # inputsFrom = [ (import ./shell.nix { inherit pkgs; }) ];
          shellHook = ''
            fastfetch
            onefetch

            # if [ t1 ]; then
            #   if [ -n "$VISUAL" ] ;then
            #     "$VISUAL"
            #   elif [ -n "$EDITOR" ]; then
            #     nvim
            #   fi
            # fi
          '';
        };
      }
    );
}
