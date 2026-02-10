#!/bin/sh
# shellcheck enable=all
#~@ Universal Project Orchestrator
#? Single source of truth for initializing any development environment
#? Works on: NixOS, Mac, WSL, Windows (Git Bash/Cygwin/MSYS2)
#? Called by: nix develop, nix-shell, mise, or directly

#╔═══════════════════════════════════════════════════════════╗
#║ Core Configuration                                        ║
#╚═══════════════════════════════════════════════════════════╝

detect_environment() {
	#> Get absolute project root
	if [ -d "${PRJ_ROOT:-}" ]; then
		#? Already set (e.g., by nix shell)
		:
	elif git rev-parse --show-toplevel >/dev/null 2>&1; then
		#> Use git if available to find project root
		PRJ_ROOT="$(git rev-parse --show-toplevel)"
	else
		#> Fallback for POSIX sh
		PRJ_ROOT="$(cd "$(dirname "$0")" && pwd)"
	fi

	#~@ Core directories
	PRJ_CODE="${PRJ_CODE:-"${PRJ_ROOT}/src"}"
	PRJ_TEMPLATES="${PRJ_TEMPLATES:-"${PRJ_ROOT}/templates"}"
	PRJ_SCRIPTS="${PRJ_SCRIPTS:-"${PRJ_ROOT}/scripts"}"

	#> Detect OS_TYPE
	case "$(uname -s | tr '[:upper:]' '[:lower:]')" in
	linux*)
		if nixos-version >/dev/null 2>&1; then
			OS_TYPE="nixos"
		else
			OS_TYPE="linux"
		fi
		;;
	darwin*) OS_TYPE="macos" ;;
	mingw* | msys* | cygwin*) OS_TYPE="windows" ;;
	*) OS_TYPE="unknown" ;;
	esac

	#> Detect if we're in Nix environment"${PRJ_SCRIPTS}"
	if [ -n "${IN_NIX_SHELL:-}" ] || [ -n "${NIX_DEVELOP:-}" ]; then
		ENV_TYPE="nix"
	elif command -v mise >/dev/null 2>&1; then
		ENV_TYPE="mise"
	else
		ENV_TYPE="bare"
	fi

	#> Export for child processes
	export PRJ_ROOT PRJ_CODE PRJ_TEMPLATES PRJ_SCRIPTS OS_TYPE ENV_TYPE
}

#╔═══════════════════════════════════════════════════════════╗
#║ Argument Parsing                                          ║
#╚═══════════════════════════════════════════════════════════╝

parse_arguments() {
	#~@ Defaults
	VERBOSITY_LEVEL=3 #? info
	deploy_from_templates=1
	OPEN_EDITOR=1
	LANG_NAME=""
	FORCE=0
	LANGUAGES="$(
		\find "${PRJ_TEMPLATES:-}" \
			-mindepth 1 -maxdepth 1 -type d -print0 2>/dev/null |
			xargs -0 -n 1 basename 2>/dev/null || true
	)"

	while [ $# -gt 0 ]; do
		case "$1" in
		#~@ Core actions
		--force)
			deploy_from_templates=1
			FORCE=1
			;;
		--deploy)
			deploy_from_templates=1
			FORCE=1
			;;
		--no-deploy)
			deploy_from_templates=0
			;;

		#~@ Editor control
		--editor | --edit | -e)
			OPEN_EDITOR=1
			;;
		--no-editor)
			OPEN_EDITOR=0
			;;

		#~@ Language selection
		-l | --lang*)
			if [ -n "${2:-}" ]; then
				LANG_NAME="$2"
				shift
			else
				print_error "$1 requires an argument"
			fi
			;;

		#~@ Verbosity
		-q | --quiet)
			VERBOSITY_LEVEL=0
			;;
		-v | --verbose)
			if [ "${VERBOSITY_LEVEL}" -lt 5 ]; then
				VERBOSITY_LEVEL="$((VERBOSITY_LEVEL + 1))"
			fi
			;;
		-t | -vvv | --trace)
			set -x
			VERBOSITY_LEVEL=5
			;;
		-d | -vv | --debug)
			VERBOSITY_LEVEL=4
			;;

		#~@ Help
		-h | --help)
			show_help
			exit 0
			;;
		--*) break ;;
		-*)
			print_error "Unknown option: $1"
			;;
		#~@ Positional argument (language shorthand)
		*)
			if [ -z "${LANG_NAME}" ]; then LANG_NAME="$1"; fi
			;;
		esac
		shift
	done
	#> Validate language if specified
	case "${LANG_NAME}" in "${LANGUAGES}") ;;
	*)
		print_error "$(
			printf "Unsupported language: %s\nSupported languages: %s" \
				"${LANG_NAME}" "${LANGUAGES}"
		)"
		;;
	esac

	#> Set language root, if specified
	if [ -n "${LANG_NAME}" ]; then
		LANG_ROOT="${PRJ_CODE}/${LANG_NAME}"
		export LANG_NAME LANG_ROOT
	fi

	#> Export settings
	export FORCE deploy_from_templates OPEN_EDITOR VERBOSITY_LEVEL
}

