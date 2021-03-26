use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x:1, y: 2 };

    // Convert the Point to a JSON string.
    let serizlized = serde_json::to_string(&point).unwrap();

    println!("serizlized = {}", serizlized);

    // Convert the JSON string to a Point.
    let deserialized: Point = serde_json::from_str(&serizlized).unwrap();

    println!("deserialized = {:?}", deserialized);
}
