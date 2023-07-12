fn main() {
    let number = 3;

    // must be bool condition
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // if, else if, else
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

    // if's are expressions
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // types can't mismatch
    // let number = if condition { 5 } else { "didu" };
}
