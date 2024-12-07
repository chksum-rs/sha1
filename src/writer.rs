//! This module is optional and can be enabled using the `writer` Cargo feature.
//!
//! The [`Writer`] allows on-the-fly calculation of the digest while writing the data.
//!
//! # Enabling
//!
//! Add the following entry to your `Cargo.toml` file to enable the `writer` feature:
//!
//! ```toml
//! [dependencies]
//! chksum-sha1 = { version = "0.1.0", features = ["writer"] }
//! ```
//!
//! Alternatively, use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-sha1 --features writer
//! ```
//!
//! # Example
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//! use std::io::Write; // required by writer
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let mut writer = sha1::writer::new(file);
//!
//! writer.write_all(b"example data")?;
//!
//! let digest = writer.digest();
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```

use std::io::Write;

use chksum_writer as writer;
#[cfg(feature = "async-runtime-tokio")]
use tokio::io::AsyncWrite;

use crate::SHA1;

/// A specialized [`Writer`](writer::Writer) type with the [`SHA1`] hash algorithm.
pub type Writer<W> = writer::Writer<W, SHA1>;

#[cfg(feature = "async-runtime-tokio")]
/// A specialized [`AsyncWriter`](writer::AsyncWriter) type with the [`SHA1`] hash algorithm.
pub type AsyncWriter<R> = writer::AsyncWriter<R, SHA1>;

/// Creates new [`Writer`].
pub fn new(inner: impl Write) -> Writer<impl Write> {
    writer::new(inner)
}

/// Creates new [`Writer`] with provided hash.
pub fn with_hash(inner: impl Write, hash: SHA1) -> Writer<impl Write> {
    writer::with_hash(inner, hash)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`].
pub fn async_new(inner: impl AsyncWrite) -> AsyncWriter<impl AsyncWrite> {
    writer::async_new(inner)
}

#[cfg(feature = "async-runtime-tokio")]
/// Creates new [`AsyncWriter`] with provided hash.
pub fn async_with_hash(inner: impl AsyncWrite, hash: SHA1) -> AsyncWriter<impl AsyncWrite> {
    writer::async_with_hash(inner, hash)
}
