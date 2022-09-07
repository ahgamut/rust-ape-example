// ./src/attribute/cfg/custom.md

/*
#[cfg(some_condition)]*/
fn conditional_function() {
    println!("condition met!");
}

fn part0() {
    conditional_function();
}

pub fn main() {
	part0();
}

