use std::path::PathBuf;

use crate::util::game::init_game_path;

pub fn init_launcher() {
    init_game_path(PathBuf::from(".minecraft"))
        .expect("Failed to initialize game path");
}