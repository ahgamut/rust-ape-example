// ./src/conversion/from_into.md


use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn part0() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
}

fn part1() {
    let int = 5;
    // Try removing the type declaration
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

pub fn main() {
	part0();
	part1();
}

