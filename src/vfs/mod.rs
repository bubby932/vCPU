#![allow(dead_code)]

use std::fmt;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VfsError {
    code : VfsErrorCode
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parsed_err = match self.code {
            VfsErrorCode::ENOPERM => "File is read-only!",
            VfsErrorCode::ENOFILE => "File does not exist!"
        };
        write!(f, "VFS error: {}", parsed_err)
    }
}

#[derive(Debug, Clone)]
enum VfsErrorCode {
    ENOPERM,
    ENOFILE
}

// Nowhere near an ideal implementation, deal w it.
pub struct VFS {
    files : HashMap<u8, File>,
    ct : u8
}

impl VFS {
    pub fn create_with_files(files : HashMap<u8, File>) -> Self {
        Self {
            ct : files.len() as u8,
            files
        }
    }
    pub fn create_empty() -> Self {
        Self {
            files : HashMap::new(),
            ct: 0
        }
    }
    pub fn create_file(&mut self, contents : Vec<u8>, name : String, read_only : bool) -> File {
        self.ct += 1;
        File {
            contents,
            identifier: self.ct,
            name,
            properties : VfsFileProperties { 
                read_only 
            }
        }
    }
    pub fn write_file(&mut self, file : File) -> Result<(), VfsError> {
        if let Some(f) = self.files.get(&file.identifier) {
            if f.properties.read_only {
                return Err(VfsError {
                    code : VfsErrorCode::ENOPERM
                });
            }
        }

        self.files.insert(file.identifier, file);
        Ok(())
    }
    pub fn read_file(&self, identifier : u8) -> Result<&File, VfsError> {
        if let Some(f) = self.files.get(&identifier) {
            return Ok(f);
        }

        Err(VfsError {
            code: VfsErrorCode::ENOFILE
        })
    }
    pub fn dmp(&self) -> &HashMap<u8, File> {
        &self.files
    }
}

#[derive(Debug)]
pub struct File {
    pub contents : Vec<u8>,
    pub identifier : u8,
    pub name : String,
    pub properties : VfsFileProperties
}

#[derive(Debug)]
pub struct VfsFileProperties {
    pub read_only : bool,
}