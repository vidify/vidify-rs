use std::fmt;

use structconf;

pub type Result<T> = std::result::Result<T, Error>;

/// The different errors that may happen are stored in this enum. These
/// include errors specific to some APIs because that way they can be handled
/// correctly.
#[derive(Debug)]
pub enum Error {
    ConfigParse(structconf::Error),
    SpotifyWebAuth,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            SpotifyWebAuth => write!(f, "Couldn't authenticate Spotify Web API"),
            ConfigParse(err) => write!(f, "Failed parsing the configuration: {}", err.to_string()),
        }
    }
}

impl From<structconf::Error> for Error {
    fn from(err: structconf::Error) -> Self {
        Error::ConfigParse(err)
    }
}
