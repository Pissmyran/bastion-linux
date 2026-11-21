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

<!-- coderabbit-ai-generated-section:v1:0b8183dc5bd147570beae2d7db6abbbb3cd2d9ba6997a6df4c0c1799c4a46e3a -->

## SSH config import

File import follows `Include` directives in order. It skips missing files and active include cycles. To bound resource use, imports fail before saving hosts if the expanded config exceeds 512 file attempts or include targets, 1 MiB of data, 4,096 entries, 256 host patterns, or 8,192 directory entries scanned. The existing include depth limit is 16.

<!-- /coderabbit-ai-generated:v1 -->

## Contents

| Path | Contents |
| --- | --- |
| `LinuxApp/src/` | The app: sessions, terminal, SFTP, sync, forwarding and tools |
| `LinuxApp/integrations/` | Integrations, for example Docker |
| `LinuxApp/tests/fixtures/` | Copies of the sync-format fixtures from Bastion for Apple |
| `LinuxApp/packaging/` | Desktop file and RPM build |
