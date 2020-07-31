use std::io;

fn main() {
    // test_f_to_c();
    // test_c_to_f();

    // test_fibonacci();

    twelve_days_of_christmas()
}

#[allow(dead_code)]
fn test_f_to_c() {
    println!("\nPlease input degrees Fahrenheit.");
    let mut f = String::new();
    io::stdin().read_line(&mut f).expect("Failed to read line");
    let f: i32 = f.trim().parse().expect("Failed to parse integer");
    let c = fahrenheit_to_celsius(f);
    println!("That equals {} degrees Celsius", c);
}

#[allow(dead_code)]
fn test_c_to_f() {
    println!("\nPlease input degrees Celsius.");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read line");
    let c: i32 = c.trim().parse().expect("Failed to parse integer");
    let f = celsius_to_fahrenheit(c);
    println!("That equals {} degrees Fahrenheit", f);
}

#[allow(dead_code)]
fn fahrenheit_to_celsius(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

#[allow(dead_code)]
fn celsius_to_fahrenheit(c: i32) -> i32 {
    c * 9 / 5 + 32
}

#[allow(dead_code)]
fn test_fibonacci() {
    println!("1st Fibonacci number: {}", fibonacci(0));
    println!("2nd Fibonacci number: {}", fibonacci(1));
    println!("3rd Fibonacci number: {}", fibonacci(2));
    println!("10th Fibonacci number: {}", fibonacci(9));
}

#[allow(dead_code)]
fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

#[allow(dead_code)]
fn twelve_days_of_christmas() {
    let count_words = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let items = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three Frech hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..(items.len() + 1) {
        println!(
            "\nOn the {} day of Christmas my true love sent to me",
            count_words[i - 1]
        );
        for j in (0..i).rev() {
            let and = if i > 1 && j == 0 { "And " } else { "" };
            let delim = if j == 0 { '.' } else { ',' };
            println!("{}{}{}", and, items[j], delim);
        }
    }
}
