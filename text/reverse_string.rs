
fn reverse_string(string :&str) -> String {
    string.chars().rev().collect()
}

#[test]
fn reverse_string_test(){
    assert!(reverse_string("hallo") == String::from_str("ollah"), "expected {} is {}", String::from_str("ollah"), reverse_string("hallo"));
}

#[test]
fn reverse_string_test_unicode(){
    assert!(reverse_string("世界") == String::from_str("界世"), "expected {} is {}", String::from_str("界世"), reverse_string("世界"));
}

fn main() {
    println!("{}",reverse_string("ollah"));
}