{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };

        # Function to generate mise.toml if it doesn't exist
        setupMise = ''
          if [ ! -f .mise.toml ]; then
            printf "📝 Generating .mise.toml...\n"
            cat > .mise.toml << 'EOF'
          # Root-level mise configuration for the entire monorepo
          # See: https://mise.jdx.dev/configuration.html

          [shell_alias]
          # Git shortcuts
          gs = "git fetch origin && git status"
          gc = "git commit"
          gp = "git push"
          gl = "git log --oneline --graph --decorate --all"
          gd = "git diff"
          ga = "git add"
          gaa = "git add --all"

          # Navigation
          rust = "cd src/rust"
          python = "cd src/python"
          nixdir = "cd src/nix"

          # Common shortcuts
          ll = "eza -la"
          cat = "bat"
          find = "fd"

          [tasks.fmt]
          description = "Format all code in monorepo"
          run = "treefmt"

          [tasks.check]
          description = "Run all checks"
          run = [
            "shellcheck **/*.sh 2>/dev/null || true",
            "actionlint .github/workflows/*.yml 2>/dev/null || true",
            "editorconfig-checker",
          ]

          [tasks.rust-dev]
          description = "Enter Rust development environment"
          run = "cd src/rust && nix develop"

          [tasks.python-dev]
          description = "Enter Python development environment"
          run = "cd src/python && nix develop"

          [env]
          # Monorepo-wide environment variables
          EOF
            printf "✓ Created .mise.toml with 'gs' alias\n"
          fi

          # Activate mise
          eval "$(mise activate bash)"
        '';
      in {
        # Root devShell - for monorepo-wide tooling
        devShells.default = pkgs.mkShell {
          inputsFrom = [(import ./shell.nix {inherit pkgs;})];
          shellHook = ''
            onefetch 2>/dev/null || true

            ${setupMise}

            printf "\n"
            printf "📁 Language-specific environments:\n"
            printf "   Rust:   cd src/rust && nix develop\n"
            # printf "   Python: cd src/python && nix develop\n"
            printf "   Nix:    cd src/nix && nix develop\n"
            printf "\n"
            printf "Or use: mise run rust-dev\n"

            # Auto-start editor
            if [ -n "$VISUAL" ]; then
              "$VISUAL"
            elif [ -n "$EDITOR" ]; then
              "$EDITOR"
            fi
          '';
        };
      }
    );
}
