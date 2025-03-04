use std::fmt::Debug;

use tracing::{debug, info};

enum Event {
    ButtonPressed(Button),
    CarArrived(Floor),
    CarDoorOpened(Floor),
    CarDoorClosed(Floor),
}

#[derive(Debug)]
enum Button {
    LobbyCall(Direction, Floor),
    CarFloor(Floor),
}

type Floor = i32;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
}

pub fn elevator_run() {
    info!("run in elevator mod");
    info!(
        "{:?}",
        Event::ButtonPressed(Button::LobbyCall(Direction::UP, 1))
    );
    info!("{:?}", Event::CarDoorOpened(2));
    info!("{:?}", Event::CarDoorClosed(2));
    info!("{:?}", Event::CarDoorOpened(1));
    info!("{:?}", Event::CarDoorClosed(1));
    info!("{:?}", Event::ButtonPressed(Button::CarFloor(0)));
    info!("{:?}", Event::CarDoorOpened(12));
    debug!("{:?}", Event::CarDoorClosed(12));
}

impl Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base_str = "evelator event: ";
        write!(
            f,
            "{:?}",
            match self {
                Event::ButtonPressed(Button::LobbyCall(direction, floor)) => format!(
                    "{}LobbyCall -> direction: {:?} at floor: {}",
                    base_str, direction, floor
                ),
                Event::ButtonPressed(Button::CarFloor(floor)) =>
                    format!("{}FloorCall at {} floor", base_str, floor),
                Event::CarArrived(floor) => format!("{}arrived at {} floor", base_str, floor),
                Event::CarDoorOpened(floor) => format!("{}opened at {} floor", base_str, floor),
                Event::CarDoorClosed(floor) => format!("{}closed at {} floor", base_str, floor),
            }
        )
    }
}
