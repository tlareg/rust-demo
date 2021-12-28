use std::collections::HashMap;

pub fn vec_example() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.push(7);
    println!("{:?}", numbers);
}

pub fn hash_map_example() {
    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("{:?}", map.get("key1"));
    println!("{}", map.get("key2").unwrap_or(&""));
}
