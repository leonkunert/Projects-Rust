use std::f32::to_str_digits;
use std::num::atan;

fn get_pi(num :uint) ->~str {
    to_str_digits(16f32*atan(1f32/5f32)-4f32*atan(1f32/239f32), num)
}

/* Build Tests with rustc --test *file*.rs*/

#[test]
fn get_pi_test() {
    assert!(get_pi(12) == to_str_digits(3.1415927410125732421875f32, 12), "{} {}", get_pi(12), 3.141592741013);
    assert!(get_pi(0) == to_str_digits(3.1415927410125732421875f32, 0), "{} {}", get_pi(0), 3);
}

fn main() {
    let pi = get_pi(12);
    println!("{}", pi);
}
