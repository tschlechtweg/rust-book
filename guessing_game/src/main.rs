use std::io;

// Trait
// provides following variants
// Less, Greater, Equal
use std::cmp::Ordering;

// Trait
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng(): specific random number generator
    // seeded by the os and local to the thread of execution
    // gen_range(): imported with the Trait: use rand::Rng
    // 1..101: start..end is a range expression (exclusive 1-100)
    // for inclusive use: 1..=100
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    loop {
        println!("Please input your guess.");

        // create variable
        // immutable by default
        // mut: makes variable mutable
        // String::new(): create new instance of String
        // a UTF-8 encoded text
        let mut guess = String::new();

        // without use, would be std::io::stdin
        // returns instance of std::io:Stdin
        io::stdin()
            // calls read_line method
            // appends whatever user types to parameter string
            // &: is a reference argument, no copy
            // mut: context specific: with & means: mutable reference
            // returns a io::Result
            // io::Result is enumeration (have fixed set of variants)
            // Result variants: Ok | Err
            // Ok(number of bytes in user input)
            .read_line(&mut guess)
            // crashes program if returned Result variant is Err
            .expect("Failed to read line");

        // create variable called guess
        // shadows previous guess variable
        // trim: eliminates whitespace at beginning and end
        // also eliminates \r\n and \n
        // parse: parses string to number
        // ...: u32: annotate the variables type
        let guess: u32 = match guess.trim().parse() {
            // no expect, rather match expression
            Ok(num) => num,
            // _ (underscore): is catchall value
            Err(_) => {
                println!("Please only type numbers!");
                continue;
            },
        };
        
        // {}: placeholders for values
        println!("You guessed: {}", guess);

        // .cmp: compares two values
        // returns variant of Ordering
        // match: matches the Ordering enum
        // match expression is made of arms
        // arm has a pattern to match against
        // and the code it should run, if it matches
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }
}
