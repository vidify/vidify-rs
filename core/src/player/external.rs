use crate::player::PlayerBase;

pub struct External {}

impl PlayerBase for External {
    fn new() -> External {
        External {}
    }

    fn pause(&mut self) {}
    fn is_paused(&self) -> bool {
        true
    }
    fn get_position(&self) -> u32 {
        123
    }
    fn seek(&mut self) {}
    fn start_video(&mut self) {}
}
