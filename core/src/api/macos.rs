use crate::api::APIBase;
use crate::config::Config;
use crate::error::Result;

pub struct MacOS {}

impl APIBase for MacOS {
    fn new(config: &Config) -> Result<Self> {
        Ok(MacOS {})
    }

    fn get_player_name(&self) -> &str {
        "Mac OS"
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
