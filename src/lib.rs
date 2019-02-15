//! Implement simple reading of Human Readable Archive (.hrx) data.
//!
//! The Human Readable Achive format specification lives at
//! [https://github.com/google/hrx](https://github.com/google/hrx).
//!
//! This crate only supports _reading_ `.hrx` data.
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;

/// Parsed Human Readable Archive data.
#[derive(Debug)]
pub struct Archive {
    files: BTreeMap<String, String>,
}

impl Archive {
    /// Load hrx data from a file system path.
    pub fn load(file: &Path) -> Result<Archive, String> {
        let mut f = File::open(file).map_err(|e| format!("{}", e))?;
        let mut data = vec![];
        f.read_to_end(&mut data).map_err(|e| format!("{}", e))?;
        Archive::parse(from_utf8(&data).map_err(|e| format!("{}", e))?)
    }

    /// Parse hrx data from an in-memory buffer.
    pub fn parse(data: &str) -> Result<Archive, String> {
        let mut files = BTreeMap::new();
        let boundary = format!(
            "\n{}",
            find_boundary(data).ok_or("No archive boundary found".to_string())?
        );
        for item in data[boundary.len() - 1..].split(&boundary) {
            if item == "" || item.starts_with('\n') {
                eprintln!("Got comment: {:?}", item);
            } else if item.starts_with(' ') {
                if let Some(nl) = item.find('\n') {
                    let name = &item[1..nl];
                    let body = &item[1 + nl..];
                    files.insert(name.into(), body.into());
                } else {
                    Err(format!("Invalid item: {:?}", item))?
                }
            } else {
                Err(format!("Invalid item: {:?}", item))?
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
