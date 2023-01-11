fn main() {
    let s = String::from("hi");
    let s2 = test(s);
    println!("{}", s2);
}

fn test(s: String) -> Strinlapg {
    println!("{}", s);
    let s2 = s;
    s2
}
