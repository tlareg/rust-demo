pub fn greet(target: String) {
  println!("Hello, {}", target);
}

pub fn print_type_of<T>(_: &T) {
  println!("Type is: {}", std::any::type_name::<T>())
}
