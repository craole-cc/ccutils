#!/bin/sh
# shellcheck enable=all
# Language-specific shell hook

set -eu

LANG_NAME="${1:-}"
MISE_TEMPLATE="${2:-}"

if [ -z "${LANG_NAME}" ]; then
	printf "Usage: shellhook-lang.sh <language-name> <mise-template>\n" >&2
	exit 1
fi

printf "%s development environment\n" "${LANG_NAME}"

#> Setup mise from template
if [ ! -f .mise.toml ] && [ -n "${MISE_TEMPLATE}" ]; then
	printf "📝 Generating .mise.toml...\n"
	cp "${MISE_TEMPLATE}" .mise.toml
	printf "✓ Created .mise.toml\n"
	mise trust 2>/dev/null || true
fi

#> Activate mise
eval "$(mise activate bash)" 2>/dev/null || true
