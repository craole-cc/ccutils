#!/bin/sh
# shellcheck enable=all

set -eu

TEMPLATE_FILE="${1:-}"
OUTPUT_FILE="${2:-.mise.toml}"

if [ -z "${TEMPLATE_FILE}" ]; then
	printf "Usage: setup-mise.sh <template-file> [output-file]\n" >&2
	exit 1
fi

if [ ! -f "${OUTPUT_FILE}" ]; then
	printf "📝 Generating %s...\n" "${OUTPUT_FILE}"
	cp "${TEMPLATE_FILE}" "${OUTPUT_FILE}"
	printf "✓ Created %s\n" "${OUTPUT_FILE}"
	mise trust 2>/dev/null || true
else
	printf "ℹ️  %s already exists\n" "${OUTPUT_FILE}"
fi
