use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct MyNumber(i32);

impl Add for MyNumber {
    type Output = Self;
    fn add(self, other: MyNumber) -> Self {
        MyNumber(self.0 + other.0)
    }
}

fn main() {
    let num1 = MyNumber(10);
    let num2 = MyNumber(20);

    let sum = num1 + num2;
    println!("sum: {:?}", sum); 
}