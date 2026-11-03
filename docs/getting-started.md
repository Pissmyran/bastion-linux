# Getting started

## Prerequisites

Rust (latest stable) and the GTK development packages above.

## Build and test

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

<!-- coderabbit-ai-generated:v1 -->

<!-- coderabbit-ai-generated-section:v1:ab7b32a01ba93f9a30d3380c3b5a14b4269b263ed331216b51172eac07bfcc2e -->

SFTP directory uploads skip symbolic links; a symbolic link selected as the upload root is rejected.

<!-- /coderabbit-ai-generated:v1 -->

## Contents

| Path | Contents |
| --- | --- |
| `LinuxApp/src/` | The app: sessions, terminal, SFTP, sync, forwarding and tools |
| `LinuxApp/integrations/` | Integrations, for example Docker |
| `LinuxApp/tests/fixtures/` | Copies of the sync-format fixtures from Bastion for Apple |
| `LinuxApp/packaging/` | Desktop file and RPM build |
