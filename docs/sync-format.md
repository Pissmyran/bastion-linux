# Sync format

Bastion for Linux reads and writes the same sync format as the other platforms. The **source** of the format is the Swift models in [Bastion for Apple](https://github.com/Pissmyran/bastion-apple).

## The fixtures

`LinuxApp/tests/fixtures/` holds copies of the fixtures in [Bastion for Apple](https://github.com/Pissmyran/bastion-apple/tree/main/Tests/fixtures). The tests in `host.rs` and `tool_release.rs` read them and show that the Rust side can decode the format.

## When the format changes in Bastion for Apple

1. Copy the changed fixtures from `Tests/fixtures/` in Bastion for Apple to `LinuxApp/tests/fixtures/`.
2. Update the readers and writers in `host.rs`, `snippet.rs`, `sync.rs` and `sync_crypto.rs`.
3. Run the tests. Never change a fixture to make a test pass.
