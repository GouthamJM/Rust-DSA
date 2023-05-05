use crate::simple::fibonacci;
pub mod simple;

fn main() {
    let result = fibonacci(6);
    println!("The fibonacci value of {result}");
}
