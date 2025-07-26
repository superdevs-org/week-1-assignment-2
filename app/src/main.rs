use std::fmt::Error;
use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct, Debug)]
struct Swap {
    qty_1: u32,
    qty_2: i32,
    token_1: String,
}

fn main() {
    println!("Hello, world!");
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        token_1: "SOasdadL".to_string(),
    };
    let bytes = s.serialize();
    println!("{:?}", bytes);
    let original = Swap::deserialize(&bytes).unwrap();
    println!("{:?}", original);
}