// ./src/scope/lifetime/static_lifetime.md


use std::fmt::Debug;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn part0() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);

    // oops, &i only has the lifetime defined by the scope of
    // part0(), so it's not 'static:
    // print_it(&i);
}

pub fn main() {
	part0();
}

