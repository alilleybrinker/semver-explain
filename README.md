# `semver-explain`

`semver-explain` is a CLI tool to explain Semantic Versioning requirements by converting them to a form with only less-than, greater-than and/or equal-to comparators.

```sh
$ semver-explain "^1.4.0"
>=1.4.0, <2.0.0
$ semver-explain "~0.5.3"
>=0.5.3, <0.6.0
$ semver-explain "5.6.*"
>=5.6.0, <5.7.0
```
