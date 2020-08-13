use crate::config::Config;
use crate::error::Result;
use crate::player::PlayerBase;

pub struct External {}

impl PlayerBase for External {
    fn new(config: &Config) -> Result<Self> {
        Ok(External {})
    }

    fn pause(&mut self) {}

    fn is_paused(&self) -> bool {
        true
    }

    fn position(&self) -> u32 {
        123
    }

    fn seek(&mut self) {}

    fn start_video(&mut self) {}
}
