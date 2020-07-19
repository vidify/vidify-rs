use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

/// The different errors that may happen are stored in this enum. These
/// include errors specific to some APIs because that way they can be handled
/// correctly.
#[derive(Debug)]
pub enum Error {
    ConfigParse(structconf::Error),
    IO(std::io::Error),
    FailedRequest(String),
    NoTrackPlaying,
    SpotifyWebAuth,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Error::*;
        match self {
            ConfigParse(err) => write!(
                f,
                "Failed parsing the configuration: {}",
                err.to_string()
            ),
            IO(err) => write!(f, "IO error: {}", err.to_string()),
            FailedRequest(desc) => write!(f, "Failed request: {}", desc),
            NoTrackPlaying => write!(f, "No track currently playing"),
            SpotifyWebAuth => {
                write!(f, "Couldn't authenticate Spotify Web API")
            }
        }
    }
}

impl From<structconf::Error> for Error {
    fn from(err: structconf::Error) -> Self {
        Error::ConfigParse(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}
