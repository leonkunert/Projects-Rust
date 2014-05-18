use std::io;

fn to_interger(string :~str) -> int {
    println!("{} {}", string, from_str::<int>(string));
    match from_str::<int>(string) {
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