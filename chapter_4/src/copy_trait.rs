fn main() {

    let x = 5;
    makes_copy(x);

    let s = String::from("Hello, world!");
    takes_ownership(s);

    println!("{}", x);

}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}