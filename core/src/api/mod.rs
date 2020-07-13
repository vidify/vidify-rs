pub mod macos;
pub mod mpris;
pub mod spotifyweb;
pub mod windows;

use crate::config::Config;
use crate::error::Result;

use strum_macros::{Display, EnumString};

#[derive(Debug, Display, EnumString)]
pub enum API {
    #[cfg(any(target_os = "linux", target_os = "bsd"))]
    MPRIS,
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "macos")]
    MacOS,
    SpotifyWeb,
}

// TODO: Relevant book section:
// https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html
// With this design pattern, the API can make sure the rest of the methods
// can't be used before `connect` is called.
//
// TODO: could `connect` and `new` be merged into `new`?
//
// TODO: Relevant book section:
// https://doc.rust-lang.org/book/ch17-02-trait-objects.html#object-safety-is-required-for-trait-objects
// Using `new` breaks object-safe rules unless `Sized` is used, so it's
// not considered a good practise.
pub trait APIBase {
    // Creating the object and initializing it
    fn new(config: &Config) -> Result<Self>
    where
        Self: Sized;

    // Obtaining data about the API
    fn get_player_name(&self) -> &str;
    fn get_artist(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_position(&self) -> u32;
    fn is_playing(&self) -> bool;
    fn event_loop(&mut self);
}

pub fn init_api(api: API, config: &Config) -> Result<&dyn APIBase> {
    let api: &dyn APIBase = match api {
        #[cfg(any(target_os = "linux", target_os = "bsd"))]
        API::MPRIS => &mpris::MPRIS::new(config)?,
        #[cfg(target_os = "windows")]
        API::Windows => &windows::Windows::new(config)?,
        #[cfg(target_os = "macos")]
        API::MacOS => &macos::MacOS::new(config)?,
        API::SpotifyWeb => &spotifyweb::SpotifyWeb::new(config)?,
    };

    Ok(api)
}
