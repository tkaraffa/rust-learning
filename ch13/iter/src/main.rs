fn main() {
    let v = Numero { value: 2 };
    println!("{}", v.square());
}
pub trait RaiseToPower {
    fn square(&self) -> i64;
}
pub struct Numero {
    pub value: i64,
}

impl RaiseToPower for Numero {
    fn square(&self) -> i64 {
        self.value * self.value
    }
}
