# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Run [`cargo-release`][cr] to publish a release.

[cr]: https://github.com/sunng87/cargo-release/

## [Unreleased]

### Fixed

- Multiline messages are no longer lost (see [GH-2]), following an update of [libsystemd] (see [libsystemd GH-70] and [libsystemd GH-70]).

[GH-2]: https://github.com/lunaryorn/systemd-journal-logger.rs/pull/2
[libsystemd]: https://github.com/lucab/libsystemd-rs
[libsystemd GH-70]: https://github.com/lucab/libsystemd-rs/issues/70
[libsystemd GH-72]: https://github.com/lucab/libsystemd-rs/pull/72

## [0.1.0] – 2021-05-28

Initial release with `systemd_journal_logger::LOG` and `systemd_journal_logger::init`.

Do **not use** this version; it looses multiline messages and has been yanked from crates.io.

[Unreleased]: https://github.com/lunaryorn/systemd-journal-logger.rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/lunaryorn/systemd-journal-logger.rs/releases/tag/v0.1.0