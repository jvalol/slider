mod checker;
mod input;
mod ring;
mod run;
mod ship;
mod slider_game;
mod tunnel;

use blitzkit::start;
use slider_game::SliderGame;

fn main() {
    start("slider", Box::new(SliderGame::new()));
}
