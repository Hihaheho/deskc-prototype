use bevy::prelude::*;
use plugins::*;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(physics::PhysicsPlugin)
        .add_plugin(card::CardPlugin)
        .add_plugins(language::LanguagePlugins)
        .add_plugins(egui::EguiPlugins)
        .run();
}
