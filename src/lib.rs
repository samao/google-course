mod elevator;

pub use elevator::elevator_run;

mod player;

pub use player::player_run;

mod trait_api;

pub use trait_api::*;

mod math_agi;

pub use math_agi::*;

mod generics;

pub use generics::*;

mod std_api;
pub use std_api::*;

pub fn math_agi_run_proxy() {
    math_agi_run();
}
