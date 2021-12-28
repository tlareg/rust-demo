use std::fmt::{Display, Formatter, Debug, Result};
use super::Light;

#[derive(Debug)]
pub struct HouseLight {
    on: bool,
}

impl HouseLight {
    pub fn new() -> Self {
        Self { on: false }
    }
}

impl Display for HouseLight {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Houselight is {}", if self.on { "on" } else { "off" })
    }
}


impl Light for HouseLight {
    fn get_name(&self) -> &str {
        "House light"
    }

    fn get_state(&self) -> &dyn Debug {
        &self.on
    }
}
