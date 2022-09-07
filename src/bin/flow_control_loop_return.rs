// ./src/flow_control/loop/return.md


fn part0() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

pub fn main() {
	part0();
}

