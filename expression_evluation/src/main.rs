// write a simple recursive evaluator for arithmetic expressions
// use todo!()
// the standard Result<Value, String> type

// create options
#[derive(Debug)]
enum Value { 
    Number(i32),
    Add(Box<Value>, Box<Value>),
    Sub(Box<Value>, Box<Value>),
}

fn eval(expr: Value) -> Result<i32, String> {
    match expr {
        Value::Number(n) => Ok(n),
        Value::Add(a, b) => {
            let a = eval(*a)?;
            let b = eval(*b)?;
            Ok(a + b)
        }
        Value::Sub(a, b) => {
            let a = eval(*a)?;
            let b = eval(*b)?;
            Ok(a - b)
        }
    }
}

fn main() {
    let num1 = 5;
    let num2 = 6;

    let _add = Value::Add(Box::new(Value::Number(num1)), Box::new(Value::Number(num2)));
    let _sub = Value::Sub(Box::new(Value::Number(num1)), Box::new(Value::Number(num2)));

    println!("add: {:?}", eval(_add));


}
