
fn is_prime(num :u64) -> bool {
    for i in range(2, num) {
        if num % i == 0 {
            return false
        }
    }
    true
}

fn fatorize_prime(num :u64) {
    for i in range(2, num) {
        if num % i == 0 && is_prime(i) {
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
    fatorize_prime(2332377667);
}