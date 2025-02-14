// Copyright (c) 2021-2022 Harry [Majored] [hello@majored.pw]
// MIT License (https://github.com/Majored/rs-async-zip/blob/main/LICENSE)

//! An asynchronous ZIP archive reading/writing crate powered by [`tokio`].
//!
//! ## Features
//! - Support for Stored, Deflate, bzip2, LZMA, zstd, and xz compression methods.
//! - Various different reading approaches (seek, stream, filesystem, in-memory buffer).
//! - Support for writing complete data (u8 slices) or stream writing using data descriptors.
//! - Initial support for ZIP64 reading.
//! - Aims for reasonable [specification](https://github.com/Majored/rs-async-zip/blob/main/SPECIFICATION.md) compliance.
//!
//! ## Installation
//!
//! ```toml
//! [dependencies]
//! async_zip = { version = "0.0.11", features = ["full"] }
//! ```
//!
//! ### Feature Flags
//! - `full` - Enables all below features.
//! - `chrono` - Enables support for parsing dates via `chrono`.
//! - `fs` - Enables support for the `fs` reading module.
//! - `deflate` - Enables support for the Deflate compression method.
//! - `bzip2` - Enables support for the bzip2 compression method.
//! - `lzma` - Enables support for the LZMA compression method.
//! - `zstd` - Enables support for the zstd compression method.
//! - `xz` - Enables support for the xz compression method.
//!
//! [Read more.](https://github.com/Majored/rs-async-zip)

pub mod error;
pub mod read;
pub mod write;

pub(crate) mod entry;
pub(crate) mod file;
pub(crate) mod spec;
pub(crate) mod utils;

#[cfg(test)]
pub(crate) mod tests;

pub use crate::spec::attribute::AttributeCompatibility;
pub use crate::spec::compression::{Compression, DeflateOption};
pub use crate::spec::date::ZipDateTime;

pub use crate::entry::{builder::ZipEntryBuilder, StoredZipEntry, ZipEntry};
pub use crate::file::{builder::ZipFileBuilder, ZipFile};
