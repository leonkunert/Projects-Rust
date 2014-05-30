/*fn fibbonacci(num :u64) {
    let mut _curr:u64 = 1;
    let mut prev:u64 = 0;
    let mut _cont:u64 = 0;
    while _curr < num {
        println!("{}, ", prev);
        _cont = _curr;
        _curr = _curr + prev;
        prev = _cont;
    }
}
*/
fn fibbonacci_func_caller(num :u64) {
    fibbonacci_func(num, 0, 1);
}

fn fibbonacci_func(num :u64, prev :u64, curr :u64) {
    println!("{}, ", prev);
    if prev >= num {
        return;
    }
    fibbonacci_func(num, curr, (prev+curr));
}

fn main() {
    fibbonacci_func_caller(10000000000000000000);
}