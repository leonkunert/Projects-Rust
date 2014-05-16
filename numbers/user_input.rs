use std::io;

fn main() {
    for line in io::stdin().lines() {
        print!("{}", line.unwrap());
    }
}