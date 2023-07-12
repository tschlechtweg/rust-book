fn main() {
    println!("Hello, world!");

    // argument 5
    another_function(5);

    print_labeled_measurement(5, 'C');
}

// name in snake case
// parameter x with type i32
// type needs to be declared
fn another_function(x: i32) {
    println!("Got the parameter x with argument: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// return values
fn five() -> i32 {

    // return value is synonymous with the final expression of 
    // the block of the body in a function
    5
}

fn return_fn() {
    return;
}