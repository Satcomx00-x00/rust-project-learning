// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let mut data = "Rust is great!".to_string();  // Declare data as mutable

    get_char(&data);   // data est emprunté

    string_uppercase(&mut data);  // mutable et emprunté
}

// Should not take ownership
fn get_char(data: &String) -> char {     // String est emprunté
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &mut String) {    
    *data = data.to_uppercase();   // *data pointe vers la valeur de data (String)

    println!("{}", data);
}
