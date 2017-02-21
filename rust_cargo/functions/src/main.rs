fn main() {
    println!("Hello, world!");

    another_function();

    function_with_params(5, 6);

    statements_exprs();

    let a = return_value();
    println!("The value of a is: {}", a);

    let b = return_expr(7);
    println!("The value of b is: {}", b);
}

fn another_function() {
    println!("Another function.");
}

fn function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_expr(x: i32) -> i32 {
    x + 1
}

fn return_value() -> i32 {
    4
}

fn statements_exprs() {
    let x = 5;    // This is a statement

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