#╔═══════════════════════════════════════════════════════════╗
#║ Template Deployment System                                ║
#╚═══════════════════════════════════════════════════════════╝

deploy_from_templates() {
	print_debug "Deploying templates..."

	# #> Ensure directories exist
	# mkdir -p "${PRJ_TEMPLATES}" "${PRJ_CODE}" "${PRJ_SCRIPTS}" 2>/dev/null || true

	#> Deploy language-specific templates if language is specified
	if [ -n "${LANG_NAME}" ] && [ -d "${PRJ_TEMPLATES}/languages" ]; then
		deploy_language_templates "${LANG_NAME}"
	fi

	#> Make shellscripts executable
	\find "${PRJ_TEMPLATES:-}" "${PRJ_SCRIPTS:-}" \
		-name "*.sh" -type f -exec chmod +x {} \; 2>/dev/null || true
}

deploy_template() {
	template_path="$1"
	config_path="$1"
	content="$2"
	# target_path="${PRJ_ROOT}/${filename}"

	# Deploy template, if necessary
	if [ "${FORCE}" -eq 1 ] || [ ! -f "${config_path}" ]; then
		print_debug "Creating ${config_path}"
		printf '%s' "${content}" >"${config_path}"
	else
		print_debug "Skipping ${config_path} (already exists)"
	fi
}

deploy_language_templates() {
	lang="$1"
	lang_template_dir="${PRJ_TEMPLATES}/${lang}"

	if [ ! -d "${lang_template_dir}" ]; then
		print_warn "No templates for language: ${lang}"
		return 0
	fi

	# Create language directory
	mkdir -p "${PRJ_CODE}/${lang}"

	# Deploy all templates from language directory
	if [ -d "${lang_template_dir}" ]; then
		msg "Deploying ${lang} templates..." "info"

		# Use find to copy all files, preserving structure
		(cd "${lang_template_dir}" && find . -type f) | while read -r template; do
			src="${lang_template_dir}/${template}"
			dest="${PRJ_CODE}/${lang}/${template}"

			if [ "${FORCE}" -eq 1 ] || [ ! -f "${dest}" ]; then
				mkdir -p "$(dirname "${dest}")"
				cp "${src}" "${dest}"
				msg "  → ${template}" "debug"
			fi
		done
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Environment Initialization                                ║
#╚═══════════════════════════════════════════════════════════╝

initialize_environment() {
	print_info "Initializing ${ENV_TYPE} environment..."

	#> Deploy templates if requested
	if [ "${deploy_from_templates}" -eq 1 ]; then
		deploy_from_templates
	fi

	#> Setup based on environment type
	case "${ENV_TYPE}" in
	nix) setup_nix_environment ;;
	mise) setup_mise_environment ;;
	bare | *) setup_bare_environment ;;
	esac

	#> Open editor if requested
	if [ "${OPEN_EDITOR}" -eq 1 ]; then
		open_editor
	fi
}

setup_nix_environment() {
	case "${ENV_TYPE}" in
	nix)
		print_debug "Setting up Nix environment..."
		;;
	*)
		print_debug "Not in Nix environment, cannot setup Nix"
		return
		;;
	esac

	#> Check if we're already in a nix shell
	if [ -n "${IN_NIX_SHELL:-}" ]; then
		print_debug "Already in Nix shell"

		#> Run the root shell hook
		if [ -f "${PRJ_TEMPLATES}/shellhook-root.sh" ]; then
			# shellcheck disable=SC1091
			. "${PRJ_TEMPLATES}/shellhook-root.sh"
		fi

		#> Language-specific setup
		if [ -n "${LANG_NAME}" ] && [ -f "${PRJ_CODE}/${LANG_NAME}/shell.nix" ]; then
			msg "Entering ${LANG_NAME} environment..." "info"
			cd "${PRJ_CODE}/${LANG_NAME}"
		fi
	else
		#> Start nix develop
		if command -v nix >/dev/null 2>&1; then
			if nix flake --version >/dev/null 2>&1; then
				msg "Starting nix develop (flakes)..." "info"
				if [ -n "${LANG_NAME}" ]; then
					exec nix develop ".#${LANG_NAME}" || exec nix develop
				else
					exec nix develop
				fi
			else
				msg "Starting nix-shell..." "info"
				if [ -n "${LANG_NAME}" ] && [ -f "${PRJ_CODE}/${LANG_NAME}/shell.nix" ]; then
					exec nix-shell "${PRJ_CODE}/${LANG_NAME}/shell.nix"
				else
					exec nix-shell
				fi
			fi
		fi
	fi
}

