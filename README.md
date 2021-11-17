# `semver-explain`

`semver-explain` is a CLI tool to explain Semantic Versioning requirements by converting them to a form with only less-than, greater-than and/or equal-to comparators.

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

You can install `semver-explain` with `cargo install semver-explain` or by cloning this repository and running `cargo install --path <path to this repository>`.

