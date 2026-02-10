#!/bin/sh
# shellcheck enable=all
#~@ Base shell hook

set -eu

#> Display project info
onefetch 2>/dev/null || true

#> Setup mise
MISE_TEMPLATE="${TEMPLATES:?}/mise-root.toml"
if [ ! -f .mise.toml ] && [ -n "${MISE_TEMPLATE:-}" ]; then
	printf "📝 Generating .mise.toml...\n"
	cp "${MISE_TEMPLATE}" .mise.toml
	printf "✓ Created .mise.toml\n"
	mise trust 2>/dev/null || true
fi

#> Activate mise
if command -v mise >/dev/null 2>&1; then
	eval "$(mise activate bash 2>/dev/null)" || true
	mise trust 2>/dev/null || true
fi

echo "done"
