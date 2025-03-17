use bevy::{
    app::App,
    log::tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt},
};

pub(super) fn plugin(_app: &mut App) {
    tracing_subscriber::registry()
        .with(Some(tui_logger::tracing_subscriber_layer()))
        .init();
    tui_logger::init_logger(tui_logger::LevelFilter::Info).unwrap();
}
