// ./src/error/panic.md


fn drink(beverage: &str) {
    // You shouldn't drink too much sugary beverages.
    if beverage == "lemonade" { panic!("AAAaaaaa!!!!"); }

    println!("Some refreshing {} is all I need.", beverage);
}

fn part0() {
    drink("water");
    drink("lemonade");
}

pub fn main() {
	part0();
}

