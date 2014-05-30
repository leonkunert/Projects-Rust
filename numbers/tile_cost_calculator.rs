extern crate debug;

use std::io;

fn to_interger(string :String) -> int {
    match from_str::<int>(string.as_slice().trim()) {
        Some(0) => 0,
        Some(x) => x,
        _       => 0
    }
}

fn main() {
    println!("Enter width of the room:");
    let width = to_interger(io::stdin().read_line().unwrap());
    println!("Enter depth of the room:");
    let depth = to_interger(io::stdin().read_line().unwrap());
    println!("Enter price of sqrm of tiles:");
    let tile_price = to_interger(io::stdin().read_line().unwrap());

    let price = width * depth * tile_price;

    println!("The Price for the tiles would be: {}€", price);
}