fn is_prime(num :f64) -> bool {
    for i in range(2, (num.sqrt() as uint + 1)) {
        if num as uint % i == 0 {
            return false
        }
    }
    true
}

fn fatorize_prime(num :f64) {
    for i in range(2, (num.sqrt() as uint + 1)) {
        if num as uint % i == 0 && is_prime(i as f64) {
            println!("{}", i);
        }
    }
}

#[test]
fn is_prime_test() {
    assert!(is_prime(4f64) == false, "is {} expected {} {}", is_prime(4f64), false, 4);
    assert!(is_prime(5f64) == true, "is {} expected {} {}", is_prime(5f64), true, 5);
    assert!(is_prime(9f64) == false, "is {} expected {} {}", is_prime(9f64), false, 9);
}

fn main() {
    fatorize_prime(233237766723323232f64);
}