# [![GGRS LOGO](./ggrs_logo.png)](https://gschup.github.io/ggrs/)

[![crates.io](https://img.shields.io/crates/v/ggrs?style=for-the-badge)](https://crates.io/crates/ggrs)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/gschup/ggrs/Rust?style=for-the-badge)

## P2P Rollback Networking in Rust

GGRS (good game rollback system) is a reimagination of the [GGPO network SDK](https://www.ggpo.net/) written in 100% safe [Rust 🦀](https://www.rust-lang.org/). The callback-style API from the original library has been replaced with a much saner, simpler control flow. Instead of registering callback functions, GGRS returns a list of requests for the user to fulfill.

If you are interested in integrating rollback networking into your game or just want to chat with other rollback developers (not limited to Rust), check out the [GGPO Developers Discord](https://discord.com/invite/8FKKhCRCCE)!

## Getting Started

To get started with GGRS, check out the following resources:

- [Website](https://gschup.github.io/ggrs/)
- [Tutorial](https://gschup.github.io/ggrs/docs/getting-started/quick-start/)
- [Examples](./examples/README.md)
- [Documentation](https://docs.rs/ggrs/newest/ggrs/)

## Development Status

GGRS is in an early stage, but the main functionality for multiple players and spectators should be quite stable. See the Changelog for the latest changes, even those yet unreleased on crates.io! If you want to contribute, check out existing issues, as well as the contribution guide!

- [Changelog](./CHANGELOG.md)
- [Issues](https://github.com/gschup/ggrs/issues)
- [Contribution Guide](https://gschup.github.io/ggrs/docs/contributing/how-to-contribute/)

## Bevy Plugin

GGRS has a Bevy plugin currently in development. Check it out!

- [Bevy GGRS](https://github.com/gschup/bevy_ggrs)

## Other Rollback Implementations in Rust

Also take a look at the awesome [backroll-rs](https://github.com/HouraiTeahouse/backroll-rs/)!

## Licensing

GGRS is dual-licensed under either

- [MIT License](./LICENSE-MIT): Also available [online](http://opensource.org/licenses/MIT)
- [Apache License, Version 2.0](./LICENSE-APACHE): Also available [online](http://www.apache.org/licenses/LICENSE-2.0)

at your option.
