# Contributing to CCUtils

We welcome and appreciate all contributions to the **ccutils** project! Whether
you're reporting a bug, suggesting an enhancement, improving documentation, or
submitting code, your help is invaluable. By contributing, you agree to abide by
our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

1. **Fork the Repository:** Start by forking the `ccutils` repository on GitHub.
2. **Clone Your Fork:** Clone your forked repository to your local machine:

   ```bash
   git clone https://github.com/craole-cc/ccutils.git
   cd ccutils
   ```

3. **Initialize Jujutsu (jj):** If you prefer using `jj` for version control (as
   the core developers do), initialize `jj` within your cloned Git repository:

   ```bash
   jj git init --colocate
   ```

   _This command sets up `jj` to work alongside Git in the same repository. You
   can use either tool, but we encourage using `jj` for local development due to
   its powerful history editing and collaboration features._

4. **Set up Development Environment:**
   - Ensure you have [Rust](https://www.rust-lang.org/tools/install) and Cargo
     installed.
   - Run `cargo build` to verify everything is set up correctly.

---

## How to Contribute

### 1. Reporting Bugs

- Before opening a new bug report, please check the
  [GitHub Issues page](https://github.com/craole-cc/ccutils/issues) to see if
  the issue has already been reported.
- If not, open a new issue.
- Provide a clear and concise description of the bug.
- Include steps to reproduce the issue.
- Mention your operating system, `ccutils` version, and Rust version.
- Attach any relevant error messages, logs, or screenshots.

### 2. Suggesting Enhancements / New Features

- Before suggesting a new feature, please check the
  [GitHub Issues page](https://github.com/craole-cc/ccutils/issues) and the
  [Roadmap](utilities/ccutils/ROADMAP.md) document to see if similar ideas are
  already being discussed or planned.
- Open a new issue for your proposal.
- Clearly describe the feature and its purpose.
- Explain why you believe it would be a valuable addition to **ccutils**.
- Provide any mockups or examples if applicable.

---

### 3. Submitting Code Contributions

We welcome pull requests for bug fixes, new features, and improvements!

#### **Review the Roadmap**

Refer to the [Roadmap](utilities/ccutils/ROADMAP.md) to find a suitable task or
see where your proposed feature fits.

#### **Create a Changeset or Branch**

- **Using jj:** Create a new changeset for your work. This is similar to
  starting a new branch in Git, but with jj you work with changesets and
  bookmarks.

  ```bash
  jj new -m "feat: brief description of your change"
  # or, if you already have changes staged:
  jj commit -m "feat: brief description of your change"
  ```

  To make your changes easy to push and reference, set a bookmark (jj's
  equivalent of a branch):

  ```bash
  jj bookmark set feature/your-feature-name
  ```

- **Using git:**

  ```bash
  git checkout -b feature/your-feature-name
  ```

#### **Implement Your Changes**

- Write clean, idiomatic Rust code.
- Adhere to existing coding styles.
- Ensure your code is well-commented.
- Add integration and unit tests as appropriate.

#### **Test Your Changes**

- Run existing tests:

  ```bash
  cargo test
  ```

- Add new tests for new functionality or bug fixes.

#### **Update Documentation**

- Update `README.md` and `ROADMAP.md` as needed.

#### **Commit Your Work**

- **With jj:** Your changes are automatically tracked. To finalize your
  changeset and prepare for pushing, ensure your commit message is clear and
  complete:

  ```bash
  jj describe -m "feat: brief description of your change"
  ```

  If you set a bookmark, move it to your latest commit:

  ```bash
  jj bookmark move feature/your-feature-name
  ```

- **With git:**

  ```bash
  git add .
  git commit -m "feat: brief description of your change"
  ```

#### **Push Your Changes**

- **With jj:** Push your bookmark to your fork on GitHub:

  ```bash
  jj git push --branch feature/your-feature-name
  ```

  Or, to push all your bookmarks:

  ```bash
  jj git push --all
  ```

- **With git:**

  ```bash
  git push origin feature/your-feature-name
  ```

#### **Open a Pull Request**

- Open a pull request from your fork to the `main` branch of the
  `craole-cc/ccutils` repository.
- Provide a detailed description of your changes.
- Reference any related issues (e.g., `Closes #123`).

---

## Code of Conduct

Please note that all contributors are expected to adhere to our
[Code of Conduct](./CODE_OF_CONDUCT.md).

## Quick Reference: Jujutsu (jj) Workflow

```bash
# Create a new changeset
jj new -m "feat: your feature description"

# Set a bookmark for your feature
jj bookmark set feature/your-feature-name

# Update your commit message, if needed
jj describe

# Move your bookmark to the latest commit (if you made more commits)
jj bookmark move feature/your-feature-name

# Push your changes to your fork
jj git push --branch feature/your-feature-name
```

---

Thank you for contributing to **ccutils**!

---
