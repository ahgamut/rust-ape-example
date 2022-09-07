// ./src/expression.md


fn part0() {
    // statement
    // statement
    // statement
}

fn part1() {
    // variable binding
    let x = 5;

    // expression;
    x;
    x + 1;
    15;
}

fn part2() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    let z = {
        // The semicolon suppresses this expression and `()` is assigned to `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

pub fn main() {
	part0();
	part1();
	part2();
}

