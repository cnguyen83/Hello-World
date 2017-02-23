fn main() {
    println!("Hello, world!");

    simple_if();

    nonzero();

    if_else();

    if_let();
}

fn if_else() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn if_let() {
    let condition = true;
    let number = if condition {
        6
    } else {
        5
    };

    println!("the value of number is: {}", number);
}

fn nonzero() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

fn simple_if() {
    let number = 3;
    let large_number = 7;

    // true condition
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // false condition
    if large_number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
