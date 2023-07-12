fn main() {
    // endless loop (here its capped by 10)
    // loop {
    //     println!("again!");
    // }

    // labeled loops
    let mut count = 0;

    // you can label the loop
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // then use the label with break, continue
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // loops can give back values
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // return values need to be after a break
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // conditional loops
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for loops
    // while loop imitating for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // real for
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // rev: reverse range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
