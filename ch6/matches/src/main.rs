fn add_one(i: i32) -> i32 {
    i + 1
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(add_one(i)),
        other => todo!(),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{}, {}", five.unwrap(), six.unwrap());
}
