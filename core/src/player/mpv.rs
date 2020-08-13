use crate::config::Config;
use crate::error::Result;
use crate::player::PlayerBase;

use libmpv;

pub struct Mpv {}

impl PlayerBase for Mpv {
    fn new(config: &Config) -> Result<Mpv> {
        Ok(Mpv {})
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
