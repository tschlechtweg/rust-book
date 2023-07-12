enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match against enum variants
    // matches need to be exhaustive !!!
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch all
        // put it last, patterns are evaluated in order
        other => move_player(other),
    }

    // if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);

    // pattern = expression
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    println!("Value of a Penny: {} Cents", value_in_cents(Coin::Penny));

}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}