#!/bin/sh
# shellcheck enable=all
# Initialize development environment with best available method

set -eu

#╔═══════════════════════════════════════════════════════════╗
#║ Environment                                               ║
#╚═══════════════════════════════════════════════════════════╝

initialize_defaults() {
	PRJ_ROOT_RELATIVE="$(cd "$(dirname "$0")/." && pwd)"
	PRJ_ROOT="${PRJ_ROOT:-$PRJ_ROOT_RELATIVE}"
	SRC_ROOT="${SRC_ROOT:-"${PRJ_ROOT}/src"}"

	unset LANG_NAME VERBOSE LANG_ROOT
}

parse_arguments() {
	while [ $# -gt 0 ] && [ "${1#-}" != "$1" ]; do
		case "$1" in
		-h | --help)
			printf "Usage: %s [OPTIONS] [language]\n" "$(basename "$0")"
			printf "\n"
			printf "Arguments:\n"
			printf "  language              Language environment to initialize (e.g., 'rust', 'nix')\n"
			printf "\n"
			printf "Options:\n"
			printf "  -h, --help           Show this help message\n"
			printf "  -v, --verbose        Enable verbose output\n"
			printf "  -d, --debug          Enable debug mode (very verbose)\n"
			printf "  -l, --language NAME  Specify language explicitly\n"
			exit 0
			;;
		-d | --debug)
			set -x
			VERBOSE=true
			;;
		-v | --verbose)
			VERBOSE=true
			;;
		-l | --language | --source)
			if [ -n "${2:-}" ]; then
				LANG_NAME="$2"
				shift
			else
				printe --parse-arg "$1"
			fi
			;;
		--*) break ;;
		-*)
			printe "$(printf "Unknown option: %s\n" "$1")"
			;;
		*)
			printe "$(printf "Unknown option: %s\n" "$1")"
			;;
		esac
		shift
	done

	#> Handle positional argument for language
	if [ $# -gt 0 ] && [ -z "${LANG_NAME:-}" ]; then
		LANG_NAME="$1"
		shift
	fi
}

validate_arguments() {
	#? Language
	if [ -n "${LANG_NAME:-}" ]; then
		printf "Selected language: %s\n" "${LANG_NAME}"
	else
		printf "No language specified, using default environment\n"
		return 0
	fi

	#? Language Directory
	LANG_ROOT="${SRC_ROOT}/${LANG_NAME}"
	if ! [ -d "${LANG_ROOT}" ]; then
		printe "Source directory not found: src/${LANG_NAME}"
	fi
}

initialize_environment() {
	#> Detect and use best available method
	if has_flakes; then
		use_flakes "${LANG_NAME:-}"
	elif has_nix; then
		use_nix "${LANG_ROOT:-}"
	else
		use_mise "${LANG_NAME:-}"
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Utilities                                                 ║
#╚═══════════════════════════════════════════════════════════╝

printe() {
	keep_alive=0
	code=1
	printf "Error: " >&2
	while [ $# -gt 0 ]; do
		case "$1" in
		-p | --parse-arg)
			printf "%s requires an argument\n" "$2" >&2
			;;
		-c | --continue)
			keep_alive=1
			;;
		-x | --code)
			if [ -n "${2:-}" ]; then
				code="$2"
				shift
			else
				printe --parse-arg "$1"
			fi
			;;
		*)
			printf "%s\n" "$1" >&2
			;;
		esac
		shift
	done

	if [ "${keep_alive}" -eq 1 ]; then
		return "${code}"
	else
		exit "${code}"
	fi
}

has_nix() {
	command -v nix >/dev/null 2>&1
}

has_flakes() {
	if has_nix; then
		nix flake --version >/dev/null 2>&1
	else
		return 1
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Runners                                                   ║
#╚═══════════════════════════════════════════════════════════╝

use_flakes() {
	printf "✓ Nix with flakes detected\n"
	printf "Entering development environment...\n\n"

	if [ -n "${1:-}" ]; then
		#? Language-specific shell
		nix develop ".#${1}"
	else
		#? Default shell
		nix develop
	fi
}

use_nix() {
	printf "✓ Nix detected (flakes not available)\n"
	printf "Entering development environment...\n\n"

	if [ -n "${1:-}" ]; then
		#> Language-specific shell via default.nix
		nix-shell "$1"
	else
		# Default shell
		nix-shell
	fi
}

use_mise() {
	printf "⚠️  Nix not detected\n"
	printf "Initializing with mise...\n\n"

	#> Check for mise
	if ! command -v mise >/dev/null 2>&1; then
		printf "❌ mise is not installed\n"
		printf "Install from: https://mise.jdx.dev/getting-started.html\n"
		printf "\n"
		printf "Quick install:\n"
		printf "  curl https://mise.run | sh\n"
		printf "  # or on Windows:\n"
		printf "  winget install jdx.mise\n"
		exit 1
	else
		mise --version
	fi

	#> Run the mise initialization script
	if [ -f "${PRJ_ROOT}/templates/init-project.sh" ]; then
		sh "${PRJ_ROOT}/templates/init-project.sh"
	else
		printf "❌ templates/init-project.sh not found\n"
		exit 1
	fi

	printf "\n"
	printf "To activate mise:\n"
	printf "  eval \"\$(mise activate bash)\"  # or zsh, fish, etc.\n"

	if [ -n "${1:-}" ]; then
		printf "\nThen navigate to: cd src/${1}\n"
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Main Entry Point                                          ║
#╚═══════════════════════════════════════════════════════════╝

main() {
	initialize_defaults
	parse_arguments "$@" || return "$?"
	validate_arguments
	initialize_environment
} && main "$@"