setup_mise_environment() {
	pring_debug "Setting up mise environment..."

	#> Ensure mise is installed
	if ! command -v mise >/dev/null 2>&1; then
		print_info "Installing mise..."
		error_msg="Please install mise manually: https://mise.jdx.dev"

		case "${OS_TYPE}" in
		linux | macos | nixos)
			curl https://mise.run | sh
			;;
		windows)
			#> Try winget, then scoop, then chocolatey
			if command -v winget >/dev/null 2>&1; then
				winget install jdx.mise
			elif command -v scoop >/dev/null 2>&1; then
				scoop install mise
			elif command -v choco >/dev/null 2>&1; then
				choco install mise
			else print_error "${error_msg}"; fi
			;;
		*) print_error "${error_msg}" ;;
		esac
	fi

	#> Setup mise in current directory
	if {
		[ -f "${PRJ_ROOT}/.mise.toml" ] ||
			[ -f "${PRJ_ROOT}/mise.toml" ]
	} && [ "${FORCE}" -eq 0 ]; then
		:
	elif [ -f "${PRJ_TEMPLATES}/mise-root.toml" ]; then
		cp "${PRJ_TEMPLATES}/mise-root.toml" .mise.toml
	fi

	#> Trust mise config
	mise trust 2>/dev/null || true

	#> Install tools
	msg "Installing mise tools..." "info"
	mise install 2>/dev/null || true

	#> Activate mise
	eval "$(mise activate bash 2>/dev/null)" || true

	# Enter language directory
	if [ -n "${LANG_NAME}" ]; then
		if [ -d "${PRJ_CODE}/${LANG_NAME}" ]; then
			cd "${PRJ_CODE}/${LANG_NAME}"
			if [ -f .mise.toml ]; then
				mise trust 2>/dev/null || true
				mise install 2>/dev/null || true
			fi
		fi
	fi

	# Keep shell open
	msg "Type 'exit' to leave the environment" "info"
	exec "${SHELL:-bash}"
}

setup_bare_environment() {
	msg "Setting up basic environment..." "warn"
	msg "Consider installing mise or nix for better tool management" "info"

	# Just setup directories
	mkdir -p "${PRJ_CODE}" "${PRJ_TEMPLATES}" "${PRJ_SCRIPTS}" 2>/dev/null || true

	if [ -n "${LANG_NAME}" ]; then
		mkdir -p "${PRJ_CODE}/${LANG_NAME}"
		cd "${PRJ_CODE}/${LANG_NAME}"
	fi
}

#╔═══════════════════════════════════════════════════════════╗
#║ Editor Integration                                        ║
#╚═══════════════════════════════════════════════════════════╝

open_editor() {
	#> Don't open editor if we're not in a terminal
	[ -t 0 ] || return 0

	#> Don't open editor if we're already in one
	case "${TERM_PROGRAM:-}" in
	vscode | *code* | *Code*)
		print_debug "Already in VSCode"
		return 0
		;;
	*) ;;
	esac

	#> Check for common editor environment variables
	editor=""
	for ed in "${VISUAL:-}" "${EDITOR:-}" "code-insiders" "code" "zededitor" "nvim" "vim" "nano"; do
		if command -v "${ed}" >/dev/null 2>&1; then
			editor="${ed}"
			break
		fi
	done

	if [ -n "${editor}" ]; then
		print_info "Opening editor: ${editor}"

		#> Determine what to open
		target="."
		if [ -n "${LANG_NAME}" ] && [ -d "${PRJ_CODE}/${LANG_NAME}" ]; then
			target="${PRJ_CODE}/${LANG_NAME}"
		fi

		#> Open editor (background if GUI)
		case "${editor}" in
		*code*) "${editor}" "${target}" & ;;
		*) "${editor}" "${target}" ;;
		esac
	fi
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
	if [ "${VERBOSITY_LEVEL:-0}" -ne 0 ]; then
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
	if [ "${VERBOSITY_LEVEL:-0}" -ge 2 ]; then
		printf "[WARN]  %s\n" "$*" >&2
	fi
}

