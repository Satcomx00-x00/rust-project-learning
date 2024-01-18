// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    // on peut acceder à un item du tuple en utilisant un point suivi de l'index de la valeur à laquelle on veut acceder
    let second = numbers.1;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}

fn main() {
    let numbers = (1, 2, 3);
    let second = numbers.1;

    println!("The second number is {}", second);
}