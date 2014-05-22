fn main() {
    let numbers = ~[1,2,3];

    let (tx, rx)  = channel();
    tx.send(numbers);

    spawn(proc() {
        let numbers = rx.recv();
        println!("{}", numbers[2]);
    })
}