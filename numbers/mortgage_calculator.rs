use std::num;

fn calculate_mortgage(amount :u64, interest :f64, years :u64) -> f64{
    let months: uint = (years as f64 * 12f64).ceil() as uint;

    let r = interest / 12f64;
    let a = amount as f64 * (r / 100f64 * num::pow(1f64+r/100f64, months)) / (num::pow(1f64+r/100f64, months) - 1f64);
    a
}

fn main() {
    println!("Monthly Payment: {}", calculate_mortgage(1000, 3.4f64, 2));
}
