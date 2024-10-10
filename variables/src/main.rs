fn main() {
    let mut x = 5;

    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    change_variable();
    shadowing();
    characters();
}

fn change_variable() {
    println!("-- Change variable");
    let y = 5;

    let y = y * 2;

    println!("The value of y is: {y}");
}

fn shadowing() {
    println!("-- Shadowing");
    let n = 5;
    let n = n + 1;

    {
        let n = n * 2;
        println!("The value of n is: {n}");
    }

    println!("The value of n is: {n}");
}

fn characters() {
    println!("-- Characters");
    let heart_eyed_cat = '😻';

    println!("{heart_eyed_cat}");
}
