use std::{collections::HashMap, fs::read_to_string, fmt::{Display, Formatter, Result}};

fn main() {
    // greet("World".to_owned());

    // readme_fn();

    // print_type_of(&"Hi!");
    // print_type_of(&String::new());

    // vec_example();
    // hash_map_example();

    struct_example();
}

fn greet(target: String) {
    println!("Hello, {}", target);
}

fn readme_fn() {
    let source = read_to_string("./README.md").unwrap();
    let mut files_map = HashMap::new();

    files_map.insert("README", source.clone());
    files_map.insert("README2", source);

    let files_ref = &files_map;
    let files_ref2 = &files_map;

    print_borrowed_map(files_ref);
    print_borrowed_map(files_ref2);

    let files_ref3 = &mut files_map;

    // error: cannot borrow `files_map` more than once at a time
    // let files_ref4 = &mut files_map;
    needs_mutable_ref(files_ref3);

    let files_ref4 = &mut files_map;
    needs_mutable_ref(files_ref4);
}

fn print_borrowed_map(map: &HashMap<&str, String>) {
    println!("{:?}", map)
}

fn needs_mutable_ref(_map: &mut HashMap<&str, String>) {}

fn print_type_of<T>(_: &T) {
    println!("Type is: {}", std::any::type_name::<T>())
}

fn vec_example() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(7);
    println!("{:?}", numbers);
}

fn hash_map_example() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get("key1"));
    println!("{}", map.get("key2").unwrap_or(&""));
}

fn struct_example() {
    // let mut light = TrafficLight::new();
    // println!("{}", light);
    // println!("{:?}", light);

    // println!("State: {}", light.get_state());
    // light.turn_green();
    // println!("State: {}", light.get_state());

    let mut traffic_light = TrafficLight::new();
    let mut house_light = HouseLight::new();

    print_light_state(&traffic_light);
    print_light_state(&house_light);
}

fn print_light_state(light: &impl Light) {
  println!("{}'s state is : {:?}", light.get_name(), light.get_state());
}

#[derive(Debug)]
struct TrafficLight {
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
}

impl std::fmt::Display for TrafficLight {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    write!(f, "Traffic light is {}", self.color)
  }
}

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
struct HouseLight {
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

trait Light {
  fn get_name(&self) -> &str;
  fn get_state(&self) -> &dyn std::fmt::Debug;
}

impl Light for HouseLight {
  fn get_name(&self) -> &str {
    "House light"
  }

  fn get_state(&self) -> &dyn std::fmt::Debug {
    &self.on
  }
}

impl Light for TrafficLight {
  fn get_name(&self) -> &str {
    "Traffic light"
  }

  fn get_state(&self) -> &dyn std::fmt::Debug {
    &self.color
  }
}
