#!/bin/sh
# shellcheck enable=all
# Base shell hook

set -eu

# Display project info
onefetch 2>/dev/null || true

# # Setup mise
# if [ ! -f .mise.toml ] && [ -n "${MISE_TEMPLATE:-}" ]; then
# 	printf "📝 Generating .mise.toml...\n"
# 	cp "${MISE_TEMPLATE}" .mise.toml
# 	printf "✓ Created .mise.toml\n"
# 	mise trust 2>/dev/null || true
# fi

# # Activate mise - handle the unbound variable issue
# if command -v mise >/dev/null 2>&1; then
# 	# Temporarily disable -u, activate, then re-enable
# 	set +u
# 	eval "$(mise activate bash 2>/dev/null)" || true
# 	set -u
# fi

#? Display navigation info
printf "\n"
printf "📁 Language-specific environments:\n"
printf "   Rust:   cd src/rust && nix develop\n"
printf "   Python: cd src/python && nix develop\n"
printf "   Nix:    cd src/nix && nix develop\n"
printf "\n"
printf "Or use: mise run rust-dev\n"
