use regex::Regex;
use std::io;

fn main() {
    println!("Convert fahrenheit to celcius, or celcius to fahrenheit");
    println!("Enter the temperature you want to convert, followed by F for fahrenheit or C for celsius (ex: 84F): ");
    let mut temperature = String::new();

    loop {
        temperature = String::new();
        // read input
        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read input");

        temperature = temperature.trim().to_owned();

        // validate input. must be <number>F, <number>f, <number>C, or <number>c
        let re = Regex::new(r"^-?[0-9]+[fFcC]$").unwrap();
        if re.is_match(temperature.as_str()) {
            break;
        } else {
            println!("Invalid input, please try again");
        }
    }

    let unit = temperature.pop();
    let value: f64 = temperature.trim().parse().unwrap();

    if unit == Some('f') || unit == Some('F') {
        println!("{}C", f_to_c(value));
    } else {
        println!("{}F", c_to_f(value));
    }
}

fn f_to_c(x: f64) -> f64 {
    (5.0 / 9.0) * (x - 32.0)
}

fn c_to_f(x: f64) -> f64 {
    x * (9.0 / 5.0) + 32.0
}
