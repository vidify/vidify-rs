use crate::api::APIBase;
use crate::config::Config;
use crate::error::Result;

pub struct MacOS {}

impl APIBase for MacOS {
    fn new(config: &Config) -> Result<Self> {
        Ok(MacOS {})
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
