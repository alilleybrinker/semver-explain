# `semver-explain`

`semver-explain` is a CLI tool to explain Semantic Versioning requirements by converting them to a form with only less-than, greater-than and/or equal-to comparators.

## Why?

I often forget the exact meaning of SemVer requirements, and looking at the documentation and figuring it out each time I forget is tedious. This way, I can run a program to tell me the most obvious bounds for the specific requirements I'm looking at.

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

```sh
# To install from Crates.io
$ cargo install semver-explain
# To install from source (after cloning or downloading)
$ cargo install --path "<path to download>"
```

## License

`semver-explain` is MIT licensed. The full license text can be found in `LICENSE.md`.