print_info() {
	if [ "${VERBOSITY_LEVEL:-0}" -ge 3 ]; then
		printf "[INFO]  %s\n" "$*"
	fi
}

print_debug() {
	if [ "${VERBOSITY_LEVEL:-0}" -ge 4 ]; then
		printf "[DEBUG] %s\n" "$*"
	fi
}

print_trace() {
	if [ "${VERBOSITY_LEVEL:-0}" -ge 5 ]; then
		printf "[TRACE] %s\n" "$*"
	fi
}

msg() {
	message="$1"
	level="${2:-info}"

	#> Check if we should output based on verbosity
	case "${level}" in
	error)
		if [ "${VERBOSITY_LEVEL}" -ge 1 ]; then
			color="31"
			prefix="[ERROR]"
		fi
		;;
	warn)
		if [ "${VERBOSITY_LEVEL}" -ge 2 ]; then
			color="33"
			prefix="[WARN] "
		fi
		;;
	info)
		if [ "${VERBOSITY_LEVEL}" -ge 3 ]; then
			color="36"
			prefix="[INFO] "
		fi
		;;
	debug)
		if [ "${VERBOSITY_LEVEL}" -ge 4 ]; then
			color="90"
			prefix="[DEBUG]"
		fi
		;;
	trace)
		if [ "${VERBOSITY_LEVEL}" -ge 4 ]; then
			color="90"
			prefix="[TRACE]"
		fi
		;;
	success)
		if [ "${VERBOSITY_LEVEL}" -ge 3 ]; then
			color="32"
			prefix="[INFO] "
		fi
		;;
	*)
		color=""
		prefix=""
		;;
	esac

	#> Output message if level matches verbosity
	if [ -n "${color}" ]; then
		printf "\033[0;%sm%s\033[0m %s\n" "${color}" "${prefix}" "${message}" >&2
	fi
}

show_help() {
	cat <<EOF
Universal Project Orchestrator - init.sh

USAGE:
  ./init.sh [OPTIONS] [language]
  nix develop    # calls ./init.sh --deploy
  mise run init  # calls ./init.sh --deploy --editor

ARGUMENTS:
  [language]     Language to initialize (rust, nix, python, shellscript, etc.)

OPTIONS:
  --deploy       Force deploy all templates (overwrites existing)
  --no-deploy    Skip template deployment
  --editor       Open editor after initialization
  --no-editor    Don't open editor
  -l, --language NAME  Explicit language selection
  -v, --verbose  Increase verbosity (can repeat: -vv for max)
  -q, --quiet    Quiet mode (errors only)
  -d, --debug    Debug output
  -t, --trace    Trace execution
  -h, --help     Show this help

EXAMPLES:
  # Initialize with Nix (preferred)
  nix develop

  # Initialize with mise
  mise run init

  # Direct usage
  ./init.sh rust --editor
  ./init.sh --deploy --no-editor
  ./init.sh -vv nix

ENVIRONMENT:
  PRJ_ROOT     Project root directory (auto-detected)
  PRJ_CODE     Source directory (\$PRJ_ROOT/src)
  PRJ_TEMPLATES    Templates directory (\$PRJ_ROOT/templates)
  LANG_NAME    Language to initialize
  LANG_ROOT    Language directory (\$PRJ_CODE/\$LANG_NAME)
EOF
}

#╔═══════════════════════════════════════════════════════════╗
#║ Main Entry Point                                          ║
#╚═══════════════════════════════════════════════════════════╝

main() {
	detect_environment
	parse_arguments "$@"
	# initialize_environment
}

#> Only run main if script is executed directly
if [ "${0##*/}" = "init.sh" ]; then
	echo "we here"
	main "$@"
else
	set -eu
	echo "sourced"
	main "$@"
fi

#╔═══════════════════════════════════════════════════════════╗
#║ Shell.nix Integration                                     ║
#╚═══════════════════════════════════════════════════════════╝

# This script can be sourced from shell.nix
# shell.nix should look like:
# { pkgs ? import <nixpkgs> {} }:
# pkgs.mkShell {
#   buildInputs = with pkgs; [ /* your packages */ ];
#   shellHook = ''
#     export PRJ_ROOT="$(pwd)"
#     # Run init.sh in deployment mode
#     sh "$PRJ_ROOT/init.sh" --deploy
#   '';
# }
