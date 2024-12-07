//! This crate provides an implementation of the SHA-1 hash function with a straightforward interface for computing digests of bytes, files, directories, and more.
//!
//! For a low-level interface, you can explore the [`chksum_hash_sha1`] crate.
//!
//! # Setup
//!
//! To use this crate, add the following entry to your `Cargo.toml` file in the `dependencies` section:
//!
//! ```toml
//! [dependencies]
//! chksum-sha1 = "0.1.0"
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```sh
//! cargo add chksum-sha1
//! ```     
//!
//! # Usage
//!
//! Use the [`chksum`] function to calculate digest of file, directory and so on.
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha1::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Asynchronous Runtime
//!
//! Use the [`async_chksum`] function to calculate digest of file, directory and so on.
//!
//! ```rust
//! # #[cfg(feature = "async-runtime-tokio")]
//! # {
//! # use std::path::Path;
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//! use tokio::fs::File;
//!
//! # async fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path).await?;
//! let digest = sha1::async_chksum(file).await?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! # Input Types
//!
//! ## Bytes
//!
//! ### Array
//!
//! ```rust
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let data = [0, 1, 2, 3];
//! let digest = sha1::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Vec
//!
//! ```rust
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let data = vec![0, 1, 2, 3];
//! let digest = sha1::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### Slice
//!
//! ```rust
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let data = &[0, 1, 2, 3];
//! let digest = sha1::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Strings
//!
//! ### str
//!
//! ```rust
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let data = "&str";
//! let digest = sha1::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ### String
//!
//! ```rust
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let data = String::from("String");
//! let digest = sha1::chksum(data)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## File
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::File;
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let file = File::open(path)?;
//! let digest = sha1::chksum(file)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Directory
//!
//! ```rust
//! # use std::path::Path;
//! use std::fs::read_dir;
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let readdir = read_dir(path)?;
//! let digest = sha1::chksum(readdir)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Path
//!
//! ```rust
//! # use std::path::Path;
//! use std::path::PathBuf;
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper(path: &Path) -> Result<()> {
//! let path = PathBuf::from(path);
//! let digest = sha1::chksum(path)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! ## Standard Input
//!
//! ```rust
//! use std::io::stdin;
//!
//! # use chksum_sha1::Result;
//! use chksum_sha1 as sha1;
//!
//! # fn wrapper() -> Result<()> {
//! let stdin = stdin();
//! let digest = sha1::chksum(stdin)?;
//! assert_eq!(
//!     digest.to_hex_lowercase(),
//!     "9fc42adac31303d68b444e6129f13f6093a0e045"
//! );
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! Cargo features are utilized to enable extra options.
//!
//! * `reader` enables the [`reader`] module with the [`Reader`] struct.
//! * `writer` enables the [`writer`] module with the [`Writer`] struct.
//!
//! By default, neither of these features is enabled.
//!
//! To customize your setup, disable the default features and enable only those that you need in your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! chksum-sha1 = { version = "0.1.0", features = ["reader", "writer"] }
//! ```
//!
//! Alternatively, you can use the [`cargo add`](https://doc.rust-lang.org/cargo/commands/cargo-add.html) subcommand:
//!
//! ```shell
//! cargo add chksum-sha1 --features reader,writer
//! ```
//!
//! ## Asynchronous Runtime
//!
//! * `async-runtime-tokio`: Enables async interface for Tokio runtime.
//!
//! By default, neither of these features is enabled.
//!
//! # Disclaimer
//!
//! The SHA-1 hash function should be used only for backward compatibility due to security issues.
//!
//! Check [RFC 6194: Security Considerations for the SHA-0 and SHA-1 Message-Digest Algorithms](https://www.rfc-editor.org/rfc/rfc6194) for more details.
//!
//! # License
//!
//! This crate is licensed under the MIT License.

#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![forbid(unsafe_code)]

#[cfg(feature = "reader")]
pub mod reader;
#[cfg(feature = "writer")]
pub mod writer;

use std::fmt::{self, Display, Formatter, LowerHex, UpperHex};

use chksum_core as core;
#[cfg(feature = "async-runtime-tokio")]
#[doc(no_inline)]
pub use chksum_core::AsyncChksumable;
#[doc(no_inline)]
pub use chksum_core::{Chksumable, Error, Hash, Hashable, Result};
#[doc(no_inline)]
pub use chksum_hash_sha1 as hash;

#[cfg(all(feature = "reader", feature = "async-runtime-tokio"))]
#[doc(inline)]
pub use crate::reader::AsyncReader;
#[cfg(feature = "reader")]
#[doc(inline)]
pub use crate::reader::Reader;
#[cfg(all(feature = "writer", feature = "async-runtime-tokio"))]
#[doc(inline)]
pub use crate::writer::AsyncWriter;
#[cfg(feature = "writer")]
#[doc(inline)]
pub use crate::writer::Writer;

/// Creates a new hash.
///
/// # Example
///
/// ```rust
/// use chksum_sha1 as sha1;
///
/// let mut hash = sha1::new();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "9fc42adac31303d68b444e6129f13f6093a0e045"
/// );
/// ```
#[must_use]
pub fn new() -> SHA1 {
    SHA1::new()
}

/// Creates a default hash.
///
/// # Example
///
/// ```rust
/// use chksum_sha1 as sha1;
///
/// let mut hash = sha1::default();
/// hash.update(b"example data");
/// let digest = hash.digest();
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "9fc42adac31303d68b444e6129f13f6093a0e045"
/// );
/// ```
#[must_use]
pub fn default() -> SHA1 {
    core::default()
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_sha1 as sha1;
///
/// let data = b"example data";
/// let digest = sha1::hash(data);
/// assert_eq!(
///     digest.to_hex_lowercase(),
///     "9fc42adac31303d68b444e6129f13f6093a0e045"
/// );
/// ```
pub fn hash(data: impl core::Hashable) -> Digest {
    core::hash::<SHA1>(data)
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_sha1 as sha1;
///
/// let data = b"example data";
/// if let Ok(digest) = sha1::chksum(data) {
///     assert_eq!(
///         digest.to_hex_lowercase(),
///         "9fc42adac31303d68b444e6129f13f6093a0e045"
///     );
/// }
/// ```
pub fn chksum(data: impl core::Chksumable) -> Result<Digest> {
    core::chksum::<SHA1>(data)
}

/// Computes the hash of the given input.
///
/// # Example
///
/// ```rust
/// use chksum_sha1 as sha1;
///
/// # async fn wrapper() {
/// let data = b"example data";
/// if let Ok(digest) = sha1::async_chksum(data).await {
///     assert_eq!(
///         digest.to_hex_lowercase(),
///         "9fc42adac31303d68b444e6129f13f6093a0e045"
///     );
/// }
/// # }
/// ```
#[cfg(feature = "async-runtime-tokio")]
pub async fn async_chksum(data: impl core::AsyncChksumable) -> Result<Digest> {
    core::async_chksum::<SHA1>(data).await
}

/// The SHA-1 hash instance.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SHA1 {
    inner: hash::Update,
}

impl SHA1 {
    /// Calculates the hash digest of an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1::SHA1;
    ///
    /// let data = b"example data";
    /// let digest = SHA1::hash(data);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "9fc42adac31303d68b444e6129f13f6093a0e045"
    /// );
    /// ```
    #[must_use]
    pub fn hash<T>(data: T) -> Digest
    where
        T: AsRef<[u8]>,
    {
        let mut hash = Self::new();
        hash.update(data);
        hash.digest()
    }

    /// Creates a new hash.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1::SHA1;
    ///
    /// let mut hash = SHA1::new();
    /// hash.update(b"example data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "9fc42adac31303d68b444e6129f13f6093a0e045"
    /// );
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let inner = hash::Update::new();
        Self { inner }
    }

    /// Updates the hash state with an input data.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1::SHA1;
    ///
    /// let mut hash = SHA1::new();
    /// hash.update(b"example");
    /// hash.update(" ");
    /// hash.update("data");
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "9fc42adac31303d68b444e6129f13f6093a0e045"
    /// );
    /// ```
    pub fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.inner.update(data);
    }

    /// Resets the hash state to its initial state.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1::SHA1;
    ///
    /// let mut hash = SHA1::new();
    /// hash.update(b"example data");
    /// hash.reset();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    /// );
    /// ```
    pub fn reset(&mut self) {
        self.inner.reset();
    }

    /// Produces the hash digest.
    ///
    /// # Example
    ///
    /// ```
    /// use chksum_sha1::SHA1;
    ///
    /// let mut hash = SHA1::new();
    /// let digest = hash.digest();
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    /// );
    /// ```
    #[must_use]
    pub fn digest(&self) -> Digest {
        self.inner.digest().into()
    }
}

