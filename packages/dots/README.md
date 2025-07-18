# Dots

The purpose of this is to use rust to improve the portability of my dotsrc which essentially is meant to achieve certain things.

- Reset the environment
- Establish the script environment variables
- Ensure DOTS variables are set
- Conditionally add scripts to PATH
- Set the interactive shell (starship, oh-my-posh, etc)
- Update the shell rc files available on the system (powershell, pswh, zsh, bash, fish, nushell, etc)

```sh .dotsrc
#!/bin/sh

main() {
  #@ Reset the environment
  cleanup

  #@ Establish the script environment variables
  set_flags

  #@ Ensure DOTS variables are set
  set_dots_paths \
    --rc ".dotsrc" \
    --ignore ".ignore" ||
    return 1

  #@ Set the interactive shell
  set_interactive_shell bash

  #@ Load DOTS
  initialize_dots
}

set_dots_paths() {
  while [ "$#" -gt 0 ]; do
    case "$1" in
    --rc) rc="$2" ;;
    --ignore) ignore="$2" ;;
    --dots) dots="$2" ;;
    esac
    shift
  done

  #@ Set the path to the DOTS directory based in argument
  if [ "$dots" ]; then
    DOTS="$dots"
  else
    if [ -f "$HOME/$rc" ]; then
      DOTS="$HOME"
    elif [ -f "$PWD/$rc" ]; then
      DOTS="$PWD"
    elif [ -f "$HOME/Documents/dotfiles/$rc" ]; then
      DOTS="$HOME/Documents/dotfiles"
    elif [ -f "$HOME/.dots/$rc" ]; then
      DOTS="$HOME/.dots"
    elif [ -f "$HOME/dots/$rc" ]; then
      DOTS="$HOME/dots"
    elif [ -f "$HOME/.dotfiles/$rc" ]; then
      DOTS="$HOME/.dotfiles"
    elif [ -f "$HOME/.config/dotfiles/$rc" ]; then
      DOTS="$HOME/.config/dotfiles"
    elif [ -f "$HOME/.config/dots/$rc" ]; then
      DOTS="$HOME/.config/dots"
    elif [ -f "/dots/$rc" ]; then
      DOTS="/dots"
    else
      DOTS="$(cd "$(dirname "$0")" 2>/dev/null && pwd)" || return 1
    fi
  fi

  if [ -d "$DOTS" ]; then
    #@ Set the DOTS variables
    DOTS_RC="${DOTS_RC:-"${DOTS}/${rc}"}"

    #@ Print the DOTS variable
    case "$VERBOSITY" in
    0) ;;
    1) printf "DOTS: %s\n" "$DOTS" ;;
    *)
      if [ "$dots" ]; then
        printf "DOTS set relative to '%s': %s\n" "$DOTS" "$rc"
      else
        printf "DOTS set to: %s\n" "$DOTS"
      fi
      ;;
    esac
  else
    printf "%s\n" "Unable to determine the DOTS directory"
    return 1
  fi

  #@ Set the DOTS_IGNORE variable
  if [ -f "$ignore" ]; then
    DOTS_IGNORE="$(pwd)/$ignore"
  elif [ "$ignore" ]; then
    DOTS_IGNORE="$DOTS/$ignore"
  else
    DOTS_IGNORE="$DOTS/.ignore"
  fi

  #@ Create the DOTS_IGNORE file, if necessary
  [ -f "$DOTS_IGNORE" ] || touch "$DOTS_IGNORE"

  export DOTS DOTS_RC DOTS_IGNORE
}

set_verbosity_level() {
  #@ Check for global verbosity settings
  # shellcheck disable=SC2153
  VERBOSITY="$verbose$verbosity$VERBOSE$VERBOSITY"

  # [ "$VERBOSITY" ] || VERBOSITY="$1"

  case "$VERBOSITY" in
  [0-9] | [1-9][0-9] | [1-9][0-9][0-9]) ;;
  false | 0 | '')
    VERBOSITY=0
    ;;
  true | *)
    VERBOSITY=1
    ;;
  esac

  #@ Make variables available globally
  export VERBOSITY

  [ "$VERBOSITY" -gt 0 ] && verbose_flag=true
  [ "$VERBOSITY" -gt 1 ] && {
    printf "Verbosity level set to %s\n" "$VERBOSITY"
  }
}

set_interactive_shell() {
  #@ Set the preferred interactive shell prompt
  SHELL_INTERACTIVE="${1:-bash}"

  #@ Ensure the variable is available globally
  export SHELL_INTERACTIVE
}

get_os_type() {
  if [ "$WSL_DISTRO_NAME" ]; then
    os_type="Windows Subsystem for Linux [WSL]"
  elif [ -f "/proc/version" ]; then
    os_type=$(cat "/proc/version")
  elif command -v uname >/dev/null 2>&1; then
    os_type="$(uname --kernel-name)"
  elif command -v python >/dev/null 2>&1; then
    os_type="$(
      python -c 'import platform; print(platform.system())'
    )"
  elif command -v hostnamectl >/dev/null 2>&1; then
    os_type="$(
      hostnamectl | awk -F ': ' '/Kernel/ {print $2}'
    )"
  fi

  case "$(printf "%s" "$os_type" | tr '[:upper:]' '[:lower:]')" in
  *linux* | *gnu*)
    os_type="GNU/Linux"
    ;;
  *wsl* | *microsoft*)
    os_type="Windows Subsystem for Linux [WSL]"
    ;;
  *cygwin* | *msys* | *mingw* | *windows*)
    os_type="Windows"
    # This line is needed to avoid a warning from Nix when using WSL.
    # See https://github.com/microsoft/WSL/issues/1936
    # and https://nixos.wiki/wiki/FAQ#How_can_I_avoid_the_.22winsymlinks.22_warning_when_installing_Nix_on_WSL.3F
    # for more information.
    MSYS=winsymlinks:nativestrict
    ;;
  *darwin*)
    os_type="Mac"
    ;;
  *freebsd*)
    os_type="FreeBSD"
    ;;
  *netbsd*)
    os_type="NetBSD"
    ;;
  *openbsd*)
    os_type="OpenBSD"
    ;;
  *hp*)
    os_type="HP"
    ;;
  *solaris* | *sunos*)
    os_type="Solaris"
    ;;
  *aix*)
    os_type="AIX"
    ;;
  *irix*)
    os_type="IRIX"
    ;;
  esac

  printf "%s" "$os_type"
}

delete_line_from_file() {
  sed --in-place --expression "/$1/d" "$profile"
}

update_user_profile() {
  #@ Define the profile path
  profile="$HOME/.profile"

  #@ Create the profile file if it doesn't exist
  [ -f "$profile" ] || touch "$profile"

  #@ Define the lines with and without quotes
  dots_unquoted="DOTS=$DOTS"
  dots_quoted="DOTS=\"$DOTS\""
  dots_init="[ -f \"\$DOTS/.dotsrc\" ] && . \"\$DOTS/.dotsrc\""

  #@ Check if the DOTS line exists and if it's different or if dots_init is missing
  if grep --quiet --regexp "^DOTS=" "$profile"; then
    {
      #@ Check if the DOTS line matches the current DOTS variable or is quoted
      grep --quiet --regexp "^DOTS=$DOTS" --regexp "^DOTS=\"$DOTS\"" "$profile" ||

        #@ Check if dots_init is missing
        grep --quiet --fixed-strings "$dots_init" "$profile"
    } ||
      {
        #@ If either condition is false, remove both lines
        delete_line_from_file "^DOTS="
        delete_line_from_file "$(printf "%s" "$dots_init" | sed 's/[\/&]/\\&/g')"
      }
  fi

  #@ Append the new lines to the profile file only if they are missing
  if
    ! grep --quiet --regexp "^DOTS=" "$profile" ||
      ! grep --quiet --fixed-strings "$dots_init" "$profile"
  then
    #@ If either line is missing, append both lines
    temp_file=$(mktemp)
    grep --invert-match "^DOTS=" "$profile" |
      grep --invert-match "$dots_init" >"$temp_file"
    printf "\nDOTS=\"%s\"\n%s\n" "$DOTS" "$dots_init" >>"$temp_file"
    mv "$temp_file" "$profile"
    [ "$verbose_flag" ] && printf "Updated DOTS in profile.\n"
  else
    #@ If both lines are present and the DOTS line is the same, no need to update
    [ "$verbose_flag" ] && printf "DOTS in profile is already up to date.\n"
  fi
}

set_flags() {
  [ "$(get_os_type)" = "Windows" ] && windows_flag=true

  set_verbosity_level
}

print_usage_guide() {
  printf "%s\n" "$usage_guide"
  exit "$exit_code"
}

get_sources() {
  prepare_ignore_list() { #? Update .ignore file
    if [ "$ignore_action" = "EXCLUDE" ]; then
      grep --line-regexp --quiet "$1" "$DOTS_IGNORE" ||
        printf "\n%s" "$1" >>"$DOTS_IGNORE"
    elif [ "$ignore_action" = "INCLUDE" ]; then
      sed --in-place "/$1/d" "$DOTS_IGNORE"
    else
      return 0
    fi
  }

  exclude_source() {
    #? Remove blank lines
    sed -i '/^[[:space:]]*$/d' "$DOTS_IGNORE"

    #? Sort alphabetically
    sort --human-numeric-sort --output "$DOTS_IGNORE" "$DOTS_IGNORE"

    #? Prep for use with `grep`
    sed -e 's/^/\//g' "$DOTS_IGNORE" |
      tr '\n' '|' | sed '$s/|$/\n/'
  }

  include_source() {
    if [ "$(exclude_source)" ]; then
      grep \
        --extended-regexp \
        --invert-match \
        --ignore-case \
        "$(exclude_source)" |
        sort
    else
      sort
    fi
  }

  #? Identify valid sources
  generate_sources() {
    find "$1" | include_source
  }

  if [ -e "$1" ]; then
    generate_sources "$1"
  else
    prepare_ignore_list "$1"
  fi
}

process_sources() {
  #@ Enable global variable export
  set -o allexport

  #@ Process sources recursively
  for src_path in $(get_sources "$1"); do
    [ -d "$src_path" ] && [ "$src_type" = "XDG" ] &&
      case ":${XDG_DATA_DIRS}:" in
      *:"$src_path":*) ;;
      *)
        XDG_DATA_DIRS="${src_path}${XDG_DATA_DIRS:+:${XDG_DATA_DIRS}}"
        [ "$verbose_flag" ] && printf "Appended to XDG_DATA_DIRS: %s\n" "${src_path}"
        ;;
      esac

    #@ Update PATH directories
    [ -d "$src_path" ] && [ "$src_type" = "BIN" ] &&
      case ":${PATH}:" in
      *:"$src_path":*) ;;
      *)
        PATH="${PATH:+$PATH:}$src_path"
        [ "$verbose_flag" ] && printf "Appended to PATH: %s\n" "${src_path}"
        ;;
      esac

    #@ Activate Scripts and Variables
    [ -f "$src_path" ] &&

      #| Make scripts executable
      if [ "$src_type" = "BIN" ]; then
        if [ "$verbose_flag" ]; then
          [ "$windows_flag" ] || chmod --changes +x "$src_path"
        else
          [ "$windows_flag" ] || chmod +x "$src_path"
        fi

      #@ Load environmental variables from files
      elif [ "$src_type" = "ENV" ]; then
        # shellcheck disable=SC1090
        # EOLor --lf "$src_path"
        . "$src_path"
        [ "$verbose_flag" ] && printf "Initialized: %s\n" "$src_path"
      fi

  done

  #@ Disable global export
  set +o allexport
}

initialize_source() {
  #| Core Arguments
  case "$1" in
  -h | --help)
    exit_code=0
    print_usage_guide
    ;;
  -v | --version)
    printf "%s\n" "$version"
    exit 0
    ;;
  -d | --verbose)
    verbose_flag=true
    shift
    ;;
  -t | --simulate | --test)
    test_flag=true
    shift
    ;;
  -q | --quiet)
    unset verbose_flag
    shift
    ;;
  --ignore-file)
    ignore_file="$2"
    shift 2
    ;;
  *) ;;
  esac

  #| Process Arguments
  while [ "$#" -ge 1 ]; do
    case "$1" in
    --bin) #| Expects a file/directory
      src_type="BIN"
      shift
      ;;
    --env) #| Expects a file/directory
      src_type="ENV"
      shift
      ;;
    --xdg) #| Expects a directory
      src_type="XDG"
      shift
      ;;
    --exclude) #| Expects a string
      ignore_action="EXCLUDE"
      shift
      ;;
    -I | --include) #| Expects a string
      ignore_action="INCLUDE"
      shift
      ;;
    -*)
      printf "Invalid Option: %s\n" "$1"
      exit_code=1
      print_usage_guide
      ;;
    *) ;;
    esac

    if [ "$test_flag" = true ]; then
      simulate_initialization "$1"
    else
      process_sources "$1"
    fi

    shift
  done
}

simulate_initialization() {
  # [ "$src_type" ] && printf "\n%s: %s\n" "$src_type" "$1"
  # [ "$ignore_action" ] && printf "%s: %s\n" "$ignore_action" "$1"
  get_sources "$1"
}

cleanup() {
  [ "$reset_flag" = true ] &&
    rm -rf "$DOTS_IGNORE"

  unset -v \
    BIN_ \
    ENV_ \
    src_path \
    src_type \
    ignore_file \
    reset_flag \
    test_flag \
    verbose_flag
}

initialize_profile() {
  #@ Parse arguments
  while [ "$#" -gt 0 ]; do
    case "$1" in
    --file | --profile) profile="$2" ;;
    --key) key="$2" ;;
    --val) val="$2" ;;
    --rc) rc="$2" ;;
    esac
    shift
  done

  { [ "$key" ] && [ "$val" ]; } || return 1
  keyval="$key=$val"

  [ "$rc" ] && {
    key_rc="$(printf "%s_rc" "$key" | tr '[:lower:]' '[:upper:]')"
    val_rc="$val/$rc"
    initrc="$key_rc=\"\$$key/$rc\" && [ -f \"\$$key_rc\" ] && . \"\$$key_rc\""
  }

  grep --quiet --regexp "^$key=*" "$profile" || {
    # grep --quiet --regexp "$keyval" "$profile" ||
    # printf "\n%s" "$keyval" >>"$profile"

    # grep --quiet --regexp "$initrc" "$profile" ||
    # printf "\n%s" "$initrc" >>"$profile"

    printf "\n%s\n%s\n" "$keyval" "$initrc" >>"$profile"
  }

  delete_line_from_file() {
    sed --in-place --expression "/$1/d" "$2"
  }

  append_lines_to_file() {
    temp_file=$(mktemp)
    grep --invert-match "^$key=" "$file" |
      grep --invert-match "$initrc" >"$temp_file"
    printf "\n%s=\"%s\"\n%s\n" "$key" "$val" "$initrc" >>"$temp_file"
    mv "$temp_file" "$1"
    [ "$verbose_flag" ] && printf "Updated DOTS in %s.\n" "$1"
  }

  update_profile() {
    #@ Create the profile if it doesn't exist
    [ -f "$profile" ] || touch "$profile"

    #@ Check if the key/value declaration exists

    # grep --quiet \
    #   --regexp "^$key=$val" \
    #   --regexp "^$key=\"$val\"" "$profile" || {
    #   delete_line_from_file "^$key=" "$profile"
    #   # printf "\n%s=\"%s\"" "$key" "$val" >>"$profile"
    # }

    # [ "$rc" ] && {
    #   grep --quiet --fixed-strings "$initrc" "$profile" || {
    #     delete_line_from_file "^[ -f \"$key/$rc\" ]" "$profile"
    #   # printf "\n%s\n" "$initrc" >>"$profile"
    #   }
    # }

    # { &&
    #     grep --quiet --fixed-strings "$initrc" "$profile"
    # } || {
    #   #@ If either condition is false, remove both lines
    #   delete_line_from_file "$(printf "%s" "$initrc" | sed 's/[\/&]/\\&/g')" "$profile"
    #   printf "\n%s=\"%s\"\n%s\n" "$key" "$val" "$initrc" >>"$profile"
    # }

    #@ Check if the initrc declaration exists
    # printf "\n%s=\"%s\"" "$key" "$val" >>"$profile"
    # printf "\n%s\n" "$initrc" >>"$profile"

    #@ Check if the key declaration exists and if it's different
    # {
    #   grep --quiet \
    #     --regexp "^$key=$val" \
    #     --regexp "^$key=\"$val\"" "$profile" ||

    #     #@  Check if the rc initialization declaration exists
    #     grep --quiet --fixed-strings "$initrc" "$profile"
    # } || {
    #   #@ If either condition is false, remove both lines
    #   delete_line_from_file "^$key=" "$profile"
    #   delete_line_from_file "^[ -f \"$key/$rc\" ]*" "$profile"
    #   # delete_line_from_file "$(printf "%s" "$initrc" | sed 's/[\/&]/\\&/g')" "$profile"
    # }
    # delete_line_from_file "^[ -f \"$key/$rc\" ]" "$profile"

    # #@ Append the new lines to the file only if they are missing
    # if
    #   ! grep --quiet --regexp "^$key=" "$profile" ||
    #     ! grep --quiet --fixed-strings "$initrc" "$profile"
    # then
    #   #@ If either line is missing, append both lines
    #   # append_lines_to_file "$profile" "$initrc"
    # printf "\n%s=\"%s\"\n%s\n" \
    #   "$key" "$val" \
    #   "[ -f \"\$$key/$rc\" ] && . \"\$$key/$rc\"" \
    #   >>"$profile"
    # else
    #   #@ If both lines are present and the DOTS line is the same, no need to update
    #   [ "$verbose_flag" ] &&
    #     printf "%s in %s is already up to date.\n" "$key" "$profile"
    # fi

    # echo profiling
  }

  update_profile
  # echo profiler
}

initialize_shell() {
  # while [ "$#" -gt 0 ]; do
  #   case "$1" in
  #   --shell) shell="$2" ;;
  #   esac
  #   shift
  # done

  profile="$HOME/.profile"
  bashrc="$HOME/.bashrc"
  zshrc="$HOME/.zshrc"

  case "$SHELL_INTERACTIVE" in
  bash)
    grep --regexp "[ -f $profile ]*"
    # grep --quiet --regexp "[ -f $profile ]*" ||
    # printf "\n%s" "[ -f $profile ] && . $profile" >>"$bashrc"
    ;;
  esac
}

initialize_dots() {
  #@ Update the dotsrc launcher through .profile
  # initialize_profile \
  #   --file "$HOME/.profile" \
  #   --key "DOTS" \
  #   --val "$DOTS" \
  #   --rc ".dotsrc"

  # initialize_profile \
  #   --file "$HOME/.bashrc" \
  #   --key "BDOTDIR" \
  #   --val "$DOTS/Configuration/cli/bash" \
  #   --rc "config"

  #@ Load Environment
  initialize_source \
    --exclude "archive" "review" "template" "temp" "tmp" \
    --bin "$DOTS/Bin"
  # --env "$DOTS/Environment/export"

  #@ Load Shell
  # initialize_shell
  # shell.init "$SHELL_INTERACTIVE"

  #@ Load Fonts
  # fonts.init #TODO: This needs to be made portable to work on Windows

  #@ Load Additional Configs
  # DeployConfig git
  # DeployConfig zed
}

main "$@"
```
