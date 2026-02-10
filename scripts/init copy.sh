#!/bin/sh
# shellcheck enable=all
# Initialize development environment with best available method

set -eu

#╔═══════════════════════════════════════════════════════════╗
#║ Environment                                               ║
#╚═══════════════════════════════════════════════════════════╝

initialize_defaults() {
	# Get script directory
	PRJ_ROOT_RELATIVE="$(cd "$(dirname "$0")" && pwd)"
	PRJ_ROOT="${PRJ_ROOT:-${PRJ_ROOT_RELATIVE}}"
	SRC_ROOT="${SRC_ROOT:-"${PRJ_ROOT}/src"}"
	TEMPLATES="${TEMPLATES:-"${PRJ_ROOT}/templates"}"

	# Parse and normalize verbosity
	VERBOSITY="${VERBOSITY:-${VERBOSE:-}}"
	VERBOSITY_LEVEL="$(parse_verbosity "${VERBOSITY}")"

	unset LANG_NAME LANG_ROOT
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
			printf "  -q, --quiet          Suppress all output (verbosity=0)\n"
			printf "  -v, --verbose        Increase verbosity (can be used multiple times)\n"
			printf "  -d, --debug          Enable debug mode (verbosity=4)\n"
			printf "  -t, --trace          Enable trace mode (verbosity=5) with shell tracing\n"
			printf "  -l, --language NAME  Specify language explicitly\n"
			printf "\n"
			printf "Verbosity levels:\n"
			printf "  0: quiet (errors only)\n"
			printf "  1: error\n"
			printf "  2: warning\n"
			printf "  3: info (default)\n"
			printf "  4: debug\n"
			printf "  5: trace\n"
			exit 0
			;;
		-t | --trace)
			set -x
			VERBOSITY_LEVEL=5
			;;
		-d | --debug)
			VERBOSITY_LEVEL=4
			;;
		-v | --verbose)
			# Increase verbosity by 1 level for each -v
			if [ "${VERBOSITY_LEVEL:-3}" -lt 5 ]; then
				VERBOSITY_LEVEL="$((VERBOSITY_LEVEL + 1))"
			fi
			;;
		-q | --quiet)
			VERBOSITY_LEVEL=0
			;;
		-l | --language | --source)
			if [ -n "${2:-}" ]; then
				LANG_NAME="$2"
				shift
			else
				print_error --parse-arg "$1"
			fi
			;;
		--*) break ;;
		-*)
			print_error "Unknown option: $1"
			;;
		*)
			print_error "Unknown option: $1"
			;;
		esac
		shift
	done

	#> Handle positional argument for language
	if [ $# -gt 0 ] && [ -z "${LANG_NAME:-}" ]; then
		LANG_NAME="$1"
		shift
	fi

	#> Print initial info if verbosity is at least info level
	print_debug "Project root: ${PRJ_ROOT}"
	print_debug "Source root: ${SRC_ROOT}"
	print_debug "Templates: ${TEMPLATES}"
}

validate_arguments() {
	#? Language
	if [ -n "${LANG_NAME:-}" ]; then
		if [ "${VERBOSITY_LEVEL}" -ge 3 ]; then
			printf "Selected language: %s\n" "${LANG_NAME}"

			#? Language Directory
			LANG_ROOT="${SRC_ROOT}/${LANG_NAME}"
			if ! [ -d "${LANG_ROOT}" ]; then
				print_error "Source directory not found: src/${LANG_NAME}"
			fi
		fi
	else
		if [ "${VERBOSITY_LEVEL}" -ge 3 ]; then
			print_info "No language specified, using default environment\n"
		fi
		return 0
	fi
}

initialize_environment() {
	#> Make all shell scripts in templates executable
	find "${TEMPLATES}" -name "*.sh" -type f -exec chmod +x {} \;

	#> Detect and use best available method
	#? Try nix first
	use_nix "${LANG_NAME:-}"
	nix_result=$?
	if [ "${nix_result}" -eq 0 ]; then
		return 0
	fi

	#? Fall back to mise
	use_mise "${LANG_ROOT:-"${PRJ_ROOT:-}"}"
}

#╔═══════════════════════════════════════════════════════════╗
#║ Utilities                                                 ║
#╚═══════════════════════════════════════════════════════════╝

parse_verbosity() {
	#> Parse verbosity from various formats
	#? Verbosity levels (standardized)
	#? 0: quiet (errors only)
	#? 1: error
	#? 2: warning
	#? 3: info (default)
	#? 4: debug
	#? 5: trace
	case "${1:-}" in
	0 | q | quiet | silent | off | false | no) printf 0 ;;
	1 | e | error | err) printf 1 ;;
	2 | w | warn | warning) printf 2 ;;
	3 | i | info | true | on | yes) printf 3 ;;
	4 | d | debug) printf 4 ;;
	5 | t | trace | all | verbose) printf 5 ;;
	*) printf 3 ;;
	esac
}

