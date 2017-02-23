fn main() {
    // for sanity's sake, this function is not called
    // infinite_loop();
}

// this function can be escaped using:
// ctrl-c
// or by inserting a "break" inside the loop
fn infinite_loop() {
    loop {
        println!("again!");
    }
}
