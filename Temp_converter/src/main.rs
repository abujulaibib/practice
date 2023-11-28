use std::io;

fn celcius_fahrenheit(x: f64) -> f64 {
    (x * 1.8) + 32.0
}

fn fahrenheit_celcius(x: f64) -> f64 {
    (x - 32.0) / 1.8
}

fn main() {

    println!("Temperature scale converter!");

    println!("What do you want to convert?");
    
    println!("Enter \"A\" to convert from Celcius to Fahrenhiet or \"B\" from Fahrenhiet to Celcius. ");

    let mut z = String::new();
    io::stdin().read_line(&mut z).expect("Failed to read line");

    let z = z.trim().chars().next();

    if z == Some('A') {

        loop {

            println!("Please enter the number: ");

            let mut y = String::new();

            io::stdin().read_line(&mut y)
                .expect("Failed to read line");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Thats not a number, Try again!");
                    continue
                }
            };

            println!("The result in Fahrenhiet is: {}", celcius_fahrenheit(y));
            break;

        };
    } else if z == Some('B') {

        loop {

            println!("Please enter the number: ");

            let mut y = String::new();

            io::stdin().read_line(&mut y)
                .expect("Failed to read line");

            let y: f64 = match y.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Thats not a number, Try again!");
                    continue
                }
            };

            println!("The result in Celcius is: {}", fahrenheit_celcius(y));
            break; 
        };
    }
}