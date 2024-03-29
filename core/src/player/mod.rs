pub mod external;
pub mod mpv;

use crate::config::Config;
use crate::error::Result;

use strum_macros::{Display, EnumString};

#[derive(Clone, Debug, Display, EnumString)]
pub enum Player {
    Mpv,
    External,
}

pub trait PlayerBase {
    fn new(config: &Config) -> Result<Self>
    where
        Self: Sized;

    fn pause(&mut self);
    fn is_paused(&self) -> bool;
    fn position(&self) -> u32;
    fn seek(&mut self);
    fn start_video(&mut self);
}
