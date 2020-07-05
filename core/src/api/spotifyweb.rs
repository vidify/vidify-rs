use crate::api::APIBase;

pub struct SpotifyWeb {}

impl APIBase for SpotifyWeb {
    fn new() -> Self {
        SpotifyWeb {}
    }

    fn connect(&mut self) {}

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
