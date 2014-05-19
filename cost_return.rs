
fn calculate_return(cost :f64, given :f64) -> f64 {
    return given - cost
}

#[test]
fn calculate_return_test(){
    assert!(calculate_return(13.23f64, 15.00f64) == 1.77f64, "expected {:?} is {:?}", 1.77f64, calculate_return(13.23f64, 15.00f64));
}

#[test]
fn calculate_return_test2(){
    assert!(calculate_return(132.23f64, 150.00f64) == 17.77f64, "expected {:?} is {:?}", 17.77f64, calculate_return(132.23f64, 150.00f64));
}

fn main() {
    println!("{}",calculate_return(123.45, 130.00));
}
