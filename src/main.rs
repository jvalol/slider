mod checker;
mod input;
mod ring;
mod run;
mod ship;
mod slider_game;
mod tunnel;

use blitzkit::start;
use slider_game::SliderGame;

/// Whether this run is only here to be photographed, for `refresh-screenshots`
/// in the project above.
pub fn staged() -> bool {
    std::env::args().any(|arg| arg == "--screenshot")
}

fn main() {
    start("slider", Box::new(SliderGame::new()));
}
