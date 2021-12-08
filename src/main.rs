use std::{collections::HashMap, fs::read_to_string};

fn main() {
    greet("World".to_owned());

    // readme_fn();

    // print_type_of(&"Hi!");
    // print_type_of(&String::new());
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
