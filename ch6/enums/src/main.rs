fn main() {
    let x: Option<u8> = Some(5);
    let y: Option<u8> = Some(5);
    let z = x.unwrap() + y.unwrap();

    println!("{}", z);
}
