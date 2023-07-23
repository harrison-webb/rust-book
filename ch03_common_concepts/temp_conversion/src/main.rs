use std::io;

fn main() {
    loop {
        println!("Enter temperature to convert (e.g. 86F or 32C) or 'quit' to exit: ");
        let mut input = String::new();

        // read input
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        let input = input.trim().to_lowercase();

        if input == "quit" {
            break;
        }

        let (temp, unit) = input.split_at(input.len() - 1);

        let temp: f64 = match temp.parse() {
            Ok(val) => val,
            Err(_) => {
                println!(
                    "Invalid input: '{}'. Please enter a number followed by 'F' or 'C'",
                    temp
                );
                continue;
            }
        };

        match unit {
            "f" => {
                println!("{}C", f_to_c(temp));
                break;
            }
            "c" => {
                println!("{}F", c_to_f(temp));
                break;
            }
            _ => {
                println!("Invalid unit: '{}'. Please use 'F' or 'C'", unit);
            }
        }
    }
}

fn f_to_c(x: f64) -> f64 {
    (5.0 / 9.0) * (x - 32.0)
}

fn c_to_f(x: f64) -> f64 {
    x * (9.0 / 5.0) + 32.0
}
