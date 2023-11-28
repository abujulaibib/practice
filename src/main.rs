use std::io;

fn main() {
    
    let a = [1, 2, 3, 4, 5];

    loop {

        println!("Please enter an array index. ");

        let mut index = String::new();
        io::stdin().read_line(&mut index).expect("Failed to read line");
        let index: usize = match index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if index > 5 {
            println!("The number must be between 0 and 5.");
        } else {

        let element = a[index];
        println!("The value of the element at index {} is: {}", index, element);
        break;
        }
    }
}