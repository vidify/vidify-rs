//! This implements the official web API, using the `rspotify` module.
//! The web API provides much more metadata about the Spotify player but
//! it's limited in terms of usabilty:
//!     * The user has to sign in and manually set it up
//!     * Only Spotify Premium users are able to use some functions
//!     * API calls are limited, so it's not as responsive

use crate::api::APIBase;
use crate::config::Config;
use crate::error::{Error, Result};

use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;

use log::{error, info};
use rspotify::blocking::client::Spotify;
use rspotify::blocking::oauth2::{
    SpotifyClientCredentials, SpotifyOAuth, TokenInfo,
};
use rspotify::model::playing::Playing;

pub struct SpotifyWeb {
    spotify: Spotify,
    playing: Playing,
}

impl APIBase for SpotifyWeb {
    fn new(config: &Config) -> Result<Self> {
        let mut oauth = SpotifyOAuth::default()
            .client_id(&config.client_id.clone().ok_or(Error::SpotifyWebAuth)?)
            .client_secret(
                &config.client_secret.clone().ok_or(Error::SpotifyWebAuth)?,
            )
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
            None => get_token(&mut oauth)?,
        };
        let creds = SpotifyClientCredentials::default()
            .token_info(token)
            .build();

        let spotify =
            Spotify::default().client_credentials_manager(creds).build();

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
        self.playing
            .item
            .as_ref()
            .and_then(|item| Some(item.name.as_str()))
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

/// A small server will be ran to obtain the token without user interaction,
/// besides logging in to Spotify in the browser.
fn get_token(oauth: &mut SpotifyOAuth) -> Result<TokenInfo> {
    // A thread will open a web server in order to obtain the authentication
    // code.
    let (sx, rx) = mpsc::channel();
    let uri = oauth.redirect_uri.clone();
    thread::spawn(move || {
        let uri = to_bind_format(&uri);
        let listener = TcpListener::bind(uri).unwrap();
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => match get_code(stream) {
                    Ok(code) => {
                        sx.send(code).unwrap();
                        break;
                    }
                    Err(err) => {
                        error!(
                            "Error when obtaining the code: {}",
                            err.to_string()
                        );
                    }
                },
                Err(err) => {
                    error!("Unable to connect: {}", err);
                }
            }
        }
    });

    // Obtaining the autorization URL to open it and start the authentication
    // process. The web server thread will send the obtained code and close
    // itself once it's done.
    let url = oauth.get_authorize_url(None, Some(true));
    webbrowser::open(&url).unwrap();
    let code: String = rx.recv().unwrap();
    let token = oauth
        .get_access_token_without_cache(&code)
        .ok_or(Error::SpotifyWebAuth)?;

    Ok(token)
}

/// Converting a redirect uri like `http://localhost:8888/callback/` into
/// `localhost:8888` so that it can be used for the TCP listener.
fn to_bind_format(bind_uri: &str) -> &str {
    bind_uri.split("/").nth(2).expect(
        "Invalid redirect uri, it must follow the regular expression \
                `.*//(.+:\\d+).*`.",
    )
}

fn get_code(mut stream: TcpStream) -> Result<String> {
    // Reading the request for the redirect URI
    let mut buf = [0u8; 4096];
    stream.read(&mut buf)?;
    let req_str = String::from_utf8_lossy(&buf);
    info!("{}", req_str);

    // Returning some basic HTML
    let response = b"HTTP/1.1 200 OK
Content-Type: text/html; charset=UTF-8

<html><body>Authentication complete! Please go back to Vidify</body></html>";
    stream.write(response)?;

    stream.shutdown(Shutdown::Both)?;
    Ok(String::from(""))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn correct_bind_format() {
        assert_eq!(to_bind_format("http://localhost:0000"), "localhost:0000",);
        assert_eq!(to_bind_format("https://localhost:1234"), "localhost:1234",);
        assert_eq!(
            to_bind_format("http://localhost:8888/callback/"),
            "localhost:8888",
        );
        assert_eq!(
            to_bind_format("http://localhost:0/callback/"),
            "localhost:0",
        );
    }

    #[test]
    #[should_panic]
    fn incorrect_bind_format() {
        to_bind_format("localhost:8888");
    }
}
