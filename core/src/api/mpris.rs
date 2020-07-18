use crate::api::APIBase;
use crate::config::Config;
use crate::error::Result;

use dbus;

pub struct MPRIS {}

impl APIBase for MPRIS {
    fn new(config: &Config) -> Result<Self> {
        Ok(MPRIS {})
    }

    fn get_player_name(&self) -> &str {
        "test"
    }

    fn get_artist(&self) -> Option<&str> {
        None
    }

    fn get_title(&self) -> Option<&str> {
        None
    }

    fn get_position(&self) -> Option<u32> {
        None
    }

    fn is_playing(&self) -> bool {
        true
    }

    fn event_loop(&mut self) {
        unimplemented!();
    }
}
