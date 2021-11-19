# `semver-explain`

__Convert SemVer requirements to their most-obvious equivalents.__

`semver-explain` is a CLI tool to explain Semantic Versioning requirements by converting them to a form with only less-than, greater-than and/or equal-to comparators, where the major, minor, and patch versions are all specified.

The exact interpretation of SemVer requirements' meaning [comes from Cargo][cargo_semver]. In particular, it does not handle [hyphen ranges or x-ranges][node_semver] from the `node-semver` library for JavaScript.

## Why?

It's easy to forget the exact meaning of SemVer comparators like `~`, `^`, and `*`. Rather than looking up documentation to confirm what's meant by a requirement, you can plug it into `semver-explain` and get a set of more-obvious equivalent requirements!

## Examples

```sh
$ semver-explain "^1.4.0"
>=1.4.0, <2.0.0
$ semver-explain "~0.5.3"
>=0.5.3, <0.6.0
$ semver-explain "5.6.*"
>=5.6.0, <5.7.0
```

## Installation

`semver-explain` is written in Rust, so you'll need to [install Rust][install_rust] first.

```sh
# To install from Crates.io
$ cargo install semver-explain
# To install from source (after cloning or downloading)
$ cargo install --path "<path to download>"
```

## How to Use

Just give it a SemVer requirement as a string! The CLI is purposefully simple.

```
semver-explain 0.2.0

USAGE:
    semver-explain <VERSION_REQ>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <VERSION_REQ>    semantic versioning requirement to explain
```

## License

`semver-explain` is MIT licensed. The full license text can be found in `LICENSE.md`.

## Thanks

This tool uses the excellent Rust [`semver`][semver] library, without which it would have been much more tedious to build.

[cargo_semver]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html "Link to Cargo's explanation of semver requirements"
[node_semver]: https://github.com/npm/node-semver#advanced-range-syntax "Link to node-semver's range syntax"
[install_rust]: https://www.rust-lang.org/tools/install "Link to Rust installation instructions."
[semver]: https://crates.io/crates/semver "Link to the semver crate"

