fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const: constant is always immutable
    // data type MUST be annotated
    // compiler evaluates limited set of operations at compile time
    // such that it is easier to write the expression rather than
    // 10.800
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("Three hours have: {} seconds.", THREE_HOURS_IN_SECONDS);

    println!("Shadowing...");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    println!("The value of y is: {}", y);

    println!("Shadowing is cool...");

    // can shadow with different data type
    let spaces = "    ";
    
    print!("This space|{}|", spaces);
    
    let spaces = spaces.len();

    println!(" is {} spaces wide.", spaces);

    // let mut space = "    ";
    // spaces = spaces.len();
    // cant mutate variables type

    // infer data type
    let everything: u32 = "42".parse().expect("Not a number!");

    // scalar types
    // represent single values
    // four primary types: intergers, floating-point, bool, chars

    // integer
    let a1 = 98_222; // 98222
    let a2 = 60u8; // data type: unsigned 8 bit integer
    let a3 = 0xBB; // hex
    let a4 = 0b1111_0000; // binary
    let a5 = b'A'; // Byte (u8 only)
    let a6 = 0o77; // octal

    // floating point
    let b1 = 2.0; // f64 (default)
    let b2: f32 = 3.0; // f32

    // bool
    let c1 = true;
    let c2: bool = false;

    // character 
    // 4 bytes in size
    // Unicode Scalar Value
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("heart eyed cat emoji: {}", heart_eyed_cat);

    // compound types
    // multiple values grouped into one
    // primary types: tuples, arrays

    // tuple
    // different values with variety of types
    // fixed size
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching a tuple
    // called destructure
    let (_, y, _) = tup;

    // unit type
    // tuple with special type () that has one value ()
    let unit_value: () = ();

    let z = tup.2;

    // array
    // fixed length
    // same type
    // data allocated on the stack, duh
    let d1 = [1, 2, 3, 4, 5];
    let d2: [i32, 5] = [1, 2, 3, 4, 5];
    
    // equal to [10, 10, 10, 10, 10]
    let d3: [10; 5];
    let first = d3[0];
}

