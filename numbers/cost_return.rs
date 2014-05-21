
fn calculate_return(cost :f64, given :f64) -> f64 {
    return given - cost
}

#[test]
fn calculate_return_test(){
    assert_eq!(calculate_return(13.23f64, 15.00f64), 15f64-13.23f64);
}

#[test]
fn calculate_return_test2(){
    assert_eq!(calculate_return(132.23f64, 150.00f64), 150f64-132.23f64);
}

#[test]
fn calculate_return_test3(){
    assert_eq!(calculate_return(1f64, 2f64), 1f64);
}

fn main() {
    println!("{}",calculate_return(123.45, 130.00));
}
