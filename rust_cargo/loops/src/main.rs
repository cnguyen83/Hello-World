fn main() {
    // for sanity's sake, this function is not called
    // infinite_loop();

    liftoff();
    liftoff_for();

    array_while();
    array_for();
}

fn array_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

// Is there no array.length() function in Rust?
// Why is the index length hard-coded in the while loop in this example?
fn array_while() {
    let a = [11, 21, 31, 41, 51];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

// this function can be escaped using:
// ctrl-c
// or by inserting a "break" inside the loop
fn infinite_loop() {
    loop {
        println!("again!");
    }
}

// Function for while loop
fn liftoff() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!");
}

fn liftoff_for() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
