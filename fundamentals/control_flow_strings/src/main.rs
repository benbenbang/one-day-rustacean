#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    // This collects any command-line arguments into a vector of Strings.
    // For example:
    //
    //     cargo run apple banana
    //
    // ...produces the equivalent of
    //
    //     vec!["apple".to_string(), "banana".to_string()]
    let args: Vec<String> = std::env::args().skip(1).collect();

    // This consumes the `args` vector to iterate through each String
    for arg in args {
        if arg == "sum" {
            sum()
        } else if arg == "double" {
            double()
        } else {
            count(arg)
        }
    }
}

fn sum() {
    let mut sum = 0;
    // Use a "for loop" to iterate through integers from 7 to 23 *inclusive* using a range
    // and add them all together (increment the `sum` variable).  Hint: You should get 255

    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    // Use a "while loop" to count how many times you can double the value of `x` (multiply `x`
    // by 2) until `x` is larger than 500.  Increment `count` each time through the loop. Run it
    // with `cargo run double`  Hint: The answer is 9 times.

    while x <= 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x={} is larger than 500",
        count, x
    );
}

fn count(arg: String) {
    // print!("{} ", arg); // Execute this line 8 times, and then break. `print!` doesn't add a newline.

    let mut count = 0;

    loop {
        if count > 7 {
            break;
        }
        print!("{} ", arg);
        count += 1;
    }

    println!();
}
