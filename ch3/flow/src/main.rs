fn fib(n: u32) -> u32 {
    let mut fib_past_2 = 2;
    let result = if n == 1 || n == 2 {
        1
    } else {
        for _ in 3..n {
            let last_fib = fib_past_2;
            fib_past_2 += last_fib;
        }
        fib_past_2
    };

    result
}

fn main() {
    for n in 1..10 {
        let f = fib(n);
        println!("The {n}th fib is {f}");
    }
}
