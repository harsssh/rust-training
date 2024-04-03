fn greet_map(id: i32, name: &str) -> std::collections::HashMap<i32, String> {
    let mut map = std::collections::HashMap::new();
    let value = format!("Hello, {name}");
    map.insert(id, value);
    map
}

fn main() {
    let map = greet_map(1, "Alice");
    println!("{:?}", map)
}
