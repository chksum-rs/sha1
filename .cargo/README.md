# chksum-sha1

[![GitHub](https://img.shields.io/badge/github-chksum--rs%2Fsha1-24292e?style=flat-square&logo=github "GitHub")](https://github.com/chksum-rs/sha1)
[![Build](https://img.shields.io/github/actions/workflow/status/chksum-rs/sha1/rust.yml?branch=master&style=flat-square&logo=github "Build")](https://github.com/chksum-rs/sha1/actions/workflows/rust.yml)
[![docs.rs](https://img.shields.io/docsrs/chksum-sha1?style=flat-square&logo=docsdotrs "docs.rs")](https://docs.rs/chksum-sha1/)
[![MSRV](https://img.shields.io/badge/MSRV-1.74.0-informational?style=flat-square "MSRV")](https://github.com/chksum-rs/sha1/blob/master/Cargo.toml)
[![deps.rs](https://deps.rs/crate/chksum-sha1/0.1.0/status.svg?style=flat-square "deps.rs")](https://deps.rs/crate/chksum-sha1/0.1.0)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square "unsafe forbidden")](https://github.com/rust-secure-code/safety-dance)
[![LICENSE](https://img.shields.io/github/license/chksum-rs/sha1?style=flat-square "LICENSE")](https://github.com/chksum-rs/sha1/blob/master/LICENSE)

An implementation of the SHA-1 hash function with a straightforward interface for computing digests of bytes, files, directories, and more.

## Setup

To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:

```toml
[dependencies]
chksum-sha1 = "0.1.0"
```

Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:

```shell
cargo add chksum-sha1
```

## Usage

Use the `chksum` function to calculate digest of file, directory and so on.

```rust
use chksum_sha1 as sha1;

let file = File::open(path)?;
let digest = sha1::chksum(file)?;
assert_eq!(
    digest.to_hex_lowercase(),
    "efaa311ae448a7374c122061bfed952d940e9e37"
);
```

For more usage examples, refer to the documentation available at [docs.rs](https://docs.rs/chksum-sha1/).

## License

This crate is licensed under the MIT License.
