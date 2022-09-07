// ./src/variable_bindings/mut.md


fn part0() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error!
    // _immutable_binding += 1;
    // FIXME ^ Comment out this line
}

pub fn main() {
	part0();
}

