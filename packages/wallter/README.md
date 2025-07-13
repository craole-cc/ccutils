# Wallter: Redefine Your Desktop with Dynamic Wallpaper Alteration

Unlock the full potential of your desktop with **Wallter**, an innovative application designed to empower you to **alter** your wallpapers dynamically. This includes future capabilities to apply direct image changes, such as grayscale, mosaic effects, and more, before setting the wallpaper. Starting as a robust command-line tool, **Wallter** is set to expand its reach with future graphical (GUI) and web-based interfaces, providing a seamless and highly personalized environment on any platform.

---

## Motivation

**Wallter** was born out of a desire for a truly **cross-platform, open-source, highly customizable, and automated** wallpaper management solution. Existing tools often fall short in specific environments, are proprietary, or lack the flexibility needed for power users:

- **[Variety](https://github.com/varietywalls/variety) (Linux):** While a great tool, it often struggles with Wayland, limiting its utility for users on modern Linux desktops.
- **[John's Background Switcher](https://johnsad.ventures/software/backgroundswitcher/) (Windows/Mac):** A popular **proprietary** choice, its lack of a command-line interface and limited automation capabilities hinder advanced customization and integration into automated workflows.

**Wallter** aims to fill these critical gaps by providing:

- **A Fully Open-Source Solution:** Providing transparency, fostering community contribution, and offering ultimate user control over their wallpaper management.
- **Native Cross-Platform Support:** Built with Rust, `wallter` is designed from the ground up to work seamlessly across Windows, macOS, and Linux (including Wayland environments), ensuring a consistent and unified experience regardless of your operating system.
- **Automation at its Core:** Its CLI-first design allows for easy integration into scripts, cron jobs, and custom automation routines, giving you unprecedented control.
- **The Power of Rust:** Leveraging Rust's performance, safety, and robust ecosystem allows for a reliable and efficient application, while also serving as a personal project to expand a Rust-based `dotfiles` ecosystem.

---

## Features

- **Dynamic Wallpaper Sourcing:** Seamlessly downloads and sets high-quality wallpapers from **multiple online sources** (e.g., Wallhaven, Unsplash, Pixabay), with the flexibility to **add your own custom sources**. Wallpaper shuffling can intelligently pull from any configured source.
- **Local Image Integration:** Utilize your personal image collections by designating custom directories for wallpaper selection.
- **Intelligent Multi-Monitor Support:** Optimizes wallpaper display across diverse monitor setups, intelligently adapting to different resolutions, orientations, and positions.
- **Automated Workflow Customization:** Execute custom commands both before and after setting wallpapers, allowing for personalized automation of your desktop environment.
- **Advanced Content Curation:** Fine-tune your wallpaper discovery with granular search parameters for supported APIs, ensuring you find exactly what you're looking for.
- **Smart Caching:** Efficiently stores downloaded wallpapers locally for faster access and offline use, enhancing performance for features like slideshows.
- **Wallpaper Alterations (Planned):** Future versions will include the ability to apply direct image changes, such as grayscale, mosaic effects, and adding captions/watermarks, before setting the wallpaper.

---

## Installation

### From Source

1. **Clone the repository:**

   ```bash
   git clone [https://github.com/craole-cc/wallter.git](https://github.com/craole-cc/wallter.git)
   ```

2. **Navigate to the project directory:**

   ```bash
   cd wallter
   ```

3. **Build the project:**

   ```bash
   cargo build --release
   ```

4. **Install the executable:**

   ```bash
   cargo install --path .
   ```

### From crates.io

1. **Install directly:**

   ```bash
   cargo install wallter
   ```

---

## Usage

**Wallter** provides intuitive commands for **altering** and managing your wallpapers.

- **Initialize Configuration:**

  ```bash
  wallter init
  ```

  _Creates a default configuration file, typically located within your `Pictures/Wallter` directory (`~/<Username>/Pictures/Wallter/config.toml`)._
- **Download Wallpapers:**

  ```bash
  wallter download
  ```

  _Fetches and saves new wallpapers from configured online sources to your local downloads directory._
- **Set Wallpaper:**

  ```bash
  wallter set
  ```

  _Applies a selected or random wallpaper to your desktop._
- **Start Slideshow:**

  ```bash
  wallter slideshow
  ```

  _Initiates a rotating display of wallpapers from your configured sources or directories._
- **Customize Configuration:**

  ```bash
  wallter config
  ```

  _Opens the main configuration file for manual adjustments and advanced setup._

---

## Configuration

The core of **Wallter**'s customization lies in its `config.toml` file. By default, **Wallter** sets up its primary directories within your user's `Pictures` directory for cross-platform simplicity and ease of access. For example, on Windows, this might be `C:\Users\<Username>\Pictures\Wallter\` _(with 'Username' representing your actual username)_, and on Linux, `~/Pictures/Wallter/`.

You can customize the following settings to tailor **Wallter** to your precise needs:

- `home_dir`: The root directory for **Wallter**'s operations, containing subdirectories for downloads, favorites, and current wallpapers.
- `api_key`: Your API key for integrated services (e.g., Wallhaven API key).
- `downloads_dir`: The designated local directory where **Wallter** will store downloaded wallpapers.
- `favorites_dir`: A dedicated directory for wallpapers you've marked as favorites.
- `wallpaper_dir`: The directory where the currently set wallpaper(s) are managed.
- `config_name`: The base name of the configuration file (e.g., "config").
- `config_type`: The format type of the configuration file (e.g., "toml", "json").
- `config_file`: The constructed full path to the configuration file.
- `custom_commands`: A list of shell commands to execute before and after a wallpaper is set, enabling dynamic actions.
- `monitors`: A detailed list defining each connected monitor by ID, name, resolution, orientation, scale, and position, allowing for precise multi-monitor control.
- `slideshow_interval`: The duration (as an integer) to wait between each image change during a slideshow.
- `slideshow_unit`: The unit of time for `slideshow_interval` (e.g., "seconds", "minutes", "hours").

---

## Contributing

We welcome contributions from the community to help make **Wallter** even better! Whether you're reporting a bug, suggesting a new feature, or submitting code, your input is highly valued.

Please see our comprehensive [Contributing Guide](../../CONTRIBUTING.md) for detailed instructions on how to get involved.

### Code of Conduct

All contributors are expected to adhere to **our** [Code of Conduct](../../CODE_OF_CONDUCT.md). Please read it carefully before participating.

---

## Project Development

Our development [Roadmap](ROADMAP.md) provides an in-depth look at current tasks and planned milestones for **Wallter**.

---

## License

This project is licensed under the MIT License. See the [License](../../LICENSE-MIT.md) file for more information.
