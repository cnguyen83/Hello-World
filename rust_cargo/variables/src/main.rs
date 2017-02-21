// File to work with mutable v immutables in Rust

fn main() {
   mutate();
   shadow();
}

fn mutate() {
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);
}

fn shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
