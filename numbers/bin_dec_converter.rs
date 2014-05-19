fn to_binary(num :int) {
    println!("{:t}", num)
}

fn to_decimal(num :int) {
    println!("{}", num);
}

fn main() {
    to_binary(200);
    to_decimal(0b11001000);
}