impl core::Hash for SHA1 {
    type Digest = Digest;

    fn update<T>(&mut self, data: T)
    where
        T: AsRef<[u8]>,
    {
        self.update(data);
    }

    fn reset(&mut self) {
        self.reset();
    }

    fn digest(&self) -> Self::Digest {
        self.digest()
    }
}

/// A hash digest.
pub struct Digest(hash::Digest);

impl Digest {
    /// Creates a new digest.
    #[must_use]
    pub const fn new(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        let inner = hash::Digest::new(digest);
        Self(inner)
    }

    /// Returns a byte slice of the digest's contents.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }

    /// Consumes the digest, returning the digest bytes.
    #[must_use]
    pub fn into_inner(self) -> [u8; hash::DIGEST_LENGTH_BYTES] {
        let Self(inner) = self;
        inner.into_inner()
    }

    /// Returns a string in the lowercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1 as sha1;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xDA, 0x39, 0xA3, 0xEE,
    ///     0x5E, 0x6B, 0x4B, 0x0D,
    ///     0x32, 0x55, 0xBF, 0xEF,
    ///     0x95, 0x60, 0x18, 0x90,
    ///     0xAF, 0xD8, 0x07, 0x09,
    /// ];
    /// let digest = sha1::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_lowercase(),
    ///     "da39a3ee5e6b4b0d3255bfef95601890afd80709"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_lowercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_lowercase()
    }

    /// Returns a string in the uppercase hexadecimal representation.
    ///
    /// # Example
    ///
    /// ```rust
    /// use chksum_sha1 as sha1;
    ///
    /// #[rustfmt::skip]
    /// let digest = [
    ///     0xDA, 0x39, 0xA3, 0xEE,
    ///     0x5E, 0x6B, 0x4B, 0x0D,
    ///     0x32, 0x55, 0xBF, 0xEF,
    ///     0x95, 0x60, 0x18, 0x90,
    ///     0xAF, 0xD8, 0x07, 0x09,
    /// ];
    /// let digest = sha1::Digest::new(digest);
    /// assert_eq!(
    ///     digest.to_hex_uppercase(),
    ///     "DA39A3EE5E6B4B0D3255BFEF95601890AFD80709"
    /// );
    /// ```
    #[must_use]
    pub fn to_hex_uppercase(&self) -> String {
        let Self(inner) = self;
        inner.to_hex_uppercase()
    }
}

impl core::Digest for Digest {}

impl AsRef<[u8]> for Digest {
    fn as_ref(&self) -> &[u8] {
        let Self(inner) = self;
        inner.as_bytes()
    }
}

impl Display for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        Display::fmt(inner, f)
    }
}

impl LowerHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        LowerHex::fmt(inner, f)
    }
}

impl UpperHex for Digest {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let Self(inner) = self;
        UpperHex::fmt(inner, f)
    }
}

impl From<[u8; hash::DIGEST_LENGTH_BYTES]> for Digest {
    fn from(digest: [u8; hash::DIGEST_LENGTH_BYTES]) -> Self {
        Self::new(digest)
    }
}

impl From<hash::Digest> for Digest {
    fn from(digest: hash::Digest) -> Self {
        Self(digest)
    }
}
