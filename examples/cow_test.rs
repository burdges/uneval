
// extern crate serde;
// extern crate serde_json;
// extern crate uneval;

use serde::{Serialize, Deserialize};

use std::borrow::Cow;

#[derive(Serialize, Deserialize, Debug)]
struct Point<'a> {
    #[serde(borrow)]
    a: Cow<'a,u32>,
    #[serde(borrow)]
    b: Cow<'a,str>,
    c: Option<String>,
    d: Result<(),()>
}

fn main() {
    let point = Point { a: Cow::Owned(6), b: Cow::Borrowed("hello"), c: None, d: Err(()) };

    let unevaled = uneval::funcs::to_string(&point).unwrap();
    println!("serialized = {}", unevaled );

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
}
