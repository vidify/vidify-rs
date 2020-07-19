use crate::error::Result;

use std::fs::create_dir;
use std::ops::Deref;
use std::path::PathBuf;

use dirs::*;

/// What kind of resource it is, to determine where it should be stored. The
/// custom type holds the full path, the rest of them only contain the file
/// name, which will be appended to a predetermined directory.
pub enum ResKind {
    Custom(String),
    Config(String),
    Data(String),
}

pub struct Res {
    pub path: String,
}

/// Small wrapper for resource files used in Vidify.
impl Res {
    pub fn new(kind: ResKind) -> Result<Res> {
        use std::io::{Error, ErrorKind};
        use ResKind::*;

        let path = match kind {
            Custom(path) => path,
            Config(file) => Res::custom(
                &mut config_dir()
                    .ok_or(Error::new(ErrorKind::NotFound, "config dir"))?,
                &file,
            )?,
            Data(file) => Res::custom(
                &mut data_dir()
                    .ok_or(Error::new(ErrorKind::NotFound, "data dir"))?,
                &file,
            )?,
        };

        Ok(Res { path })
    }

    fn custom(path: &mut PathBuf, file: &str) -> Result<String> {
        // Creating the directory first
        path.push("vidify");
        if !path.exists() {
            create_dir(&path)?;
        }

        // And then the file
        path.push(file);
        if !path.exists() {
            std::fs::File::create(&path)?;
        }

        Ok(path.to_string_lossy().into_owned())
    }
}

impl Deref for Res {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.path
    }
}
