//! Implement simple reading of Human Readable Archive (.hrx) data.
//!
//! The Human Readable Achive format specification lives at
//! [https://github.com/google/hrx](https://github.com/google/hrx).
//!
//! This crate only supports _reading_ `.hrx` data.
//!
//! # Example
//!
//! ```
//! # use hrx_get::{Archive, Error};
//! # fn main() -> Result<(), Error> {
//! let archive = Archive::parse(
//!     "<===> one.txt\n\
//!      Content of one text file\n\
//!      <===>\n\
//!      This is a comment\n\
//!      <===> subdir/file.txt\n\
//!      Contents of a file in a subdir.\n\
//!      <===>\n"
//! )?;
//! assert_eq!(archive.get("one.txt"), Some("Content of one text file"));
//! # Ok(())
//! # }
//! ```
use std::collections::BTreeMap;
use std::fmt::{self, Display};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

/// Parsed Human Readable Archive data.
#[derive(Debug)]
pub struct Archive {
    files: BTreeMap<String, String>,
}

impl Archive {
    /// Load hrx data from a file system path.
    pub fn load(file: &Path) -> Result<Archive, FileError> {
        let data = read_to_string(file).map_err(|e| FileError::Io(file.into(), e))?;
        Archive::parse(&data).map_err(|e| FileError::Data(file.into(), e))
    }

    /// Parse hrx data from an in-memory buffer.
    pub fn parse(data: &str) -> Result<Archive, Error> {
        let mut files = BTreeMap::new();
        let boundary = format!("\n{}", find_boundary(data).ok_or(Error::NoBoundary)?);
        for item in data[boundary.len() - 1..].split(&boundary) {
            if item.is_empty() || item.starts_with('\n') {
                // item is a comment, ignore it.
            } else if let Some(item) = item.strip_prefix(' ') {
                if let Some(nl) = item.find('\n') {
                    let name = &item[..nl];
                    let body = &item[1 + nl..];
                    files.insert(name.into(), body.into());
                } else {
                    // Directory / empty file
                    files.insert(item.into(), String::new());
                }
            } else {
                return Err(Error::InvalidItem(item.into()));
            }
        }
        Ok(Archive { files })
    }

    /// Get a vec of the file names in the archive.
    pub fn names(&self) -> Vec<&str> {
        self.files.keys().map(|s| s.as_ref()).collect()
    }

    /// Get the contents of a file in the archive.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.files.get(name).map(|s| s.as_ref())
    }

    /// Iterate over (name, content) pairs for the files in the archive.
    pub fn entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.files.iter().map(|(k, v)| (k.as_ref(), v.as_ref()))
    }
}

fn find_boundary(data: &str) -> Option<&str> {
    for (i, b) in data.bytes().enumerate() {
        match (i, b) {
            (0, b'<') => (),
            (_i, b'=') => (),
            (i, b'>') => return Some(&data[0..i + 1]),
            _ => return None,
        }
    }
    None
}

/// An error reading or parsing a .hrx archive.
#[derive(Debug)]
pub enum FileError {
    /// Data error parsing archive
    Data(PathBuf, Error),
    /// I/O error reading archive
    Io(PathBuf, std::io::Error),
}

impl std::error::Error for FileError {}

impl Display for FileError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileError::Data(path, e) => {
                write!(out, "Failed to parse {:?}: {}", path, e)
            }
            FileError::Io(path, e) => {
                write!(out, "Failed to read {:?}: {}", path, e)
            }
        }
    }
}

/// An error reading or parsing a .hrx archive.
#[derive(Debug)]
pub enum Error {
    /// No archive bound found
    NoBoundary,
    /// Invalid item in archive
    InvalidItem(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::NoBoundary => {
                write!(out, "No archive boundary found")
            }
            Error::InvalidItem(item) => {
                write!(out, "Invalid item: {:?}", item)
            }
        }
    }
}
