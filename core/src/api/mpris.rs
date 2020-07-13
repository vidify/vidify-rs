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
