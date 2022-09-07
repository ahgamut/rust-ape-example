// ./src/scope/lifetime/trait.md


// A struct with annotation of lifetimes.
#[derive(Debug)]
struct Borrowed<'a> {
    x: &'a i32,
}

// Annotate lifetimes to impl.
impl<'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self {
            x: &10,
        }
    }
}

fn part0() {
    let b: Borrowed = Default::default();
    println!("b is {:?}", b);
}

pub fn main() {
	part0();
}

