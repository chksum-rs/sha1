//! This module is optional and can be enabled using the `reader` Cargo feature.
//!
//! The [`Reader`] allows on-the-fly calculation of the digest while reading the data.
//!
//! # Enabling
//!
//! Add the following entry to your `Cargo.toml` file to enable the `reader` feature:
//!
//! ```toml
//! [dependencies]
//! chksum-sha1 = { version = "0.0.0", features = ["reader"] }
//! ```
//!
//! Alternatively, use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-sha1 --features reader
//! ```
//!
//! # Example
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//! use std::io::Read; // required by reader
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let mut reader = sha1::reader::new(file);
//!
//! let mut buffer = Vec::new();
//! reader.read_to_end(&mut buffer)?;
//! assert_eq!(buffer, b"example data");
//!
//! let digest = reader.digest();
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```

use std::io::Read;

use chksum_reader as reader;
#[cfg(feature = "async-runtime-tokio")]
use tokio::io::AsyncRead;

use crate::SHA1;

/// A specialized [`Reader`](reader::Reader) type with the [`SHA1`] hash algorithm.
pub type Reader<R> = reader::Reader<R, SHA1>;

#[cfg(feature = "async-runtime-tokio")]
/// A specialized [`AsyncReader`](reader::AsyncReader) type with the [`SHA1`] hash algorithm.
pub type AsyncReader<R> = reader::AsyncReader<R, SHA1>;

/// Creates new [`Reader`].
pub fn new(inner: impl Read) -> Reader<impl Read> {
    reader::new(inner)
}

/// Creates new [`Reader`] with provided hash.
pub fn with_hash(inner: impl Read, hash: SHA1) -> Reader<impl Read> {
    reader::with_hash(inner, hash)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncReader`].
pub fn async_new(inner: impl AsyncRead) -> AsyncReader<impl AsyncRead> {
    reader::async_new(inner)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncReader`] with provided hash.
pub fn async_with_hash(inner: impl AsyncRead, hash: SHA1) -> AsyncReader<impl AsyncRead> {
    reader::async_with_hash(inner, hash)
}
