//used in place of loop, if, else, break expression

fn main() {
    let mut number = 3;

    while number !=0 {

        // use print! macro to output them all on the same line!
        println!("{}", number);

        number = number - 1
    }

    println!("LISTOFF!!!");
}