fn main() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        i @ 1..=5 => println!("{i} is between one and five"),
        _ => println!("anything"),
    }
}
