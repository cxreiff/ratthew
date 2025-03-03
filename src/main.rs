use bevy::app::App;
use dotenv::dotenv;
use tui_logger::{init_logger, set_default_level, LevelFilter};

fn main() {
    dotenv().ok();
    init_logger(LevelFilter::Info).unwrap();
    set_default_level(LevelFilter::Info);

    App::new().add_plugins(ratthew::plugin).run();
}
