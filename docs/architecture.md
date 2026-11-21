# Architecture

Bastion for Linux is a standalone Rust app. It shares no code with the other platforms, but reads and writes the same sync format as Bastion for Apple.

```text
        +------------------------------+
        |   GTK 4 + libadwaita + VTE   |
        +---------------+--------------+
                        |
        +---------------+--------------+
        |  LinuxApp/src (Rust, tokio)  |
        |  ssh, sftp, sync, forwarding |
        +---------------+--------------+
                        |
                 russh, reqwest
```

## Parts

- **SSH** goes through russh. RSA support is disabled in russh because the `rsa` crate has an unresolved vulnerability; see [RSA keys](rsa-disabled.md).
- **Sync** (`sync.rs`, `sync_crypto.rs`, `sync_webdav.rs`) follows the format from Bastion for Apple. WebDAV credentials are only sent over HTTPS or to an explicitly allowed loopback address.
- **Tools** such as Tailscale are fetched by `external_binary_fetcher.rs` and checked against SHA256 before use.

## Security boundary

<!-- coderabbit-ai-generated:v1 -->

<!-- coderabbit-ai-generated-section:v1:e5f95e381b71d725e90df68ec7aaf371f30b1716dd9dddc1faf7ad0a5cc47c80 -->

Host key verification in `known_hosts.rs` rejects a changed key for the same host and port. Learning and forgetting keys are serialized across Bastion processes using a persistent `known_hosts.lock` file. A connection fails if the trust store cannot be checked or a new key cannot be saved. Keys and passwords are never stored in the repo or in test data.

<!-- /coderabbit-ai-generated:v1 -->
