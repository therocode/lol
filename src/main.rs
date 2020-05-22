fn fibonacci(number: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    for _ in 0..number {
        let tmp = a;
        a = b;
        b = a + tmp;
    }

    b
}

fn main() {
    println!("10th fibonacci is {}", fibonacci(10));
}
