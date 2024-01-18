// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// Make the tests module public
pub mod tests {
    pub fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    pub fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}

fn main() {
    // Call the public test functions
    tests::simple_option();
    tests::layered_option();

    println!("All tests passed!");
}
