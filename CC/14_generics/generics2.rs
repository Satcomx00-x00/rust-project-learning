// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.
//
// Execute `rustlings hint generics2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE


// en utilisant <T> on peut utiliser n'importe quel type dans le wrapper (i32, String, etc) et donc pas seulement u32
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}

fn main() {
    let wrapped_number = Wrapper::new(42);
    let wrapped_string = Wrapper::new("Rust is fun!");

    println!("Wrapped number: {}", wrapped_number.value);
    println!("Wrapped string: {}", wrapped_string.value);
}