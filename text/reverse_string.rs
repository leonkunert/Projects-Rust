
fn reverse_string(string :&str) -> StrBuf {
    string.chars().rev().collect()
}

#[test]
fn reverse_string_test(){
    assert!(reverse_string("hallo") == StrBuf::from_str("ollah"), "expected {} is {}", StrBuf::from_str("ollah"), reverse_string("hallo"));
}

#[test]
fn reverse_string_test_unicode(){
    assert!(reverse_string("世界") == StrBuf::from_str("界世"), "expected {} is {}", StrBuf::from_str("界世"), reverse_string("世界"));
}

fn main() {
    println!("{}",reverse_string("fdsfsd"));
}