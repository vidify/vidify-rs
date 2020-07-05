pub mod macos;
pub mod mpris;
pub mod spotifyweb;
pub mod windows;

pub enum API {
    #[cfg(any(target_os = "linux", target_os = "bsd"))]
    MPRIS,
    #[cfg(target_os = "windows")]
    Windows,
    #[cfg(target_os = "macos")]
    MacOS,
    SpotifyWeb,
}

pub struct APIData {
    description: String,
    icon: String, // TODO maybe an enum from the resources module
    connect_msg: String,
    event_loop_interval: String,
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
    fn new() -> Self
        where Self: Sized;
    fn connect(&mut self);

    // Obtaining data about the API
    fn get_player_name(&self) -> &str;
    fn get_artist(&self) -> &str;
    fn get_title(&self) -> &str;
    fn get_position(&self) -> u32;
    fn is_playing(&self) -> bool;
    fn event_loop(&mut self);
}

pub fn init_api(api: API) -> Box<dyn APIBase> {
    match api {
        #[cfg(any(target_os = "linux", target_os = "bsd"))]
        API::MPRIS => Box::new(mpris::MPRIS::new()),
        #[cfg(target_os = "windows")]
        API::Windows => Box::new(windows::Windows::new()),
        #[cfg(target_os = "macos")]
        API::MacOS => Box::new(macos::MacOS::new()),
        API::SpotifyWeb => Box::new(spotifyweb::SpotifyWeb::new())
    }
}