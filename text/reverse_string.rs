
fn reverse_string(string :~str) -> ~str{
    string.chars_rev().collect()
}

#[test]
fn reverse_string_test(){
    assert!(reverse_string(~"hallo") == ~"ollah", "expected {} is {}", ~"ollah", reverse_string(~"hallo"));
}

#[test]
fn reverse_string_test_unicode(){
    assert!(reverse_string(~"世界") == ~"界世", "expected {} is {}", ~"界世", reverse_string(~"世界"));
}

fn main() {
    println!("{}",reverse_string(~"fdsfsd"));
}