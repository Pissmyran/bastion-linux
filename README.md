> Guide: [AI- och Copilot-inställningar på GitHub](https://github.com/pissmyran/.github/blob/main/public/ai-och-copilot.md) · [English](https://github.com/pissmyran/.github/blob/main/public/eng/ai-and-copilot.md)

# Bastion for Linux

**Documentation:** [pissmyran.github.io/bastion_linux](https://pissmyran.github.io/bastion_linux/)

Bastion is an SSH client. This repo holds the version for Linux with GTK 4, built with Rust, GTK 4, libadwaita, VTE and russh. The other platforms have their own repos: [Bastion for Apple](https://github.com/pissmyran/bastion_apple), [Bastion for Windows](https://github.com/pissmyran/bastion_windows), [Bastion for Android](https://github.com/pissmyran/bastion_android). The [Bastion hub](https://github.com/pissmyran/bastion) ties them together.

## Contents

| Path | Contents |
| --- | --- |
| `LinuxApp/src/` | The app: sessions, terminal, SFTP, sync, forwarding and tools |
| `LinuxApp/integrations/` | Integrations, for example Docker |
| `LinuxApp/tests/fixtures/` | Copies of the sync-format fixtures from Bastion for Apple |
| `LinuxApp/packaging/` | Desktop file and RPM build |

## Getting started

Install build dependencies (Debian and Ubuntu):

```bash
sudo apt install --no-install-recommends build-essential pkg-config libgtk-4-dev libadwaita-1-dev libvte-2.91-gtk4-dev libgtksourceview-5-dev openssh-client
```

Test:

```bash
cd LinuxApp && cargo test --workspace
```

Run the app:

```bash
cd LinuxApp && cargo run
```

`openssh-client` is needed because the key import tests run `ssh-keygen`. The RPM package is built with `LinuxApp/packaging/build-rpm.sh`.

## More

- [The documentation](https://pissmyran.github.io/bastion_linux/): getting started, architecture, operations, sync format and releases
- [SECURITY.md](SECURITY.md): report a vulnerability
- [CONTRIBUTING.md](CONTRIBUTING.md): contribute
