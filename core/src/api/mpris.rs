use crate::api::APIBase;
use crate::config::Config;
use crate::error::{Result, Error};

use std::time;

use mpris::{Player, PlayerFinder};

impl From<mpris::DBusError> for Error {
    fn from(err: mpris::DBusError) -> Self {
        Error::FailedConnection(err.to_string())
    }
}

impl From<mpris::FindingError> for Error {
    fn from(err: mpris::FindingError) -> Self {
        Error::FailedConnection(err.to_string())
    }
}

pub struct MPRIS<'a> {
    player: Player<'a>,
}

// TODO: check `player.can_play` and similars?
impl<'a> APIBase for MPRIS<'a> {
    fn new(config: &Config) -> Result<Self> {
        let player = PlayerFinder::new()?.find_active()?;

        Ok(MPRIS {
            player
        })
    }

    fn player_name(&self) -> String {
        self.player.bus_name().to_string()
    }

    fn artist(&self) -> Option<String> {
        self.player
            .get_metadata()
            .ok()?
            .album_artists()?
            .clone()
            .into_iter()
            .next()
    }

    fn title(&self) -> Option<String> {
        Some(
            self.player
                .get_metadata()
                .ok()?
                .title()?
                .to_string()
        )
    }

    // TODO: return std::time::Duration, u128 or a more appropiate data type
    // to avoid `as`.
    fn position(&self) -> Option<time::Duration> {
        self.player.get_position().ok()
    }

    fn is_playing(&self) -> bool {
        self.player.is_running()
    }

    fn event_loop(&mut self) {
        unimplemented!();
    }
}
