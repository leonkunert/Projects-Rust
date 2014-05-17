use std::io;

fn is_prime(num :f64) -> bool {
    for i in range(2, (num.sqrt() as uint + 1)) {
        if num as uint % i == 0 {
            return false
        }
    }
    true
}

fn main() {
    let mut i = 2;
    for line in io::stdin().lines() {
        while !is_prime(i as f64) {
            i += 1;
        }
        print!("{}", i);
        i += 1;
    }
}