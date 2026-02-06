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
    shfmt # Shellscript Formatter
    shellcheck # Shellscript Linter
    taplo # Toml Formatter
    stylua # Lua Formatter
    yamlfmt # YamlFormatter
    typst # Typesetting system
    typstyle # Typst Linter
    actionlint # GitHub Actions
    editorconfig-checker # EditorConfig
    eclint # EditorConfig
    markdownlint-cli2 # Markdown
    keep-sorted # Sorter
  ];

  shellHook = ''
        # Display project info
        onefetch 2>/dev/null || true

        # Generate mise.toml if it doesn't exist
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
          printf "✓ Created .mise.toml\n"

          # Auto-trust the generated config
          mise trust 2>/dev/null || true
        fi

        # Activate mise
        eval "$(mise activate bash)" 2>/dev/null || true

        printf "\n"
        printf "📁 Language-specific environments:\n"
        printf "   Rust:   cd src/rust && nix develop\n"
        printf "   Python: cd src/python && nix develop\n"
        printf "   Nix:    cd src/nix && nix develop\n"
        printf "\n"
        printf "Or use: mise run rust-dev\n"

        # Auto-start editor if available
        if [ -n "$VISUAL" ] && command -v "$VISUAL" &> /dev/null; then
          "$VISUAL"
        elif [ -n "$EDITOR" ] && command -v "$EDITOR" &> /dev/null; then
          "$EDITOR"
        fi
  '';
}
