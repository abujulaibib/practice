
fn main() {
    
    let mut s1 = String::from("Hello");

    change(&mut s1);

    println!("The string is now this {}", s1);
}
// tip: make the reference mutable to solve this
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//    let len = calculate_length(&s1);

//    println!("The length of '{}' is {}", s1, len);
//}
//fn calculate_length(s: &String) -> usize {

//    s.len()
//}

// ************************************************************************************************************

//    let (s2, len) = calculate_length(s1);

    // In other to use s1 again in the print method we must create a tuple to use it
    // to solve that we ca use the reference to s1, hence not directly moving ownership
//    println!("The length of {} is {}", s2, len);
//}
//fn calculate_length(s: String) -> (String, usize) {

//    let length = s.len();
    
//    (s, length)

//}