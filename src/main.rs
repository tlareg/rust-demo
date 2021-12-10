// use std::{collections::HashMap, fs::read_to_string};

fn main() {
    // greet("World".to_owned());

    // readme_fn();

    // print_type_of(&"Hi!");
    // print_type_of(&String::new());

    // vec_example();
    // hash_map_example();

    struct_example();
}

// fn greet(target: String) {
//     println!("Hello, {}", target);
// }

// fn readme_fn() {
//     let source = read_to_string("./README.md").unwrap();
//     let mut files_map = HashMap::new();

//     files_map.insert("README", source.clone());
//     files_map.insert("README2", source);

//     let files_ref = &files_map;
//     let files_ref2 = &files_map;

//     print_borrowed_map(files_ref);
//     print_borrowed_map(files_ref2);

//     let files_ref3 = &mut files_map;

//     // error: cannot borrow `files_map` more than once at a time
//     // let files_ref4 = &mut files_map;
//     needs_mutable_ref(files_ref3);

//     let files_ref4 = &mut files_map;
//     needs_mutable_ref(files_ref4);
// }

// fn print_borrowed_map(map: &HashMap<&str, String>) {
//     println!("{:?}", map)
// }

// fn needs_mutable_ref(_map: &mut HashMap<&str, String>) {}

// fn print_type_of<T>(_: &T) {
//     println!("Type is: {}", std::any::type_name::<T>())
// }

// fn vec_example() {
//     let mut numbers = vec![1, 2, 3, 4, 5]; // ⬅ Notice the vec! macro
//     numbers.push(7);
//     println!("{:?}", numbers);
// }

// fn hash_map_example() {
//   let mut map = HashMap::new();
//   map.insert("key1", "value1");
//   map.insert("key2", "value2");

//   println!("{:?}", map.get("key1"));
//   println!("{}", map.get("key2").unwrap_or(&""));
// }

fn struct_example() {
    let light = TrafficLight::new();
    println!("{}", light);
    println!("{:?}", light);
}

impl std::fmt::Display for TrafficLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Traffic light is {}", self.color)
    }
}

#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn new() -> Self {
        Self {
            color: "red".to_owned(),
        }
    }
}
