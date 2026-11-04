# AGENTS.md – bastion_linux

Bastion for Linux: the SSH client for Linux, built with Rust, GTK 4, libadwaita, VTE and russh.

Read the shared rules at the start of every session: [GOVERNANCE.md](https://raw.githubusercontent.com/pissmyran/.github/main/GOVERNANCE.md) and [BASTION.md](https://raw.githubusercontent.com/pissmyran/.github/main/BASTION.md).

## Read first

1. [docs/architecture.md](docs/architecture.md): components and security boundaries.
2. [docs/operations.md](docs/operations.md): verification before pushing.
3. [docs/sync-format.md](docs/sync-format.md): when the host list or sync is involved.

## Commands

Install build dependencies (Debian and Ubuntu); `openssh-client` is needed because the key import tests run `ssh-keygen`:

```bash
sudo apt install --no-install-recommends build-essential pkg-config libgtk-4-dev libadwaita-1-dev libvte-2.91-gtk4-dev libgtksourceview-5-dev openssh-client
```

Test and run:

```bash
cd LinuxApp && cargo test --workspace
cd LinuxApp && cargo run
```

The RPM package is built with `LinuxApp/packaging/build-rpm.sh`.

<!-- coderabbit-ai-generated:v1 -->

<!-- coderabbit-ai-generated-section:v1:cf7c6384f22e7cce08171419c2264772776e20717dc509ec73159cf23d7065e6 -->

SFTP uploads skip child symbolic links and reject a symbolic link selected as the root.

<!-- /coderabbit-ai-generated:v1 -->

## Rules

- **The GTK packages above must be installed** for `cargo test` to compile.
- **The fixtures** in `LinuxApp/tests/fixtures/` are copies of the sync format owned by `bastion_apple`.
- **RSA is disabled** in russh; `docs/rsa-disabled.md` gives the reason and the condition for enabling it.
- **Extra care:** host key verification, sync format and encryption, OAuth and WebDAV, forwarding and fetched tools.
