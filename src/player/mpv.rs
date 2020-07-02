use crate::player::PlayerBase;

use libmpv;

pub struct Mpv {
}

impl PlayerBase for Mpv {
    fn new() -> Mpv {
        Mpv {}
    }

    fn pause(&mut self) { }
    fn is_paused(&self) -> bool { true }
    fn get_position(&self) -> u32 { 123 }
    fn seek(&mut self) { }
    fn start_video(&mut self) { }
}
