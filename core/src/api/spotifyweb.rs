//! This implements the official web API, using the `rspotify` module.
//! The web API provides much more metadata about the Spotify player but
//! it's limited in terms of usabilty:
//!     * The user has to sign in and manually set it up
//!     * Only Spotify Premium users are able to use some functions
//!     * API calls are limited, so it's not as responsive

use crate::api::APIBase;
use crate::config::Config;
use crate::error::{Error, Result};

use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::blocking::util::get_token;

pub struct SpotifyWeb {
    spotify: Spotify,
}

impl APIBase for SpotifyWeb {
    fn new(config: &Config) -> Result<Self> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&config.client_id.clone().ok_or(Error::SpotifyWebAuth)?)
            .client_secret(&config.client_secret.clone().ok_or(Error::SpotifyWebAuth)?)
            .redirect_uri(&config.redirect_uri)
            .scope("user-read-currently-playing user-read-playback-state")
            .build();

        let oauth = get_token(&mut oauth).ok_or(Error::SpotifyWebAuth)?;
        let creds = SpotifyClientCredentials::default()
            .token_info(oauth)
            .build();

        let spotify = Spotify::default()
            .client_credentials_manager(creds)
            .build();

        Ok(SpotifyWeb {
            spotify
        })
    }

    fn get_player_name(&self) -> &str {
        "test"
    }

    fn get_artist(&self) -> &str {
        "test"
    }

    fn get_title(&self) -> &str {
        "test"
    }

    fn get_position(&self) -> u32 {
        123
    }

    fn is_playing(&self) -> bool {
        true
    }

    fn event_loop(&mut self) {
        unimplemented!();
    }
}
