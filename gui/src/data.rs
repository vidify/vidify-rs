/// Should be linked 1:1 to core::API and core::Player.
pub struct APIData {
    description: String,
    icon: String, // TODO maybe an enum from a resources module
    connect_msg: String,
    event_loop_interval: String,
}

/// Should be linked 1:1 to core::API and core::Player.
pub struct PlayerData {
    description: String,
    icon: String, // TODO maybe an enum from a resources module
}
