use super::Light;
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug)]
enum TrafficLightColor {
    Red,
    Yellow,
    Green,
}

impl Display for TrafficLightColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let color_string = match self {
            TrafficLightColor::Green => "green",
            TrafficLightColor::Red => "red",
            TrafficLightColor::Yellow => "yellow",
        };
        write!(f, "{}", color_string)
    }
}

#[derive(Debug)]
pub struct TrafficLight {
    color: TrafficLightColor,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: TrafficLightColor::Red,
        }
    }

    pub fn turn_green(&mut self) {
        self.color = TrafficLightColor::Green
    }

    pub fn turn_yellow(&mut self) {
        self.color = TrafficLightColor::Yellow
    }

    pub fn turn_red(&mut self) {
        self.color = TrafficLightColor::Red
    }
}

impl Display for TrafficLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Traffic light is {}", self.color)
    }
}

impl Light for TrafficLight {
    fn get_name(&self) -> &str {
        "Traffic light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.color
    }
}