print_error() {
	#> Unified error handling function
	#? Combines functionality of old printe() and print_error()
	#? Options:
	#?   -p, --parse-arg ARG  Print "ARG requires an argument" error
	#?   -c, --continue       Return instead of exiting
	#?   -x, --code CODE      Set exit/return code (default: 1)

	keep_alive=0
	code=1
	message=""

	#> Process options
	while [ $# -gt 0 ]; do
		case "$1" in
		-p | --parse-arg)
			if [ -n "${2:-}" ]; then
				message="$2 requires an argument"
				shift
			else
				code=2
				message="$1 requires an argument. [internal error]"
			fi
			;;
		-c | --continue)
			keep_alive=1
			;;
		-x | --code)
			if [ -n "${2:-}" ]; then
				code="$2"
				shift
			else
				message="$1 requires an argument"
			fi
			;;
		*)
			#? First non-option argument is the message
			if [ -z "${message}" ]; then
				message="$1"
			fi
			;;
		esac
		shift
	done

	#> Print error message if provided, otherwise print generic error
	if [ "${VERBOSITY_LEVEL}" -ne 0 ]; then
		if [ -n "${message}" ]; then
			printf "[ERROR]  %s\n" "${message}" >&2
		else
			printf "[ERROR] unknown error\n" >&2
		fi
	fi

	#> Handle exit/return based on keep_alive flag
	if [ "${keep_alive}" -eq 1 ]; then
		return "${code}"
	else
		exit "${code}"
	fi
}

print_warn() {
	if [ "${VERBOSITY_LEVEL}" -ge 2 ]; then
		printf "[WARN]  %s\n" "$*" >&2
	fi
}

print_info() {
	if [ "${VERBOSITY_LEVEL}" -ge 3 ]; then
		printf "[INFO]  %s\n" "$*"
	fi
}

print_debug() {
	if [ "${VERBOSITY_LEVEL}" -ge 4 ]; then
		printf "[DEBUG] %s\n" "$*"
	fi
}

print_trace() {
	if [ "${VERBOSITY_LEVEL}" -ge 5 ]; then
		printf "[TRACE] %s\n" "$*"
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Runners                                                   ║
#╚═══════════════════════════════════════════════════════════╝

use_nix() {
	# Check if nix is available
	_has_nix=false
	if command -v nix >/dev/null 2>&1; then
		_has_nix=true
		nix_version="$(
			nix --version | cut -d' ' -f3 | cut -d'+' -f1
		)"
	fi

	if [ "${_has_nix}" = true ]; then
		#> Show nix version at debug level or higher
		print_info "✓ Nix (${nix_version}) detected"
	else
		print_debug "Nix not found"
		return 1
	fi

	nix_msg() {
		case "$1" in
		--no-lang)
			print_info "No language specified, using default ${2} environment"
			;;
		--de*)
			print_info "Entering development environment for ${2}"
			;;
		*)
			print_info "Entering development environment"
			;;
		esac
	}

	#> Check for flakes support
	_has_flakes=false
	if nix flake --version >/dev/null 2>&1; then
		_has_flakes=true
	fi

	if [ "${_has_flakes}" = true ]; then
		nix_msg "flake"
		case "${1:-}" in
		"")
			nix_msg --no-lang "flake"
			exec nix develop
			;;
		*)
			exec nix develop ".#${1}"
			;;
		esac
	else
		nix_msg "shell"
		case "${1:-}" in
		"")
			nix_msg --no-lang "shell"
			exec nix-shell
			;;
		*)
			exec nix-shell "$1"
			;;
		esac
	fi
}

use_mise() {
	#> Check for mise
	_has_mise=false
	if command -v mise >/dev/null 2>&1; then
		_has_mise=true
		mise_version="$(
			mise --version 2>/dev/null | tail -n 1 | cut -d' ' -f1
		)"
	fi

	if [ "${_has_mise}" = true ]; then
		#> Show mise version at debug level or higher
		print_debug "✓ mise (${mise_version}) detected"
		print_info "Initializing with mise..."
	else
		print_warn "mise is not installed"
		print_info "Install from: https://mise.jdx.dev/getting-started.html"
		print_info ""
		print_info "Quick install:"
		print_info "  curl https://mise.run | sh"
		print_info "  # or on Windows:"
		print_info "  winget install jdx.mise"
		exit 1
	fi

	#> Run the mise initialization script
	if [ -f "${TEMPLATES}/mise-init.sh" ]; then
		sh "${TEMPLATES}/mise-init.sh"
	else
		print_error "templates/init-project.sh not found"
		exit 1
	fi

	print_info "To activate mise:"
	print_info "  eval \"\$(mise activate bash)\"  # or zsh, fish, etc."

	if [ -d "${1}" ]; then
		print_info ""
		print_info "Then navigate to: cd ${1}"
		print_info "And run: mise trust"
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Main Entry Point                                          ║
#╚═══════════════════════════════════════════════════════════╝

main() {
	initialize_defaults
	parse_arguments "$@"
	validate_arguments
	initialize_environment
}

main "$@"
