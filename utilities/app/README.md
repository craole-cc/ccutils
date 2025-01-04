# app

A command-line utility that simplifies package management across different package managers. Currently supports winget, with plans to expand to other package managers.

## Features

- **Simple Command Structure**: 
  - Default behavior is list available packages
  - Clear subcommands for common operations
- **Smart Package Manager Detection**: 
  - Automatically detects available package managers
  - OS-specific defaults:
    - Windows: winget, then chocolatey, then scoop
    - Linux: Distribution-aware defaults
      - Ubuntu/Debian: apt, snap, flatpak
      - Fedora/RHEL: dnf, snap, flatpak
      - Arch: pacman, paru, yay, flatpak
      - NixOS: nix-env, nix-shell -p
      - Universal: flatpak, snap (if available)
    - macOS: homebrew, mas
  - Override defaults via configuration or flags

- **Core Commands**:
  - `add`: Install packages
  - `remove`: Uninstall packages
  - `list`: View package lists
    - `installed`: Show installed packages
    - `available`: Show available packages from repositories (default)
    - `outdated`: Show packages with updates available
  - `search`: Alias of `app list available`

## Usage

```bash
# List available packages (default command)
app list available firefox
# or simply
app firefox

# Use specific package manager
app list available --winget firefox
app --winget firefox

# Install a package
app add firefox
app add --winget Rustlang.Rustup

# Install multiple packages
app add vscode git nodejs rust

# Install from a file
app add --file dev-tools.txt

# Remove a package
app remove firefox

# List commands
app list installed              # Show installed packages
app list installed --winget    # Show installed winget packages
app list available            # Show available packages
app list outdated            # Show packages with updates

# View detected package managers
app list managers             # Shows available package managers on your system
```

## Configuration

Create a configuration file at `~/.config/app/config.toml` to set:
- Package manager preferences and order
- Custom package sources
- Preferred installation options

Example configuration:
```toml
[package_managers]
# Override default order for Windows
windows = ["winget", "chocolatey", "scoop"]

# Linux distribution-specific defaults
debian = ["apt", "flatpak", "snap"]
ubuntu = ["apt", "flatpak", "snap"]
fedora = ["dnf", "flatpak", "snap"]
arch = ["paru", "yay", "pacman", "yaourt", "trizen", "flatpak"]
nixos = ["nix-shell", "nix-env", "flatpak"]
opensuse = ["zypper", "flatpak", "snap"]
centos = ["dnf", "flatpak", "snap"]
rhel = ["dnf", "flatpak", "snap"]

# macOS defaults
macos = ["brew", "mas"]

[sources]
# Add custom package sources
winget = ["winget-community", "msstore"]
chocolatey = ["chocolatey.org/packages"]
```

## Planned Features

- Support for additional package managers:
  - chocolatey (Windows)
  - scoop (Windows)
  - Distribution-specific package managers (Linux):
    - apt (Debian/Ubuntu)
    - dnf (Fedora/RHEL)
    - pacman/paru (Arch)
    - nix-env/nix-shell (NixOS)
  - brew (macOS)
- Package version management
- Installation history and logging
- Batch installation with dependency resolution
- Export/Import of installed packages
- System-wide vs user installation options

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License - See LICENSE file for details