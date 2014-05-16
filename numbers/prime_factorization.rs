fn is_prime(num :f64) -> bool {
    for i in range(2, num.sqrt() as u64) {
        if num as u64 % i == 0 {
            return false
        }
    }
    true
}

fn fatorize_prime(num :f64) {
    for i in range(2, num.sqrt() as u64) {
        if num as u64 % i == 0 && is_prime(i as f64) {
            println!("{}", i);
        }
    }
}

#[test]
fn is_prime_test() {
    assert!(is_prime(4) == false, "{} {} {}", is_prime(4), false, 4);
}

#[test]
fn is_prime_test2() {
    assert!(is_prime(5) == true, "{} {} {}", is_prime(5), true, 5);
}

fn main() {
    fatorize_prime(233237766723323232f64);
}