fn main() {
    scope_string_literal();

    mutated_String_type();

    copy_v_ownership();

    ownership_between_functions();

    passing_ownership_tuples();
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();	// len() is a built-in method for String type

    (s, length)
}

fn copy_v_ownership() {
    let s = String::from("hello");
    takes_ownership(s);
    // The line below will cause an error because "s" no longer exists in this scope
    // println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{} still exists after function call!", x);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{} exists in the scope of makes_copy!", some_integer);
}

fn mutated_String_type() {
    let mut s = String::from("Hello");	// s is now in scope
    s.push_str(", world!");		// s is still in scope and its value changed
    println!("{}", s);			// s is still in scope
}					// s is no longer in scope

fn ownership_between_functions() {
    let s1 = gives_ownership();		// function will move its return value into s1
    println!("s1 has value {} from the function", s1);

    let s2 = String::from("hello");	// s2 comes into scope
    println!("s2 has value {}", s2);
    let s3 = takes_and_gives_back(s2);	// s2 loses its value to the function
    println!("Now s3 has the value {} that it took from s2", s3);					// the function then moves its return value to s3, so s3 gets s2s value through a reassignment via the function
}					// s3 goes out of scope and is dropped. s2 likewise goes out of scope but its value was moved, so nothing happens. s1 goes out of scope and likewise is dropped.

fn passing_ownership_tuples() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The lenght of '{}' is {}. '{}' used to be s1.", s2, len, s2);
}

fn scope_string_literal() {
    let s = "hello";	// s is now in scope
			// s is still in scope
    println!("{}", s);
}			// s is no longer in scope

fn takes_and_gives_back(a_string: String) -> String {
    a_string			// simply returns the arg to the variable that called the function in order to get a value assignment.
}

fn takes_ownership(some_string: String) {
    println!("{} was copied into the scope of takes_ownership", some_string);
}
