// mod string;
// use string::{greet, print_type_of};

// mod readme;
// use readme::readme_fn;

// mod collections_examples;
// use collections_examples::{vec_example, hash_map_example};

// mod struct_examples;
// use struct_examples::lights_example;

// mod options_examples;
// use options_examples::options_example;

mod error_handling;
use error_handling::error_handling_examples;

fn main() {
    // greet("World".to_owned());
    // print_type_of(&"Hi!");
    // print_type_of(&String::new());

    // readme_fn();

    // vec_example();
    // hash_map_example();

    // lights_example();

    // options_example();

    error_handling_examples();
}


