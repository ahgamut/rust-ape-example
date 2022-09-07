// ./src/flow_control/if_let.md


fn part0() {
    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // Provide an altered failing condition.
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
}

// Our example enum
enum Foo1 {
    Bar,
    Baz,
    Qux(u32)
}

fn part1() {
    // Create example variables
    let a = Foo1::Bar;
    let b = Foo1::Baz;
    let c = Foo1::Qux(100);
    
    // Variable a matches Foo::Bar
    if let Foo1::Bar = a {
        println!("a is foobar");
    }
    
    // Variable b does not match Foo::Bar
    // So this will print nothing
    if let Foo1::Bar = b {
        println!("b is foobar");
    }
    
    // Variable c matches Foo::Qux which has a value
    // Similar to Some() in the previous example
    if let Foo1::Qux(value) = c {
        println!("c is {}", value);
    }

    // Binding also works with `if let`
    if let Foo1::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}

// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo2 {Bar}

fn part2() {
    let a = Foo2::Bar;

    // Variable a matches Foo::Bar
    if let Foo2::Bar = a {
    // ^-- this causes a compile-time error. Use `if let` instead.
        println!("a is foobar");
    }
}

pub fn main() {
	part0();
	part1();
	part2();
}

