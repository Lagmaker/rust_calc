

use std::io;


fn add(a: f64, b: f64) -> f64 {
    a + b
}
fn sub(a: f64, b: f64) -> f64 {
    a - b
}
fn mul(a: f64, b: f64) -> f64 {
    a * b
}
fn div(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Can't divide by zero");
        0.0
    }
    else {
        a / b
    }
}

fn main() {

    enum ACTIONS {
        ADD = 1 ,
        SUB = 2,
        MUL = 3,
        DIV = 4
    }

    let mut input: String = String::new();


    println!("Input first number: ");
    io::stdin().read_line(&mut input).expect("expected to read line");
    let num_1: f64 = input.trim().parse().unwrap();
    input.clear();

    println!("Input second number: ");
    io::stdin().read_line(&mut input).expect("expected to read line");
    let num_2: f64 = input.trim().parse().unwrap();
    input.clear();


    println!("Choose the action: [1=add, 2=sub, 3=mul, 4=div]");
    io::stdin().read_line(&mut input).expect("expected to read line");
    let action: i32 = input.trim().parse().unwrap();


    let mut result: f64 = 0.0;
    if action == ACTIONS::ADD as i32{
        result = add(num_1, num_2);
    }
    else if action == ACTIONS::SUB as i32{
        result = sub(num_1, num_2);
    }
    else if action == ACTIONS::MUL as i32{
        result = mul(num_1, num_2);
    }
    else if action == ACTIONS::DIV as i32{
        result = div(num_1, num_2);
    }
    else {
        println!("Invalid action, no action for: {}", action);
    }

    println!("Result: {}", result);

}
