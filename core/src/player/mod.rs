pub mod mpv;
pub mod external;

#[derive(Debug, Display, EnumString)]
pub enum Player {
    Mpv,
    External,
}

pub struct PlayerData {
    description: String,
    icon: String, // TODO maybe an enum from the resources module
}

pub trait PlayerBase {
    fn new() -> Self
        where Self: Sized;

    fn pause(&mut self);
    fn is_paused(&self) -> bool;
    fn get_position(&self) -> u32;
    fn seek(&mut self);
    fn start_video(&mut self);
}
