use std::fs::File;

use core::api::{init_api, API, APIEvent};
use core::player::{init_player, Player};
use core::config::{init_config, Config};
use core::data::{Res, ResKind};
use log::info;

use gtk::{
    Inhibit,
    LabelExt,
    OrientableExt,
    WidgetExt,
};
use gtk::Orientation::Vertical;
use relm::{Channel, Relm, Widget};
use relm_derive::{Msg, widget};

use self::Msg::*;

extern {
    fn gdk_x11_window_get_xid(window: gdk::Window) -> u32;
}

fn main() {
    Win::run(()).expect("Win::run failed");
}

pub struct Model {
    _channel: Channel<APIEvent>,
    text: String,
    config: Config,
}

#[derive(Clone, Msg)]
pub enum Msg {
    Quit,
    PlayerUpdate(APIEvent),
}

#[widget]
impl Widget for Win {
    fn init_view(&mut self) {
        // Initializing the Player.
        let wind = self.player_frame.get_window().unwrap();
        let xid = unsafe {gdk_x11_window_get_xid(wind)};

        let player = init_player(
            Player::Mpv,
            &init_config().unwrap(),
            xid.into()
        );
    }

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        // Initializing the config file and the logging systems.
        let config = init_config().unwrap();
        info!("Config loaded: {:#?}", config);
        Self::init_logging(&config);
        info!("Logging initialized");

        // The stream will communicate with the API messages with the GUI.
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |event| {
            // Dispatching the received messages to the GUI.
            stream.emit(PlayerUpdate(event));
        });

        // Initializing the API.
        // TODO: this shouldn't be done at the beginning.
        // TODO: use the GUI to have the API fallback.
        let api = init_api(
            config.api.clone().unwrap_or(API::SpotifyWeb),
            &config,
            sender,
        );

        // // Initializing the Player.
        // let player = init_player(
        //     config.player.clone().unwrap_or(Player::Mpv),
        //     &config,
        //     self.player_frame.get_xid()
        // );

        // Debugging
        match api {
            Ok(api) => {
                println!("API data:");
                println!("    Player name: {}", api.player_name());
                println!("    Artist: {:?}", api.artist());
                println!("    Title: {:?}", api.title());
                println!("    Position: {:?}", api.position());
                println!("    Is playing?: {}", api.is_playing());
            }
            Err(e) => eprintln!("Error loading API: {}", e),
        }

        Model {
            _channel: channel,
            text: "Computing...".to_string(),
            config,
        }
    }

    fn init_logging(config: &Config) {
        use simplelog::{
            CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode,
            WriteLogger,
        };

        let file = Res::new(ResKind::Data, "session.log")
            .expect("Couldn't load log file");
        let file: &str = &file;
        CombinedLogger::init(vec![
            TermLogger::new(
                if config.debug {
                    LevelFilter::Trace
                } else {
                    LevelFilter::Off
                },
                Config::default(),
                TerminalMode::Stderr,
            ),
            WriteLogger::new(
                LevelFilter::Trace,
                Config::default(),
                File::open(file).unwrap(),
            ),
        ])
        .expect("Couldn't load loggers");
    }

    fn update(&mut self, event: Msg) {
        match event {
            Quit => gtk::main_quit(),
            PlayerUpdate(event) => {
                info!("got event: {:?}", event);
                self.model.text = format!("{:?}", event);
            },
        }
    }

    view! {
        gtk::Window {
            gtk::Box {
                orientation: Vertical,
                gtk::Label {
                    text: &self.model.text,
                },
                #[name="player_frame"]
                gtk::Frame {
                }
            },
            delete_event(_, _) => (Quit, Inhibit(false)),
        }
    }
}
