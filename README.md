# `semver-explain`

__Convert SemVer requirements to their most-obvious equivalents.__

`semver-explain` is a CLI tool to explain Semantic Versioning requirements by converting them to a form with only less-than, greater-than and/or equal-to comparators, where the major, minor, and patch versions are all specified. These equivalent bounds are output in SemVer requirement format, to be pasted in wherever SemVer requirements are expected.

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

## License

`semver-explain` is MIT licensed. The full license text can be found in `LICENSE.md`.

[install_rust]: https://www.rust-lang.org/tools/install "Link to Rust installation instructions."

