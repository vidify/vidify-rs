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
use rspotify::model::playing::Playing;
use rspotify::blocking::util::get_token;

pub struct SpotifyWeb {
    spotify: Spotify,
    playing: Playing,
}

impl APIBase for SpotifyWeb {
    fn new(config: &Config) -> Result<Self> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&config.client_id.clone().ok_or(Error::SpotifyWebAuth)?)
            .client_secret(&config.client_secret.clone().ok_or(Error::SpotifyWebAuth)?)
            .redirect_uri(&config.redirect_uri)
            .scope("user-read-currently-playing user-read-playback-state")
            .build();

        // The refresh token is attempted to be reused from previous
        // sessions.
        let token = match &config.refresh_token {
            Some(token) => oauth
                .refresh_access_token_without_cache(&token)
                .ok_or(Error::SpotifyWebAuth)?,
            // TODO: use the GUI for this once it's finished
            None => get_token(&mut oauth).ok_or(Error::SpotifyWebAuth)?,
        };
        let creds = SpotifyClientCredentials::default()
            .token_info(token)
            .build();

        let spotify = Spotify::default()
            .client_credentials_manager(creds)
            .build();

        Ok(SpotifyWeb {
            playing: spotify
                .current_user_playing_track()
                .map_err(|e| Error::FailedRequest(e.to_string()))?
                .ok_or(Error::NoTrackPlaying)?,
            spotify,
        })
    }

    // fn update(&mut self) -> Result<()> {
        // self.playing = self.refresh_metadata()?;
    // }

    // fn refresh_metadata(&self) -> Result<Playing> {
        // self.spotify.current_user_playing_track()?.ok_or(Error::NoTrackPlaying)
    // }

    fn get_player_name(&self) -> &str {
        "Spotify Web Player"
    }

    fn get_artist(&self) -> Option<&str> {
        if let Some(item) = &self.playing.item {
            if let Some(artist) = item.artists.get(0) {
                return Some(artist.name.as_str());
            }
        }

        None
    }

    fn get_title(&self) -> Option<&str> {
        self.playing.item.as_ref().and_then(|item| Some(item.name.as_str()))
    }

    fn get_position(&self) -> Option<u32> {
        self.playing.progress_ms
    }

    fn is_playing(&self) -> bool {
        self.playing.is_playing
    }

    fn event_loop(&mut self) {
        unimplemented!();
    }
}
